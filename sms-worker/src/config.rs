use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WorkerConfig {
    pub retry: RetryConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub backoff_factor: f64,
    pub max_delay_ms: u64,
}

impl WorkerConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}

impl RetryConfig {
    pub fn delay_for_attempt(&self, attempt: usize) -> u64 {
        let base = self.initial_delay_ms as f64 * self.backoff_factor.powi((attempt - 1) as i32);
        base.min(self.max_delay_ms as f64) as u64
    }
}
