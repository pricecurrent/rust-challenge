use anyhow::Context;
use async_trait::async_trait;
use clickhouse::Client;
use clickhouse::{error::Result, sql::Identifier};

use crate::models::transfer::{Transfer, TransferOrdering};

use super::storage::Storage;

pub const TABLE: &str = "transfers";

pub struct ClickhouseStorage {
    client: Client,
}

impl ClickhouseStorage {
    pub fn new(client: Client) -> ClickhouseStorage {
        ClickhouseStorage { client }
    }

    pub async fn ensure_schema(&self) -> anyhow::Result<()> {
        self.client
            .query("DROP TABLE IF EXISTS ?")
            .bind(Identifier(TABLE))
            .with_option("wait_end_of_query", "1")
            .execute()
            .await?;

        let query = r"
            CREATE TABLE IF NOT EXISTS ? (
                ts UInt64,
                from String,
                to String,
                amount Float64,
                usd_price Float64
            ) ENGINE = MergeTree()
            ORDER BY (ts, from, to)
        ";

        self.client
            .query(query)
            .bind(Identifier(TABLE))
            .execute()
            .await
            .context("Failed to create transfers table")?;

        Ok(())
    }
}

#[async_trait]
impl Storage for ClickhouseStorage {
    async fn get_sorted(&self, transfer_ordering: TransferOrdering) -> Result<Vec<Transfer>> {
        let ordery_by_clause = match transfer_ordering {
            TransferOrdering::Raw => "",
            TransferOrdering::Chronological => " ORDER BY ts ASC",
            TransferOrdering::ByVolume => " ORDER BY amount DESC",
        };

        let query = format!("SELECT * from ? {}", ordery_by_clause);

        let res = self
            .client
            .query(&query)
            .bind(Identifier(TABLE))
            .fetch_all::<Transfer>()
            .await;

        res
    }

    async fn insert_all(&mut self, transfers: &[Transfer]) -> Result<()> {
        let mut insert = self.client.insert("transfers")?;
        for row in transfers {
            insert.write(row).await?;
        }
        insert.end().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factories::{
        clickhouse::{ClickhouseClientConfig, ClickhouseFactory},
        defaults::generator,
    };
    use clickhouse::{
        test::{
            handlers::{self, RecordControl},
            Mock,
        },
        Client,
    };
    use dotenv::dotenv;

    #[tokio::test]
    async fn inserting() {
        dotenv().ok();
        let mock = Mock::new();
        let client = Client::default().with_url(mock.url());
        let mut storage = ClickhouseStorage::new(client);
        let recording: RecordControl<Transfer> = mock.add(handlers::record());

        let transfers = generator().build().generate(20);

        storage.insert_all(&transfers).await.unwrap();

        let rows: Vec<Transfer> = recording.collect().await;

        assert_eq!(rows, transfers);
    }

    #[tokio::test]
    async fn it_gets_sorted_data() {
        dotenv().ok();
        let config = ClickhouseClientConfig::from_env().unwrap();
        let mut storage = ClickhouseFactory::storage(config).await;

        let transfers = vec![
            Transfer {
                ts: 200,
                ..Default::default()
            },
            Transfer {
                ts: 600,
                ..Default::default()
            },
            Transfer {
                ts: 100,
                ..Default::default()
            },
        ];

        let _ = storage.insert_all(&transfers).await;

        let res = storage
            .get_sorted(TransferOrdering::Chronological)
            .await
            .unwrap();

        assert_eq!(
            res,
            vec![
                Transfer {
                    ts: 100,
                    ..Default::default()
                },
                Transfer {
                    ts: 200,
                    ..Default::default()
                },
                Transfer {
                    ts: 600,
                    ..Default::default()
                },
            ]
        )
    }
}
