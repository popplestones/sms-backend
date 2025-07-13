use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub struct RequestRejection(pub StatusCode, pub String);

impl IntoResponse for RequestRejection {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({ "error": self.1 }));
        (self.0, body).into_response()
    }
}
