use anyhow::Result;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init() -> Result<()> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("newgpa=info,warn"));
    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .try_init()
        .ok();
    Ok(())
}

pub fn redact(value: &str) -> &'static str {
    let _ = value;
    "[redacted]"
}
