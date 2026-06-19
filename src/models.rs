use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GoogleResponse {
    pub status: String,
    pub payload: String,
}
