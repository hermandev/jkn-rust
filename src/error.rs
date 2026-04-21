use thiserror::Error;

#[derive(Debug, Error)]
pub enum JknError {
    #[error("config error: {0}")]
    Config(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("path error: {0}")]
    Path(String),
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("crypto error: {0}")]
    Crypto(String),
}

pub type Result<T> = std::result::Result<T, JknError>;
