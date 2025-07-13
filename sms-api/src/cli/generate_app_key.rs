use anyhow::Context;
use rand::RngCore;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, default_value = ".env")]
    pub output: String,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let mut key_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut key_bytes);
    let hex_key = hex::encode(key_bytes);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&args.output)?;
    writeln!(file, "APP_KEY={hex_key}").context("Failed to write to .env")?;

    println!("APP_KEY written to {}", args.output);
    Ok(())
}
