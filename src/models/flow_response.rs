use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FlowResponse {
    pub asset_url: String,
    pub generation_id: String,
    pub credit_usage: u32,
}
