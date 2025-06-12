use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::anyhow;

pub trait Now {
    fn now_unix() -> anyhow::Result<u64>;
}

pub struct SystemNow;

impl Now for SystemNow {
    fn now_unix() -> anyhow::Result<u64> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| anyhow!("System time is misconfiged"))?;

        Ok(duration.as_secs())
    }
}
