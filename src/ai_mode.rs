pub struct SearchAgent { pub query: String }

impl SearchAgent {
    pub fn new(q: &str) -> Self { Self { query: q.to_string() } }
}
