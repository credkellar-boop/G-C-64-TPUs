pub struct FlowEngine { pub credit_balance: u32 }

impl FlowEngine {
    pub fn new(balance: u32) -> Self { Self { credit_balance: balance } }
}
