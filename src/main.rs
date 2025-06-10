use dotenv::dotenv;
use rust_challenge::factories::clickhouse::{ClickhouseClientConfig, ClickhouseFactory};
use rust_challenge::factories::defaults::generator;
use rust_challenge::repositories::storage::Storage;
use rust_challenge::services::analytics::Analytics;
use rust_challenge::services::stats::calculator::StatsCalculator;

fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let transfers = generator().build().generate(10_000);

    let mut storage = ClickhouseFactory::storage(ClickhouseClientConfig::from_env()?);
    // let mut storage = MockStorage::default();

    let calculator = StatsCalculator::new();
    storage.insert_all(transfers);

    let stats = Analytics::new(storage, calculator).get_stats();

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }

    Ok(())
}
