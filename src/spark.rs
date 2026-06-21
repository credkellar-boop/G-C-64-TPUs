pub struct Agent;

impl Agent {
    pub async fn connect() -> crate::Result<Self> {
        Ok(Agent)
    }

    pub async fn connect_mcp_integration() -> crate::Result<Self> {
        Self::connect().await
    }
}
