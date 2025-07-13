use sms_core::models::job::OutgoingJob;

use crate::redis::get_raw_dead_letter_jobs;

pub async fn run(con: &mut redis::aio::MultiplexedConnection) -> anyhow::Result<()> {
    let jobs = get_raw_dead_letter_jobs(con).await?;
    for (i, job) in jobs.iter().enumerate() {
        if let Ok(parsed) = serde_json::from_str::<OutgoingJob>(job) {
            println!("{i}: {} — to: {}", parsed.id, parsed.message.to);
        } else {
            println!("{i}: <unparseable>");
        }
    }
    Ok(())
}
