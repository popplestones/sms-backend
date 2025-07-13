use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

const JWT_SECRET: &[u8] = b"secret-key";

pub fn decode_jwt(token: &str) -> anyhow::Result<TokenData<Claims>> {
    let key = DecodingKey::from_secret(JWT_SECRET);
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data)
}
