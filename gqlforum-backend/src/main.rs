use gqlforum_backend::{startup::run, telemetry::setup_telemetry};

#[tokio::main]
async fn main() {
    setup_telemetry();
    run().await;
}
