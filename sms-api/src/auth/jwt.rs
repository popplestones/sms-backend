use std::env;

use anyhow::Context;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::auth::{permissions::Permission, roles::Role};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
}

pub fn decode_jwt(token: &str) -> anyhow::Result<TokenData<Claims>> {
    let secret = env::var("APP_KEY").context("APP_KEY must be set in .env")?;
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data)
}
