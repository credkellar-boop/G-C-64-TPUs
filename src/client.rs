use reqwest::Client;
use std::time::Duration;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct CloudClient {
    http_client: Client,
    config: Config,
}

impl CloudClient {
    /// Initializes a new CloudClient with the provided configuration
    pub fn new(config: Config) -> Self {
        // Build a robust client with a 30-second timeout
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to initialize the HTTP client");

        Self {
            http_client,
            config,
        }
    }

    /// Example asynchronous method to ping the cluster network
    pub async fn check_network_health(&self) -> Result<(), reqwest::Error> {
        println!(
            "Pinging dual-layer network for {} TPUs on VPC: {}",
            self.config.tpu_node_count, self.config.vpc_network
        );
        
        // Placeholder for an actual GKE or internal health-check endpoint
        // let res = self.http_client.get("https://your-gke-endpoint/health").send().await?;
        
        Ok(())
    }
}
