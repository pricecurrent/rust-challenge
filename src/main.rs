use dotenv::dotenv;
use rust_challenge::app::App;
use rust_challenge::factories::clickhouse::{ClickhouseClientConfig, ClickhouseFactory};
use rust_challenge::factories::defaults::generator;
use rust_challenge::services::stats::calculator::StatsCalculator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let generator = generator().build();

    let config = ClickhouseClientConfig::from_env()?;
    let storage = ClickhouseFactory::storage(config).await?;

    // let mut storage = MockStorage::default();

    let calculator = StatsCalculator::new();

    let app = App {
        storage,
        calculator,
        generator,
    };

    let stats = app.run(20).await?;

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }

    Ok(())
}
