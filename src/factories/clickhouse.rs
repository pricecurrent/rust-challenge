use anyhow::Context;
use clickhouse::Client;

use crate::{repositories::clickhouse::ClickhouseStorage, utils::env::env_get};

pub struct ClickhouseClientConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub port: u32,
    pub database: String,
}

impl ClickhouseClientConfig {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        Ok(ClickhouseClientConfig {
            host: env_get("CLICKHOUSE_HOST")?,
            user: env_get("CLICKHOUSE_USER")?,
            password: env_get("CLICKHOUSE_PASSWORD")?,
            port: env_get("CLICKHOUSE_PORT")?
                .parse()
                .context("CLICKHOUSE_PORT env variable should be a valid number")?,
            database: env_get("CLICKHOUSE_DATABASE")?,
        })
    }
}

pub struct ClickhouseFactory;

impl ClickhouseFactory {
    pub fn storage(config: ClickhouseClientConfig) -> ClickhouseStorage<Client> {
        let client = Client::default()
            .with_url(config.host)
            .with_user(config.user)
            .with_password(config.password)
            .with_database(config.database);

        ClickhouseStorage::new(client)
    }
}
