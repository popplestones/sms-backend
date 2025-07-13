use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}
