use google_suite_rs::flow;

#[tokio::main]
async fn main() {
    let result = flow::generate_image("Modern cloud UI", "Omni").await;
    println!("Generation status: {:?}", result);
}
