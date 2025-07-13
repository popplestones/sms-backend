use std::sync::Arc;

use redis::{AsyncCommands, Commands};
use sms_core::models::job::OutgoingJob;
use tracing::{error, info};

use crate::{
    config::WorkerConfig, delayed_processor::DelayedProcessor, job_processor::JobProcessor,
};

pub struct Worker {
    client: Arc<redis::Client>,
    config: WorkerConfig,
}

impl Worker {
    pub fn new(client: Arc<redis::Client>, config: WorkerConfig) -> Self {
        Self { client, config }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Worker listening on queue: sms_queue");
        info!("Worker processing delayed messages from: sms_delayed_queue");

        // Start delayed processor
        let delayed_processor = DelayedProcessor::new(Arc::clone(&self.client));
        tokio::spawn(async move {
            if let Err(e) = delayed_processor.run().await {
                error!("Delayed worker failed: {e:?}");
            }
        });

        // Main worker looop
        let job_processor = JobProcessor::new(Arc::clone(&self.client), self.config.retry.clone());
        loop {
            let mut con = self.client.get_multiplexed_async_connection().await?;
            let (_key, payload): (String, String) = con.blpop("sms_queue", 0.0).await?;

            match serde_json::from_str::<OutgoingJob>(&payload) {
                Ok(job) => {
                    let processor = job_processor.clone();
                    tokio::spawn(async move {
                        if let Err(e) = processor.process(job).await {
                            error!("Job failed after retries: {e:?}");
                        }
                    });
                }
                Err(e) => {
                    error!("Error parsing message: {e:?}");
                }
            }
        }
    }
}
