use std::sync::Arc;

use anyhow::Context;
use axum::{Router, routing::post};
use sms_api::{handler::send_sms, state::AppState};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn init_env() {
    dotenvy::dotenv().ok();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_env();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize Application State
    let state = AppState::new().context("Cannot initialize application state")?;

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(&addr)
        .await
        .context("Cannot listen on address, is it in use?")?;

    info!("SMS API Listening on {addr}");
    axum::serve(listener, app)
        .await
        .context("Unable to serve")?;
    Ok(())
}
