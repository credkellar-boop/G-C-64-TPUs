pub fn check_env() -> bool {
    let key = std::env::var("GOOGLE_API_KEY");
    key.is_ok()
}
