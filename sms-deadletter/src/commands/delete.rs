use crate::redis::delete_dead_letter_job;

pub async fn run(con: &mut redis::aio::MultiplexedConnection, key: &str) -> anyhow::Result<()> {
    match delete_dead_letter_job(con, key).await? {
        Some(job) => {
            println!("🗑️ Deleted job {}", job.id);
        }
        None => {
            println!("❌ Job not found: {key}");
        }
    }
    Ok(())
}
