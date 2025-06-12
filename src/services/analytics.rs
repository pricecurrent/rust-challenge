use crate::{
    models::{transfer::TransferOrdering, user_stats::UserStats},
    repositories::storage::Storage,
};
use anyhow::{anyhow, Result};

use super::stats::calculator::CalculatesStats;

pub struct Analytics<C, S>
where
    S: Storage,
    C: CalculatesStats,
{
    storage: S,
    calculator: C,
}

impl<C, S> Analytics<C, S>
where
    S: Storage,
    C: CalculatesStats,
{
    pub fn new(storage: S, calculator: C) -> Self {
        Analytics {
            storage,
            calculator,
        }
    }

    pub async fn get_stats(&self) -> Result<Vec<UserStats>> {
        let transfers = self
            .storage
            .get_sorted(TransferOrdering::Chronological)
            .await
            .map_err(|e| anyhow!("Could not calculate stats: {}", e))?;

        Ok(self.calculator.calculate_user_stats(&transfers))
    }
}

#[cfg(test)]
mod tests {

    use anyhow::anyhow;
    use anyhow::Result;

    use crate::models::transfer::Transfer;
    use crate::repositories::mock::MockStorage;
    use crate::repositories::storage::Storage;
    use crate::services::stats::calculator::StatsCalculator;
    use crate::{models::user_stats::UserStats, services::stats::calculator::MockCalculatesStats};

    use super::Analytics;

    #[tokio::test]
    async fn uses_chronologically_ordered_trasnfers_to_calculate_stats() {
        let expected_transfers = vec![Transfer::default()];

        let mut storage = MockStorage::default();
        let _ = storage.insert_all(&expected_transfers);

        let calculator = StatsCalculator::new();

        let analytics = Analytics::new(storage, calculator);

        let _ = analytics.get_stats().await;
    }

    #[tokio::test]
    async fn delegates_to_calculator_to_retrieve_the_stats() -> Result<()> {
        let storage = MockStorage::default();
        let mut calculator = MockCalculatesStats::new();

        let expected_stats = vec![UserStats {
            max_balance: 100.0,
            ..Default::default()
        }];

        calculator
            .expect_calculate_user_stats()
            .once()
            .returning(move |_| expected_stats.clone());

        let analytics = Analytics::new(storage, calculator);

        let stats = analytics.get_stats().await?;

        assert_eq!(stats.len(), 1);

        assert_eq!(
            stats
                .first()
                .ok_or_else(|| anyhow!("Exactly one result expected"))?
                .max_balance,
            100.0,
            "Expecting mocked results"
        );

        Ok(())
    }
}
