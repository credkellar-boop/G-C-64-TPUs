use g_c_64_tpus::{flow, spark, ai_mode};
use infinite_zer0::Middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 1. AI Mode: Set up an Information Agent to monitor a topic
    let web_tracker = ai_mode::SearchAgent::new(
        "monitor open-source repository updates"
    );

    // 2. Gemini Spark: Create a background trigger that waits for the AI Mode alert
    // The `?` here works perfectly now because `main` returns a Result!
    let spark_agent = spark::Agent::connect_mcp_integration().await?;

    // 3. Google Flow: When triggered, generate a visual report using Omni
    spark_agent.on_alert(web_tracker, |alert_data| async {
        
        // FIX: Replaced `await?` with `await.unwrap()` to handle the error inside the closure
        let visual_report = flow::generate_image(&alert_data, "Gemini Omni").await.unwrap();
        
        println!(
            "Generated report using Flow credits. Saved to: {}",
            visual_report.path
        );
    });

    Ok(())
}
