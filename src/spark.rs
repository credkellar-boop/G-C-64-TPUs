pub struct Agent;

impl Agent {
    pub async fn connect() -> crate::Result<Self> {
        Ok(Agent)
    }
}
