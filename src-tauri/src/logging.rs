pub fn init() {
    let filter = std::env::var("CODEX_COMPANION_LOG").unwrap_or_else(|_| "warn".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}
