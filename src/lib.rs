pub mod auth;
pub mod client;
pub mod config;
pub mod error; // Assuming your error file is named error.rs
pub mod spark; // This makes the Agent available to your tests

// If your spark.rs file relies on a global Result type (crate::Result), 
// you may also need to define it here like this:
// pub type Result<T> = std::result::Result<T, error::GoogleSuiteError>;

pub fn init() {
    // Placeholder for initialization logic
}
