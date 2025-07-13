use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::models::sms::SmsMessage;

#[derive(Debug, Deserialize)]
pub struct SmsSubmission {
    #[serde(flatten)]
    pub message: SmsMessage,

    /// Optional ISO8601 timestamp to delay the job (e.g. "2025-07-12T10:00:00Z")
    pub schedule_at: Option<DateTime<Utc>>,
}
