use axum::{body::Body, http::Request, Router};
use nanoid_dictionary::NOLOOKALIKES_SAFE;
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
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros),
            )
            .make_span_with(|request: &Request<Body>| {
                info_span!(
                    "request",
                    id = %nanoid::nanoid!(21, NOLOOKALIKES_SAFE),
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
    )
}
