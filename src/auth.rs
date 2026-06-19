use std::env;

pub struct AuthClient { pub api_key: String }

impl AuthClient {
    pub fn new() -> crate::Result<Self> {
        let key = env::var("GOOGLE_API_KEY").map_err(|_| crate::GoogleSuiteError::AuthError("Missing key".into()))?;
        Ok(Self { api_key: key })
    }
}
