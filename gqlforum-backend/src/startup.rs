use async_graphql::{EmptySubscription, Schema};
use axum::{handler::Handler, routing::get, Extension, Router};
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::{log::LevelFilter, *};

use crate::{
    configuration::get_configuration,
    graphql::{MutationRoot, QueryRoot},
    routes::{
        fallback::handler_404,
        graphql::{graphql_handler, graphql_playground},
    },
};

use crate::telemetry::{init_telemetry, setup_telemetry};

pub async fn run() {
    init_telemetry();

    let configuration = get_configuration().expect("Failed to read configuration");

    let mut options = SqliteConnectOptions::from_str(&configuration.database.connection)
        .expect("Failed to create SqlitePoolOptions")
        .create_if_missing(true);
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
        .extension(async_graphql::extensions::ApolloTracing)
        .data(pool.clone())
        .finish();

    // build our application with a route
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .fallback(handler_404.into_service())
        .layer(Extension(pool))
        .layer(Extension(schema));

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404.into_service());

    let app = setup_telemetry(app);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
