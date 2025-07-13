use clap::{Parser, Subcommand};

pub mod create_user;
pub mod generate_app_key;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    GenerateAppKey(generate_app_key::Args),
    CreateUser(create_user::Args),
}
