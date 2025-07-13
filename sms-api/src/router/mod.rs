pub mod routes;

use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{router::routes::send_sms, state::AppState};

pub fn app(state: Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/sms", post(send_sms))
        .with_state(state);

    app
}
