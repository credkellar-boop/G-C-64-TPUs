#[derive(Debug)]
pub enum GoogleSuiteError {
    AuthError(String),
    // You can add more error variants here later
}

impl std::fmt::Display for GoogleSuiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoogleSuiteError::AuthError(msg) => write!(f, "Authentication Error: {}", msg),
        }
    }
}

impl std::error::Error for GoogleSuiteError {}
