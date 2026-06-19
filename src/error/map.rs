pub fn map_http_error(code: u16) -> crate::GoogleSuiteError {
    match code {
        401 => crate::GoogleSuiteError::AuthError("Invalid Key".into()),
        _ => crate::GoogleSuiteError::Unknown,
    }
}
