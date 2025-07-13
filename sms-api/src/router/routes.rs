use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use sms_core::models::SmsSubmission;

use crate::state::AppState;

pub async fn send_sms(
    State(_state): State<Arc<AppState>>,
    Json(_submission): Json<SmsSubmission>,
) -> impl IntoResponse {
    // Step 1. Authentication - handle in middleware
    // Step 2. Authorization - handle in FormRequest
    // Step 3. Validation - handle in FormRequest
    // Step 4. Act (Queue either delayed or immediate)
    // Step 5. Response
}

// pub async fn send_sms(
//     State(state): State<Arc<AppState>>,
//     Json(submission): Json<SmsSubmission>,
// ) -> impl IntoResponse {
//     {
//         let job = OutgoingJob {
//             id: Uuid::new_v4(),
//             message: submission.message,
//             attempts: 0,
//             error: None,
//             last_failed_at: None,
//             scheduled_at: submission.schedule_at,
//         };
//
//         let payload = match serde_json::to_string(&job) {
//             Ok(p) => p,
//             Err(e) => {
//                 error!(?e, "Failed to serialize job");
//                 return (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({ "error": "Serialization failed" })),
//                 );
//             }
//         };
//
//         let mut con = match state.redis_client.get_multiplexed_async_connection().await {
//             Ok(c) => c,
//             Err(e) => {
//                 error!(?e, "Failed to enqueue job");
//                 return (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({ "error": "Failed to enqueue job" })),
//                 );
//             }
//         };
//
//         // Check if this is a delayed message
//         if let Some(scheduled_at) = submission.schedule_at {
//             let score = scheduled_at.timestamp() as f64;
//             let add_result: Result<(), RedisError> =
//                 con.zadd("sms_delayed_queue", payload, score).await;
//
//             if let Err(e) = add_result {
//                 error!(?e, "Failed to enqueue delayed job");
//                 return (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({ "error": "Failed to enqueue delayed job" })),
//                 );
//             }
//
//             info!(job_id = %job.id, scheduled_at = %scheduled_at, "Queued delayed SMS job");
//             return (
//                 StatusCode::ACCEPTED,
//                 Json(json!({
//                     "status": "scheduled",
//                     "job_id": job.id,
//                     "scheduled_at": scheduled_at,
//                 })),
//             );
//         }
//
//         match is_queue_overloaded(&mut con).await {
//             Ok(true) => {
//                 return (
//                     StatusCode::TOO_MANY_REQUESTS,
//                     Json(json!({ "error": "Queue is full" })),
//                 );
//             }
//             Err(e) => {
//                 error!(?e, "Failed to check queue length");
//                 return (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({ "error": "Failed to check queue length" })),
//                 );
//             }
//             _ => {}
//         };
//
//         let push_result: Result<(), RedisError> = con.rpush("sms_queue", payload).await;
//         if let Err(e) = push_result {
//             error!(?e, "Failed to enqueue job");
//             return (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({ "error": "Failed to enqueue job" })),
//             );
//         };
//
//         info!(job_id = %job.id, "Queued SMS job");
//         (
//             StatusCode::ACCEPTED,
//             Json(json!({
//                 "status": "queued",
//                 "job_id": job.id,
//             })),
//         )
//     }
// }
