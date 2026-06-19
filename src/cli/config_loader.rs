pub fn load_local_config() -> String {
    std::env::var("GOOGLE_API_KEY").unwrap_or_else(|_| "NOT_SET".into())
}
