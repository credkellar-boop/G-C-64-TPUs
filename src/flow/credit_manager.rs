pub struct CreditManager;

impl CreditManager {
    pub fn is_sufficient(balance: u32, cost: u32) -> bool {
        balance >= cost
    }
}
