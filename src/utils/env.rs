use anyhow::Context;

pub fn env_get(key: &str) -> Result<String, anyhow::Error> {
    let v = std::env::var(key).context(format!("Missin env variable for {}", &key))?;

    Ok(v)
}
