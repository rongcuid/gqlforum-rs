use async_graphql::{EmptySubscription, Schema};
use axum::routing::get_service;
use axum::{routing::get, Extension, Router};

use hyper::StatusCode;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

use std::str::FromStr;

use tower::builder::ServiceBuilder;

use tracing::log::LevelFilter;

use super::graphql::{MutationRoot, QueryRoot};
use super::routes::{
    fallback::handler_404,
    graphql::{graphql_handler, graphql_playground},
};
use crate::configuration::get_configuration;

use crate::telemetry::{init_telemetry, setup_telemetry};

#[derive(Clone)]
pub struct HmacSecret(pub String);

#[derive(Clone)]
pub struct SessionCookieName(pub String);

pub async fn run() {
    init_telemetry();
    let configuration = get_configuration().expect("Failed to read configuration");
    let addr = format!("{}:{}", configuration.listen, configuration.port)
        .parse()
        .unwrap();

    let mut options = SqliteConnectOptions::from_str(&configuration.database.connection)
        .expect("Failed to create SqlitePoolOptions")
        .create_if_missing(true)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
        .pragma("temp_store", "MEMORY");
    options.log_statements(LevelFilter::Trace);
    let pool = SqlitePool::connect_with(options)
        .await
        .expect("SQLite connection error");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration error");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(async_graphql::extensions::Tracing)
        .limit_complexity(1024)
        // .extension(async_graphql::extensions::ApolloTracing)
        .data(HmacSecret(configuration.hmac_secret.clone()))
        .data(SessionCookieName(configuration.session_cookie_name.clone()))
        .data(pool.clone())
        .finish();

    let index = configuration.dist.clone() + "/index.html";
    let spa_service = get_service(
        ServeFile::new(index)
            .precompressed_gzip()
            .precompressed_br()
            .precompressed_deflate(),
    )
    .handle_error(|_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error") });
    // build our application with a route
    let app = Router::new()
        .route("/test", spa_service.clone())
        .route("/login", spa_service.clone())
        .route("/logout", spa_service.clone())
        .route("/topic/:id/:page", spa_service.clone())
        .route("/user/:id", spa_service.clone())
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .fallback(
            get_service(ServeDir::new(configuration.dist))
                .handle_error(|_| async move { handler_404().await }),
        );

    let app = app.layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new().gzip(true).deflate(true).br(true))
            // .layer(CorsLayer::permissive())
            .layer(Extension(pool))
            .layer(Extension(schema))
            .layer(Extension(SessionCookieName(
                configuration.session_cookie_name.clone(),
            )))
            .layer(Extension(HmacSecret(configuration.hmac_secret.clone()))),
    );

    let app = setup_telemetry(app);

    // run it

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
