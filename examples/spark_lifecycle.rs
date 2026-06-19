use google_suite_rs::spark::Agent;

#[tokio::main]
async fn main() {
    let agent = Agent::connect().await.expect("Failed to connect");
    println!("Agent is live and awaiting tasks...");
}
