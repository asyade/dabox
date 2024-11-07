use std::sync::Arc;

use dabox_api::app::{create_app, AppExt};
use dabox_core::da_repository::MemRepository;
use tracing::info;
use tracing_subscriber::{prelude::*, EnvFilter};

const DEFAULT_LISTEN_ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let listen_addres =
        std::env::var("LISTEN_ADDRESS").unwrap_or(DEFAULT_LISTEN_ADDRESS.to_string());

    info!(listen_address = listen_addres, "Starting Dabox API");
    create_app(Arc::new(MemRepository::new()))
        .enable_cors()
        .serve(listen_addres)
        .await
        .unwrap();
}
