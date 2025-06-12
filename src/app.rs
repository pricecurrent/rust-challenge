use crate::{
    factories::generator::TransferGenerator,
    models::user_stats::UserStats,
    repositories::storage::Storage,
    services::{analytics::Analytics, stats::calculator::CalculatesStats},
};

pub struct App<S: Storage, C: CalculatesStats> {
    pub storage: S,
    pub calculator: C,
    pub generator: Box<dyn TransferGenerator>,
}

impl<S, C> App<S, C>
where
    S: Storage,
    C: CalculatesStats,
{
    pub async fn run(mut self, transfer_count: usize) -> anyhow::Result<Vec<UserStats>> {
        let transfers = self.generator.generate(transfer_count);

        self.storage.insert_all(&transfers).await?;

        Analytics::new(self.storage, self.calculator)
            .get_stats()
            .await
    }
}
