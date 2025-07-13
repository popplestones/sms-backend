use crate::redis::find_job_by_key;

pub async fn run(con: &mut redis::aio::MultiplexedConnection, key: &str) -> anyhow::Result<()> {
    match find_job_by_key(con, key).await? {
        Some(job) => {
            println!("{}", serde_json::to_string_pretty(&job)?);
        }
        None => {
            println!("No job found for key: {key}");
        }
    }
    Ok(())
}
