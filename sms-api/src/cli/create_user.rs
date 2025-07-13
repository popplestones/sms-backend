use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::auth::{permissions::Permission, roles::Role};

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long)]
    pub user_id: String,

    #[arg(
        short,
        long,
        help = "Comman-separated permissiosns (e.g. sms.send, sms.view)"
    )]
    pub permissions: Option<String>,

    #[arg(short, long, help = "Comma-separated roles (e.g. admin, user)")]
    pub roles: Option<String>,

    #[arg(long, default_value = "720")]
    pub expiry_hours: u64,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let app_key = std::env::var("APP_KEY").context("APP_KEY must be set in .env")?;

    let permissions = args
        .permissions
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<Permission>())
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse permissions")?;

    let roles = args
        .roles
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<Role>())
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse roles")?;

    let exp = (Utc::now() + Duration::hours(args.expiry_hours as i64)).timestamp();

    let claims = crate::auth::jwt::Claims {
        sub: args.user_id,
        exp,
        permissions,
        roles,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_key.as_bytes()),
    )?;
    println!("{token}");
    Ok(())
}
