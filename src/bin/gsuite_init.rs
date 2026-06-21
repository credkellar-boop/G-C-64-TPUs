pub fn check_env() -> bool {
    let key = std::env::var("GOOGLE_API_KEY");
    key.is_ok()
}

fn main() {
    if check_env() {
        println!("Environment is ready.");
    } else {
        println!("GOOGLE_API_KEY is missing.");
    }
}
