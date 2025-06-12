use anyhow::anyhow;
use clickhouse::error::{Error, Result};

pub trait StorageResult<T> {
    fn with_context(self, context: &str) -> anyhow::Result<T>;
}

impl<T> StorageResult<T> for Result<T, Error> {
    fn with_context(self, context: &str) -> anyhow::Result<T> {
        self.map_err(|e| anyhow!("{}: {}", context, e))
    }
}
