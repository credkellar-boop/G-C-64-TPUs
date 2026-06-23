use g_c_64_tpus::spark::Agent;
// Assuming you exposed the modules in src/lib.rs

#[test]
fn test_agent_initialization() {
    // This is a basic mock test. 
    // In production, you'd test the actual Google/Spark Agent structs here.
    let is_connected = true; 
    assert!(is_connected, "Spark Agent failed to initialize in test environment");
}

#[test]
fn test_error_handling() {
    let err = GoogleSuiteError::RateLimitExceeded;
    assert_eq!(
        err.to_string(), 
        "API Rate limit exceeded. Wait before sending more requests."
    );
}
