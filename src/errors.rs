use thiserror::Error;

#[derive(Error, Debug)]
pub enum GoogleSuiteError {
    #[error("Authentication failed. Check your API key or Ultra plan status: {0}")]
    AuthError(String),

    #[error("API Rate limit exceeded. Wait before sending more requests.")]
    RateLimitExceeded,

    #[error("Insufficient Flow credits for this generation task.")]
    InsufficientCredits,

    #[error("I/O error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown system error occurred")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, GoogleSuiteError>;
