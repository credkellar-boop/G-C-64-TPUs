use serde::{Deserialize, Serialize};

pub mod flow_config;
pub mod flow_response;
pub mod spark_task;

#[derive(Serialize, Deserialize)]
pub struct GoogleResponse {
    pub status: String,
    pub payload: String,
}
