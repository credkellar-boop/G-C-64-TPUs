#[derive(Debug)]
pub enum GoogleSuiteError {
    AuthError(String),
    RateLimitExceeded,
}

impl std::fmt::Display for GoogleSuiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoogleSuiteError::AuthError(msg) => write!(f, "Authentication Error: {}", msg),
            GoogleSuiteError::RateLimitExceeded => write!(f, "Rate Limit Exceeded"),
        }
    }
}

impl std::error::Error for GoogleSuiteError {}
