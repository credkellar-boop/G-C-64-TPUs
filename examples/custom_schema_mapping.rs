#[tokio::main]
async fn main() {
    let raw_data = r#"{"asset_url": "..."}"#;
    let decoded: google_suite_rs::models::flow_response::FlowResponse = 
        serde_json::from_str(raw_data).unwrap();
    println!("Parsed asset: {}", decoded.asset_url);
}
