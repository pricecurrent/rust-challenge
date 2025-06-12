use anyhow::{Context, Result};

pub fn env_get(key: &str) -> Result<String> {
    let v = std::env::var(key).context(format!("Env variable with key `{}` not found. ", key))?;

    Ok(v)
}
