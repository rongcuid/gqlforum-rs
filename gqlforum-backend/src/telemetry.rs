use axum::{body::Body, http::Request, Router};
use tokio::task::JoinHandle;
use tower_http::{
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::*;
use tracing_subscriber::prelude::*;

pub fn init_telemetry() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn setup_telemetry(app: Router) -> Router {
    app.layer(
        TraceLayer::new_for_http()
            .on_request(
                DefaultOnRequest::new(), // .level(Level::INFO)
            )
            .on_response(
                DefaultOnResponse::new()
                    // .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros),
            )
            .make_span_with(|request: &Request<Body>| {
                info_span!(
                    "request",
                    id = %nanoid::nanoid!(),
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
    )
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
