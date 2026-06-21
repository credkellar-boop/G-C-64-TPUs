pub mod ai_mode; 
pub mod telemetry; 
pub mod error;
pub mod spark;
pub mod auth;

// 1. Re-export the error so `crate::GoogleSuiteError` works anywhere
pub use error::GoogleSuiteError;

// 2. Define the custom Result alias so `crate::Result<T>` works anywhere
pub type Result<T> = std::result::Result<T, GoogleSuiteError>;
