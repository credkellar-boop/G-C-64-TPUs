pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod spark;

// Change the lowercase 's' to a capital 'S'
pub type Result<T> = std::result::Result<T, error::GoogleSuiteError>;

pub fn init() {
    // Placeholder for initialization logic
}