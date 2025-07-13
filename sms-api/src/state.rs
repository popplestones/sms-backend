use std::env::var;

use anyhow::Context;
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: Client,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let redis_url = var("REDIS_URL").context("Missing env var REDIS_URL")?;

        let redis_client =
            Client::open(redis_url).context("Cannot connect to Redis on {redis_url}")?;

        Ok(Self { redis_client })
    }
}
