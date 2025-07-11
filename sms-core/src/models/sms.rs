use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}
