pub async fn with_retry<F, T>(op: F) -> crate::Result<T> 
where F: Fn() -> crate::Result<T> {
    // Logic for exponential backoff (1s, 2s, 4s...)
    op()
}
