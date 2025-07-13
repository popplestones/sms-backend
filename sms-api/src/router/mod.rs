pub mod routes;

use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{router::routes::send_sms, state::AppState};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/sms", post(send_sms))
        .with_state(Arc::new(state))
}
