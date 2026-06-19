pub struct Metrics { pub total_requests: u64 }
impl Metrics {
    pub fn increment(&mut self) { self.total_requests += 1; }
}
