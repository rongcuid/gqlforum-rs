use axum::{
    handler::Handler,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::{index::handler, fallback::handler_404};

pub async fn run() -> Result<(), hyper::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_global_404_handler=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404.into_service());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service()).await
}
