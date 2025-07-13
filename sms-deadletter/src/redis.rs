use redis::AsyncCommands;
use sms_core::models::job::OutgoingJob;

pub async fn delete_dead_letter_job(
    con: &mut redis::aio::MultiplexedConnection,
    key: &str,
) -> anyhow::Result<Option<OutgoingJob>> {
    let jobs = get_raw_dead_letter_jobs(con).await?;
    for (index, job) in jobs.iter().enumerate() {
        let parsed: OutgoingJob = match serde_json::from_str(job) {
            Ok(j) => j,
            Err(_) => continue,
        };

        if parsed.id.to_string() == key || key == index.to_string() {
            let _: () = con
                .lset("sms_dead_letter", index as isize, "__DELETED__")
                .await?;
            let _: () = con.lrem("sms_dead_letter", 1, "__DELETED__").await?;
            return Ok(Some(parsed));
        }
    }
    Ok(None)
}

pub async fn get_all_dead_letter_jobs(
    con: &mut redis::aio::MultiplexedConnection,
) -> anyhow::Result<Vec<OutgoingJob>> {
    let jobs = get_raw_dead_letter_jobs(con).await?;
    let parsed: Vec<OutgoingJob> = jobs
        .into_iter()
        .filter_map(|j| serde_json::from_str(&j).ok())
        .collect();
    Ok(parsed)
}

pub async fn get_raw_dead_letter_jobs(
    con: &mut redis::aio::MultiplexedConnection,
) -> anyhow::Result<Vec<String>> {
    let jobs: Vec<String> = con.lrange("sms_dead_letter", 0, -1).await?;
    Ok(jobs)
}

pub async fn requeue_job_by_index(
    con: &mut redis::aio::MultiplexedConnection,
    index: usize,
    job: &str,
) -> anyhow::Result<()> {
    let _: () = con.rpush("sms_queue", job).await?;
    // Push to main queue
    let _: () = con.rpush("sms_queue", job).await?;

    // Replace with marker and remove
    let _: () = con
        .lset("sms_dead_letter", index as isize, "__DELETED__")
        .await?;

    let _: () = con.lrem("sms_dead_letter", 1, "__DELETED__").await?;

    Ok(())
}

pub async fn requeue_all_jobs(
    con: &mut redis::aio::MultiplexedConnection,
    jobs: &[String],
) -> anyhow::Result<usize> {
    let mut requeued = 0;

    for job in jobs {
        if job == "__DELETED__" {
            continue;
        }

        if serde_json::from_str::<OutgoingJob>(job).is_ok() {
            // Push to main queue
            let _: () = con.rpush("sms_queue", job).await?;

            // Mark for deletion
            let _: () = con.lrem("sms_dead_letter", 1, job).await?;
            requeued += 1;
        }
    }

    Ok(requeued)
}

pub async fn find_job_by_key(
    con: &mut redis::aio::MultiplexedConnection,
    key: &str,
) -> anyhow::Result<Option<OutgoingJob>> {
    let jobs = get_raw_dead_letter_jobs(con).await?;
    for (index, job) in jobs.iter().enumerate() {
        let parsed: OutgoingJob = match serde_json::from_str(job) {
            Ok(j) => j,
            Err(_) => continue,
        };

        if parsed.id.to_string() == key || key == index.to_string() {
            return Ok(Some(parsed));
        }
    }
    Ok(None)
}
