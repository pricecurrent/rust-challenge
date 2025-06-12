use anyhow::Result;
use rust_challenge::{
    app::App, factories::defaults::generator, repositories::mock::MockStorage,
    services::stats::calculator::StatsCalculator,
};

#[tokio::test]
async fn app_runs() -> Result<()> {
    let generator = generator().build();
    let storage = MockStorage::default();
    let calculator = StatsCalculator::new();

    let app = App {
        storage,
        calculator,
        generator,
    };

    let stats = app.run(10).await?;

    assert!(!stats.is_empty());

    Ok(())
}
