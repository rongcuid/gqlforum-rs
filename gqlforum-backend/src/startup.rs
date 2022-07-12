use axum::{handler::Handler, routing::get, Router};
use std::net::SocketAddr;

use crate::routes::{fallback::handler_404, index::handler};

pub async fn run() -> Result<(), hyper::Error> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .fallback(handler_404.into_service());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}
