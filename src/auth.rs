// ----------------------------------------------------------------------------
// Original Author and Architect: Darion Kellar
// ----------------------------------------------------------------------------

use std::env;

/// Represents the different types of authentication errors that can occur
/// during connection or request validation.
#[derive(Debug, PartialEq)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
    InsufficientPermissions,
}

/// A structure to hold authenticated client or node data across the network.
#[derive(Debug, Clone)]
pub struct AuthenticatedIdentity {
    pub client_id: String,
    pub role: String,
}

/// Validates an incoming API token.
/// In a production edge deployment, this would likely verify against 
/// a distributed key-value store, JWT signature, or secure enclave.
pub fn verify_token(token: &str) -> Result<AuthenticatedIdentity, AuthError> {
    if token.trim().is_empty() {
        return Err(AuthError::MissingToken);
    }

    // Standard practice: Pull the expected token from environment variables
    // or a configuration module. Using a default fallback for scaffolding.
    let expected_token = env::var("EDGE_API_TOKEN")
        .unwrap_or_else(|_| "dev_test_token_123".to_string());

    if token == expected_token {
        Ok(AuthenticatedIdentity {
            client_id: "edge_node_01".to_string(),
            role: "admin".to_string(),
        })
    } else {
        Err(AuthError::InvalidToken)
    }
}

// Basic unit tests to ensure the module compiles and functions as expected in CI
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_token_fails() {
        let result = verify_token("");
        assert_eq!(result.unwrap_err(), AuthError::MissingToken);
    }

    #[test]
    fn test_invalid_token_fails() {
        let result = verify_token("wrong_token");
        assert_eq!(result.unwrap_err(), AuthError::InvalidToken);
    }
}
