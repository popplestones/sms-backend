use std::{sync::Arc, time::Duration};

use redis::AsyncCommands;
use sms_core::AppResult;
use tokio::time::interval;
use tracing::{error, info};

pub struct DelayedProcessor {
    client: Arc<redis::Client>,
    check_interval: Duration,
}

impl DelayedProcessor {
    pub fn new(client: Arc<redis::Client>) -> Self {
        let check_interval = std::env::var("DELAYED_WORKER_INTERVAL")
            .ok()
            .and_then(|s| s.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(10)); // Default to 10 seconds

        Self {
            client,
            check_interval,
        }
    }

    pub async fn run(&self) -> AppResult<()> {
        info!(
            "Starting delayed message processor with interval: {:?}",
            self.check_interval
        );

        let mut interval = interval(self.check_interval);

        loop {
            interval.tick().await;

            if let Err(e) = self.process_ready_messages().await {
                error!(?e, "Failed to process delayed messages");
                // Continue running even if there's an error
            }
        }
    }

    async fn process_ready_messages(&self) -> AppResult<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let current_time = chrono::Utc::now().timestamp() as f64;

        let ready_message: Vec<String> = con
            .zrangebyscore_limit("sms_delayed_queue", 0.0, current_time, 0, 100)
            .await?;

        info!(
            "Found {} delayed messages ready for processing",
            ready_message.len()
        );

        for msg in ready_message {
            if let Err(e) = self.move_to_main_queue(&mut con, &message).await {
                error!("Failed to move delayed message: {e:?}");
            }
        }

        Ok(())
    }

    async fn move_to_main_queue(&self, con: &mut redis::aio::MultiplexedConnection, message: &str) -> AppResult<()> {
        let mut pipe = redis::pipe();
        pipe.zrem("sms_delayed_queue", message)
        .rpush("sms_queue", message);
}

async fn process_delayed_messages(&self) -> AppResult<()> {
    let mut con = self
        .state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let current_time = chrono::Utc::now().timestamp() as f64;

    // Get all messages that are ready to be processed (score <= current_time)
    let ready_messages: Vec<String> = con
        .zrangebyscore_limit("sms_delayed_queue", 0.0, current_time, 0, 100)
        .await?;

    if ready_messages.is_empty() {
        return Ok(());
    }
}
