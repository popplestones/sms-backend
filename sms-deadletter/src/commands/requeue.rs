use sms_core::models::job::OutgoingJob;

use crate::redis::{get_raw_dead_letter_jobs, requeue_all_jobs, requeue_job_by_index};

pub async fn all(con: &mut redis::aio::MultiplexedConnection) -> anyhow::Result<()> {
    let jobs = get_raw_dead_letter_jobs(con).await?;
    let requeued = requeue_all_jobs(con, &jobs).await?;
    println!("✅ Requeued {requeued} job(s)");
    Ok(())
}

pub async fn one(con: &mut redis::aio::MultiplexedConnection, key: &str) -> anyhow::Result<()> {
    let jobs = get_raw_dead_letter_jobs(con).await?;
    for (index, job) in jobs.iter().enumerate() {
        let parsed: OutgoingJob = match serde_json::from_str(job) {
            Ok(j) => j,
            Err(_) => continue,
        };

        if parsed.id.to_string() == key || key == index.to_string() {
            requeue_job_by_index(con, index, job).await?;
            println!("✅ Requeued job {}", parsed.id);
            return Ok(());
        }
    }

    println!("❌ Job not found: {key}");
    Ok(())
}
