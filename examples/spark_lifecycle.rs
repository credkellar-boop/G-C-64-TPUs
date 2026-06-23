use g_c_64_tpus::spark::Agent;

#[tokio::main]
async fn main() {
    let agent = Agent::connect().await.expect("Failed to connect");
    println!("Agent is live and awaiting tasks...");
}
