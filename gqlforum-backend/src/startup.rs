use axum::{handler::Handler, routing::get, Router, Extension};
use sqlx::{ SqlitePool};
use std::net::SocketAddr;

use crate::routes::{fallback::handler_404, index::handler};

pub async fn run() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("SQLite connection error");
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .fallback(handler_404.into_service())
        .layer(Extension(pool));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Axum server error");
}
