use dotenv::dotenv;
use rust_challenge::factories::clickhouse::{ClickhouseClientConfig, ClickhouseFactory};
use rust_challenge::factories::defaults::generator;
use rust_challenge::repositories::storage::Storage;
use rust_challenge::services::analytics::Analytics;
use rust_challenge::services::stats::calculator::StatsCalculator;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let transfers = generator().build().generate(20);

    let config = ClickhouseClientConfig::from_env()?;
    let mut storage = ClickhouseFactory::storage(config).await;
    // let mut storage = MockStorage::default();
    let _ = storage.insert_all(&transfers).await;

    let calculator = StatsCalculator::new();

    let stats = Analytics::new(storage, calculator).get_stats().await;

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }

    Ok(())
}
