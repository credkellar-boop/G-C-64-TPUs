#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = google_suite_rs::auth::AuthClient::new()?;
    println!("System ready with key: {}", auth.api_key);
    Ok(())
}
