use std::sync::Arc;

use sms_core::models::job::OutgoingJob;

use crate::config::RetryConfig;

#[derive(Clone)]
pub struct JobProcessor {
    pub redis_client: Arc<redis::Client>,
    pub config: RetryConfig,
}

impl JobProcessor {
    pub fn new(redis_client: Arc<redis::Client>, config: RetryConfig) -> Self {
        Self {
            redis_client,
            config,
        }
    }

    pub async fn process(&self, _job: OutgoingJob) -> anyhow::Result<()> {
        Ok(())
    }
}
