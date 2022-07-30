use async_graphql::{EmptySubscription, Schema};
use axum::{handler::Handler, routing::get, Extension, Router};

use hyper::Method;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use std::str::FromStr;
use tracing::log::LevelFilter;

use crate::{
    configuration::get_configuration,
    graphql::{MutationRoot, QueryRoot},
    routes::{
        fallback::handler_404,
        graphql::{graphql_handler, graphql_playground},
    },
};

use crate::telemetry::{init_telemetry, setup_telemetry};

#[derive(Clone)]
pub struct HmacSecret(pub String);

#[derive(Clone)]
pub struct SessionCookieName(pub String);

pub async fn run() {
    init_telemetry();

    let configuration = get_configuration().expect("Failed to read configuration");

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
        // .extension(async_graphql::extensions::ApolloTracing)
        .data(HmacSecret(configuration.hmac_secret.clone()))
        .data(SessionCookieName(configuration.session_cookie_name.clone()))
        .data(pool.clone())
        .finish();

    // build our application with a route
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .fallback(handler_404.into_service())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // TODO: make this safe
                .layer(Extension(pool))
                .layer(Extension(schema))
                .layer(Extension(SessionCookieName(
                    configuration.session_cookie_name.clone(),
                )))
                .layer(Extension(HmacSecret(configuration.hmac_secret.clone()))),
        );

    let app = setup_telemetry(app);

    // run it
    let addr = format!("{}:{}", configuration.listen, configuration.port)
        .parse()
        .unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
