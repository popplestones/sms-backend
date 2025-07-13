use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SmsMessage {
    #[validate(length(min = 1, message = "Recipient is required"))]
    pub to: String,

    #[validate(length(min = 1, message = "Sender is required"))]
    pub from: String,

    #[validate(length(min = 1, message = "Message body is required"))]
    pub body: String,
}
