use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project_id: String,
    pub vpc_network: String,
    pub tpu_node_count: u16,
    pub gke_cluster_name: String,
    pub enable_public_layer: bool,
}

impl Config {
    /// Generates a default configuration for the 64-TPU architecture
    pub fn default_tpu_cluster() -> Self {
        Self {
            project_id: String::from("g-c-64-tpus-core"),
            vpc_network: String::from("tpu-isolated-vpc"),
            tpu_node_count: 64, // Uniting 64 TPUs via dedicated links
            gke_cluster_name: String::from("ai-orchestration-cluster"),
            enable_public_layer: true, // Spine-leaf structure active
        }
    }
}
