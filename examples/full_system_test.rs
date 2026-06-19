#[tokio::main]
async fn main() -> crate::Result<()> {
    let auth = crate::auth::AuthClient::new()?;
    println!("System ready with key: {}", auth.api_key);
    Ok(())
}
