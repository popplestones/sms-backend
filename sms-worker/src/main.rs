use std::{sync::Arc, time::Duration};

use anyhow::Context;
use chrono::Utc;
use redis::AsyncCommands;
use sms_core::models::{job::OutgoingJob, sms::SmsMessage};
use sms_worker::config::{RetryConfig, WorkerConfig};
use tokio::time::sleep;

use sms_core::{AppError, AppResult};
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

fn init_env() {
    dotenvy::dotenv().ok();
}

#[tokio::main]
async fn main() -> AppResult<()> {
    init_env();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = WorkerConfig::load_from_file("config/worker.toml")
        .context("Failed to load worker config from config/worker.toml")?;
    let retry = Arc::new(config.retry);

    let client =
        Arc::new(redis::Client::open("redis://127.0.0.1/").context("Failed to connect to redis")?);

    info!("Worker listening on queue: sms_queue");

    loop {
        let mut con = client
            .get_multiplexed_async_connection()
            .await
            .context("Failed to get Redis connection")?;

        let (_key, payload): (String, String) = con.blpop("sms_queue", 0.0).await?;

        match serde_json::from_str::<OutgoingJob>(&payload) {
            Ok(job) => {
                let config = retry.clone();
                let client = client.clone();
                tokio::spawn(async move {
                    match client.get_multiplexed_async_connection().await {
                        Ok(mut con) => {
                            if let Err(e) = process_with_retries(job, config, &mut con).await {
                                error!("Job failed after retries: {e:?}");
                            }
                        }
                        Err(e) => error!("Could not connect to Redis for retry handler: {e:?}"),
                    }
                });
            }
            Err(e) => {
                error!("Error parsing message: {e:?}");
            }
        }
    }
}

async fn process_with_retries(
    mut job: OutgoingJob,
    config: Arc<RetryConfig>,
    con: &mut redis::aio::MultiplexedConnection,
) -> AppResult<()> {
    for attempt in 1..=config.max_attempts {
        match try_send_sms(&job.message).await {
            Ok(_) => {
                info!(
                    "✅ Sent SMS (job {}) on attempt {attempt}: {:?}",
                    job.id, job.message
                );
                return Ok(());
            }
            Err(e) => {
                warn!("⚠️Job {} failed on attempt {attempt} failed: {}", job.id, e);
                if attempt == config.max_attempts {
                    error!("❌ Max attempts reached for Job {}", job.id);
                    job.attempts = attempt;
                    job.error = Some(e.to_string());
                    job.last_failed_at = Some(Utc::now());

                    error!(
                        "🪦 Job {} moved to DEAD-LETTER queue after {} attempts",
                        job.id, attempt
                    );
                    let serialized = serde_json::to_string(&job)?;
                    let _: () = con.rpush("sms_dead_letter", serialized).await?;
                    return Err(e);
                }
                let delay = config.delay_for_attempt(attempt as usize);
                warn!("⌛Job {} will retry in {delay}ms", job.id);
                sleep(Duration::from_millis(delay)).await;
            }
        }
    }

    Err(AppError::Other(anyhow::anyhow!(
        "Retry logic exhausted unexpectedly"
    )))
}

async fn try_send_sms(_msg: &SmsMessage) -> AppResult<()> {
    if rand::random::<f32>() < 0.8 {
        Err(AppError::SendFailed("Simulated failure".into()))
    } else {
        Ok(())
    }
}
