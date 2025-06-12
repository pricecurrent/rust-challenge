use anyhow::Result;
use async_trait::async_trait;
use clickhouse::sql::Identifier;
use clickhouse::Client;

use crate::errors::StorageResult;
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

    pub async fn ensure_schema(&self) -> Result<()> {
        self.client
            .query("DROP TABLE IF EXISTS ?")
            .bind(Identifier(TABLE))
            .with_option("wait_end_of_query", "1")
            .execute()
            .await
            .with_context(&format!("Could not drop the table {}", TABLE))?;

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
            .with_context(&format!("Could not create table {}", TABLE))?;

        Ok(())
    }
}

#[async_trait]
impl Storage for ClickhouseStorage {
    async fn get_sorted(&self, transfer_ordering: TransferOrdering) -> Result<Vec<Transfer>> {
        let order_by_clause = match transfer_ordering {
            TransferOrdering::Raw => "",
            TransferOrdering::Chronological => " ORDER BY ts ASC",
            TransferOrdering::ByVolume => " ORDER BY amount DESC",
        };

        let query = format!("SELECT * from ? {}", order_by_clause);

        let res = self
            .client
            .query(&query)
            .bind(Identifier(TABLE))
            .fetch_all::<Transfer>()
            .await
            .with_context("Could not fetch transfers")?;

        Ok(res)
    }

    async fn insert_all(&mut self, transfers: &[Transfer]) -> Result<()> {
        let mut insert = self
            .client
            .insert("transfers")
            .with_context("Could not insert transfers")?;

        for row in transfers {
            insert
                .write(row)
                .await
                .with_context("Could not insert transfers")?;
        }

        insert
            .end()
            .await
            .with_context("Could not insert transfers")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factories::{
        clickhouse::{ClickhouseClientConfig, ClickhouseFactory},
        defaults::generator,
    };
    use anyhow::Result;
    use clickhouse::{
        test::{
            handlers::{self, RecordControl},
            Mock,
        },
        Client,
    };
    use dotenv::dotenv;

    #[tokio::test]
    async fn inserting() -> Result<()> {
        dotenv().ok();
        let mock = Mock::new();
        let client = Client::default().with_url(mock.url());
        let mut storage = ClickhouseStorage::new(client);
        let recording: RecordControl<Transfer> = mock.add(handlers::record());

        let transfers = generator().build().generate(20);

        storage.insert_all(&transfers).await?;

        let rows: Vec<Transfer> = recording.collect().await;

        assert_eq!(rows, transfers);

        Ok(())
    }

    #[tokio::test]
    async fn it_gets_sorted_data() -> Result<()> {
        dotenv().ok();
        let config = ClickhouseClientConfig::from_env()?;
        let mut storage = ClickhouseFactory::storage(config).await?;

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

        let res = storage.get_sorted(TransferOrdering::Chronological).await?;

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
        );

        Ok(())
    }
}
