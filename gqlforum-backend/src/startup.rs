use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{handler::Handler, routing::get, Extension, Router};
use sqlx::SqlitePool;
use std::net::SocketAddr;

use crate::{
    model::QueryRoot,
    routes::{
        fallback::handler_404,
        graphql::{graphql_handler, graphql_playground},
        index::index_handler,
    },
};

pub async fn run() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("SQLite connection error");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.expect("Migration error");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    // build our application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .fallback(handler_404.into_service())
        .layer(Extension(pool))
        .layer(Extension(schema));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Axum server error");
}
