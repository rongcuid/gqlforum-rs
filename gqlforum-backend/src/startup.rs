use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{handler::Handler, routing::get, Extension, Router};
use sqlx::SqlitePool;
use std::net::SocketAddr;

use crate::{
    configuration::get_configuration,
    model::QueryRoot,
    routes::{
        fallback::handler_404,
        graphql::{graphql_handler, graphql_playground},
    },
};

use crate::telemetry::{init_telemetry, setup_telemetry};

pub async fn run() {
    init_telemetry();

    let configuration = get_configuration().expect("Failed to read configuration");

    let pool = SqlitePool::connect(&configuration.database.connection)
        .await
        .expect("SQLite connection error");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration error");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
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
