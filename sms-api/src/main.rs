use anyhow::Context;
use clap::Parser;
use std::env;
use std::io::Write;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use sms_api::{
    cli::{self, Cli, Command},
    router::app,
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    ensure_app_url_env()?;

    let cli = Cli::parse();

    match cli.command {
        Some(Command::GenerateAppKey(args)) => {
            cli::generate_app_key::run(args)?;
            return Ok(());
        }
        Some(Command::CreateUser(args)) => {
            cli::create_user::run(args)?;
            return Ok(());
        }
        None => {}
    }

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize application state
    let state = AppState::new().context("Cannot initialize application state")?;

    // Initialize listener
    let addr = env::var("APP_URL").context("Missing env var APP_URL")?;
    let listener = TcpListener::bind(&addr)
        .await
        .context("Cannot listen on address, is it in use?")?;

    info!("SMS API Listening on {addr}");

    // Serve the application
    axum::serve(listener, app(state))
        .await
        .context("Unable to serve on {addr}, is it in use?")?;

    Ok(())
}

fn ensure_app_url_env() -> anyhow::Result<()> {
    if env::var("APP_URL").is_err() {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(".env")?;
        writeln!(file, "APP_URL=0.0.0.0:3000")?;
        println!("APP_URL=0.0.0.0:3000 written to .env");
        dotenvy::from_filename(".env").ok();
    }
    Ok(())
}
