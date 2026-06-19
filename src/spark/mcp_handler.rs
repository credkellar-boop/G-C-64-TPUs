pub fn format_mcp_message(cmd: &str) -> String {
    format!(r#"{{"jsonrpc": "2.0", "method": "{}", "params": {{}}}}"#, cmd)
}
