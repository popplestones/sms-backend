use std::sync::Arc;

use anyhow::Context;
use axum::{Router, routing::post};
use sms_api::{handler::send_sms, state::AppState};
use sms_core::AppResult;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn init_env() {
    dotenvy::dotenv().ok();
}

#[tokio::main]
async fn main() -> AppResult<()> {
    init_env();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let redis_url = std::env::var("REDIS_URL").context("Missing env var REDIS_URL")?;
    let client = redis::Client::open(redis_url).context("Cannot connect to redis")?;

    let state = AppState {
        redis_client: client,
    };

    let app = Router::new()
        .route("/sms", post(send_sms))
        .with_state(Arc::new(state));

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
