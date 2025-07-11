use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use redis::AsyncCommands;
use serde_json::{Value, json};
use sms_core::models::sms::SmsMessage;
use tokio::net::TcpListener;

fn init_env() {
    dotenvy::dotenv().ok();
}

#[derive(Clone)]
struct AppState {
    redis_client: redis::Client,
}

#[tokio::main]
async fn main() {
    init_env();
    let redis_url = std::env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(redis_url).unwrap();
    let state = AppState {
        redis_client: client,
    };

    let app = Router::new()
        .route("/sms", post(send_sms))
        .with_state(Arc::new(state));

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn send_sms(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SmsMessage>,
) -> (StatusCode, Json<Value>) {
    let mut con = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let payload = serde_json::to_string(&msg).unwrap();
    let _: () = con.rpush("sms_queue", payload).await.unwrap();

    (
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "queued",
        })),
    )
}
