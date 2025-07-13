use crate::redis::get_all_dead_letter_jobs;

pub async fn run(
    con: &mut redis::aio::MultiplexedConnection,
    file: Option<String>,
) -> anyhow::Result<()> {
    let jobs = get_all_dead_letter_jobs(con).await?;
    let json = serde_json::to_string_pretty(&jobs)?;

    if let Some(file) = file {
        std::fs::write(file, json)?;
        println!("Exported jobs to file.");
    } else {
        println!("{json}");
    }

    Ok(())
}
