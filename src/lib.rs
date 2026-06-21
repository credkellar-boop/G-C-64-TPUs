pub mod ai_mode;
pub mod telemetry;
pub mod error;
pub mod spark;
pub mod auth;
pub mod models;
pub mod flow; //

pub use error::GoogleSuiteError;
pub type Result<T> = std::result::Result<T, GoogleSuiteError>;
