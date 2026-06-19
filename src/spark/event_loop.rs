pub async fn start_event_loop() {
    loop {
        // Poll for task triggers from MCP or web events
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
