pub fn attach_headers(req: reqwest::RequestBuilder, key: &str) -> reqwest::RequestBuilder {
    req.header("Authorization", format!("Bearer {}", key))
}
