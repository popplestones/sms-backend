use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::sms::SmsMessage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutgoingJob {
    pub id: Uuid,
    pub message: SmsMessage,

    // Dead-letter metadata (optional for initial jobs)
    pub attempts: u32,
    pub error: Option<String>,
    pub last_failed_at: Option<DateTime<Utc>>,
    pub scheduled_at: Option<DateTime<Utc>>,
}
