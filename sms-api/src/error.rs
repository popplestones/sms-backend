pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
}
