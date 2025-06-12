use dotenv::dotenv;
use rust_challenge::factories::clickhouse::{ClickhouseClientConfig, ClickhouseFactory};
use rust_challenge::factories::defaults::generator;
// use rust_challenge::repositories::mock::MockStorage;
use rust_challenge::repositories::storage::Storage;
use rust_challenge::services::analytics::Analytics;
use rust_challenge::services::stats::calculator::StatsCalculator;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let transfers = generator().build().generate(20);

    let config = match ClickhouseClientConfig::from_env() {
        Ok(config) => config,
        Err(e) => return eprintln!("{e}"),
    };

    let mut storage = match ClickhouseFactory::storage(config).await {
        Ok(stroage) => stroage,
        Err(e) => return eprintln!("{e}"),
    };

    // let mut storage = MockStorage::default();

    match storage.insert_all(&transfers).await {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }

    let calculator = StatsCalculator::new();

    let stats = match Analytics::new(storage, calculator).get_stats().await {
        Ok(stats) => stats,
        Err(e) => return eprintln!("{e}"),
    };

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }
}
