use anyhow::Result;
use clap::{Parser, Subcommand};
use sms_deadletter::commands::{delete, export, list, requeue, view};

#[derive(Parser)]
#[command(name = "sms-deadletter")]
#[command(about = "Inspect and manage dead-lettered SMS jobs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    View {
        key: String,
    },
    Requeue {
        key: String,
    },
    RequeueAll,
    Delete {
        key: String,
    },
    Export {
        #[arg(short, long)]
        file: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".into());
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_multiplexed_async_connection().await?;

    match cli.command {
        Commands::List => {
            list::run(&mut con).await?;
        }
        Commands::View { key } => {
            view::run(&mut con, &key).await?;
        }
        Commands::Requeue { key } => {
            requeue::one(&mut con, &key).await?;
        }
        Commands::RequeueAll => {
            requeue::all(&mut con).await?;
        }
        Commands::Delete { key } => {
            delete::run(&mut con, &key).await?;
        }
        Commands::Export { file } => {
            export::run(&mut con, file).await?;
        }
    }

    Ok(())
}
