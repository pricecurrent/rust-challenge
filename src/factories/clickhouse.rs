use anyhow::Result;
use clickhouse::Client;

use crate::{repositories::clickhouse::ClickhouseStorage, utils::env::env_get};

#[derive(Debug)]
pub struct ClickhouseClientConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl ClickhouseClientConfig {
    pub fn from_env() -> Result<ClickhouseClientConfig> {
        Ok(ClickhouseClientConfig {
            host: env_get("CLICKHOUSE_URL")?,
            user: env_get("CLICKHOUSE_USER")?,
            password: env_get("CLICKHOUSE_PASSWORD")?,
            database: env_get("CLICKHOUSE_DB")?,
        })
    }
}

pub struct ClickhouseFactory;

impl ClickhouseFactory {
    pub async fn storage(config: ClickhouseClientConfig) -> Result<ClickhouseStorage> {
        let client = Client::default()
            .with_url(&config.host)
            .with_user(&config.user)
            .with_password(&config.password)
            .with_database(&config.database);

        let storage = ClickhouseStorage::new(client);

        storage.ensure_schema().await?;

        Ok(storage)
    }
}
