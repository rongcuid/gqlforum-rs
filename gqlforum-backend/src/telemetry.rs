use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_telemetry() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "gqlforum-backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
