use mockall::automock;

use crate::models::{transfer::Transfer, user_stats::UserStats};
use crate::services::stats::accumulator::PriceAccumulator;
use std::collections::HashMap;

#[automock]
pub trait CalculatesStats {
    fn calculate_user_stats(&self, transfers: &[Transfer]) -> Vec<UserStats>;
}

pub struct StatsCalculator;

impl StatsCalculator {
    pub fn new() -> Self {
        StatsCalculator
    }
}

impl CalculatesStats for StatsCalculator {
    fn calculate_user_stats(&self, transfers: &[Transfer]) -> Vec<UserStats> {
        let mut accumulators: HashMap<&str, PriceAccumulator> = HashMap::new();
        for t in transfers {
            accumulators
                .entry(&t.to)
                .or_default()
                .accumulate(t.amount, t.usd_price);

            accumulators
                .entry(&t.from)
                .or_default()
                .accumulate(-t.amount, t.usd_price);
        }

        accumulators
            .iter()
            .map(|(&address, accumulator)| UserStats {
                address: address.to_string(),
                total_volume: accumulator.total_volume(),
                avg_buy_price: accumulator.avg_buy_price(),
                avg_sell_price: accumulator.avg_sell_price(),
                max_balance: accumulator.max_balance(),
            })
            .collect::<Vec<UserStats>>()
    }
}

#[cfg(test)]
mod tests {

    use crate::factories::defaults::generator;
    use crate::{
        factories::generator::TransferGenConfig,
        utils::time::{Now, SystemNow},
    };
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn one_transfer() -> Result<(), anyhow::Error> {
        let config = TransferGenConfig {
            min_amount: 1.0,
            max_amount: 1.0,
            min_price: 10.0,
            max_price: 10.0,
            ..Default::default()
        };

        let transfers = generator().with_config(config).build().generate(1)?;

        // Act: calculate user stats over the transfers
        let stats = StatsCalculator.calculate_user_stats(&transfers);
        assert_eq!(
            stats.len(),
            2,
            "One transfer should generate 2 stats for sender and receiver"
        );

        let sender = &transfers[0].from;
        let receiver = &transfers[0].to;

        let sender_stat = stats
            .iter()
            .find(|&stat| stat.address == *sender)
            .ok_or_else(|| anyhow!("Sender is not found in stats"))?;

        let receiver_stat = stats
            .iter()
            .find(|&stat| stat.address == *receiver)
            .ok_or_else(|| anyhow!("Receiver is not found in stats"))?;

        // Assert sender
        assert_eq!(&sender_stat.address, sender);
        assert_eq!(&sender_stat.max_balance, &0.0, "The sender balance was 0 before transfer, it ended up being -1 afterwards, hence the max balance is 0");
        assert_eq!(&sender_stat.total_volume, &1.0);
        assert_eq!(&sender_stat.avg_sell_price, &10.0);
        assert_eq!(&sender_stat.avg_buy_price, &0.0);

        // Assert Receiver
        assert_eq!(&receiver_stat.address, receiver);
        assert_eq!(&receiver_stat.max_balance, &1.0);
        assert_eq!(&receiver_stat.total_volume, &1.0);
        assert_eq!(&receiver_stat.avg_buy_price, &10.0);
        assert_eq!(&receiver_stat.avg_sell_price, &0.0);

        Ok(())
    }

    #[test]
    fn two_transfers_between_same_addresses() -> Result<(), anyhow::Error> {
        // Arrange
        let bob = "0xBob".to_string();
        let john = "0xJohn".to_string();

        let transfers = vec![
            Transfer {
                ts: SystemNow::now_unix()?,
                from: bob.clone(),
                to: john.clone(),
                amount: 10.0,
                usd_price: 50.0,
            },
            Transfer {
                ts: SystemNow::now_unix()?,
                from: john.clone(),
                to: bob.clone(),
                amount: 5.0,
                usd_price: 25.0,
            },
        ];

        // Act:
        let stats = StatsCalculator.calculate_user_stats(&transfers);
        assert_eq!(stats.len(), 2, "Only 2 actors");

        let bob_stats = stats
            .iter()
            .find(|&stat| stat.address == bob.clone())
            .ok_or_else(|| anyhow!("Bob is not found in stats"))?;

        let john_stats = stats
            .iter()
            .find(|&stat| stat.address == john.clone())
            .ok_or_else(|| anyhow!("John is not found in stats"))?;

        // Assert Bob
        assert_eq!(&bob_stats.address, &bob);
        assert_eq!(&bob_stats.max_balance, &0.0);
        assert_eq!(&bob_stats.total_volume, &15.0);
        assert_eq!(&bob_stats.avg_sell_price, &50.0);
        assert_eq!(&bob_stats.avg_buy_price, &25.0);

        // Assert John
        assert_eq!(&john_stats.address, &john);
        assert_eq!(&john_stats.max_balance, &10.0);
        assert_eq!(&john_stats.total_volume, &15.0);
        assert_eq!(&john_stats.avg_buy_price, &50.0);
        assert_eq!(&john_stats.avg_sell_price, &25.0);

        Ok(())
    }

    #[test]
    fn max_balance() -> Result<(), anyhow::Error> {
        // Arrange
        let bob = "0xBob".to_string();

        let transfers = vec![
            Transfer {
                from: bob.clone(),
                amount: 20.0,
                ..Default::default()
            },
            Transfer {
                to: bob.clone(),
                amount: 20.0,
                ..Default::default()
            },
            Transfer {
                from: bob.clone(),
                amount: 5.0,
                ..Default::default()
            },
            Transfer {
                to: bob.clone(),
                amount: 30.0,
                ..Default::default()
            },
            Transfer {
                from: bob.clone(),
                amount: 10.0,
                ..Default::default()
            },
        ];

        // Act:
        let stats = StatsCalculator.calculate_user_stats(&transfers);

        let bob_stats = stats
            .iter()
            .find(|&stat| stat.address == bob.clone())
            .ok_or_else(|| anyhow!("Bob is not found in stats"))?;

        // Assert Bob
        assert_eq!(&bob_stats.max_balance, &25.0);

        Ok(())
    }

    #[test]
    fn test_chronological_sorting_affects_max_balance_calculation(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let bob = "0xBob".to_string();

        // Bob's transaction story:
        // 1. (ts=100) Receives 1000 tokens → balance: 1000 (max so far: 1000)
        // 2. (ts=200) Spends 200 tokens   → balance: 800  (max so far: 1000)
        // 3. (ts=300) Receives 500 tokens → balance: 1300 (max so far: 1300) ← real MAX
        // 4. (ts=400) Spends 800 tokens   → balance: 500  (max so far: 1300)
        let transfers_chronological = vec![
            Transfer {
                ts: 100,
                from: "0xMint".to_string(),
                to: bob.clone(),
                amount: 1000.0,
                usd_price: 1.0,
            },
            Transfer {
                ts: 200,
                from: bob.clone(),
                to: "0xShop".to_string(),
                amount: 200.0,
                usd_price: 1.0,
            },
            Transfer {
                ts: 300,
                from: "0xBonus".to_string(),
                to: bob.clone(),
                amount: 500.0,
                usd_price: 1.0,
            },
            Transfer {
                ts: 400,
                from: bob.clone(),
                to: "0xFee".to_string(),
                amount: 800.0,
                usd_price: 1.0,
            },
        ];

        // Same transfers but in wrong chronological order
        let transfers_random_order = vec![
            transfers_chronological[3].clone(), // ts=400: spend 800
            transfers_chronological[0].clone(), // ts=100: receive 1000
            transfers_chronological[2].clone(), // ts=300: receive 500
            transfers_chronological[1].clone(), // ts=200: spend 200
        ];

        // Act
        let stats_correct_order = StatsCalculator.calculate_user_stats(&transfers_chronological);
        let stats_wrong_order = StatsCalculator.calculate_user_stats(&transfers_random_order);

        // Find Bob's stats in both results
        let bob_correct = stats_correct_order
            .iter()
            .find(|s| s.address == bob)
            .ok_or_else(|| anyhow!("Bob is not found in stats"))?;

        let bob_wrong = stats_wrong_order
            .iter()
            .find(|s| s.address == bob)
            .ok_or_else(|| anyhow!("Bob is not found in stats"))?;

        // Assert
        assert_eq!(
            bob_correct.total_volume, bob_wrong.total_volume,
            "Total volume should be same regardless of order"
        );
        assert_eq!(
            bob_correct.avg_buy_price, bob_wrong.avg_buy_price,
            "Average prices should be same regardless of order"
        );
        assert_eq!(
            bob_correct.avg_sell_price, bob_wrong.avg_sell_price,
            "Average prices should be same regardless of order"
        );

        // CRITICAL!
        assert_eq!(
            bob_correct.max_balance, 1300.0,
            "Correct chronological order: max balance should be 1300"
        );
        assert_ne!(bob_wrong.max_balance, 1300.0);

        // Show what actually happens with wrong order
        println!(
            "Correct chronological order - Bob's max balance: {}",
            bob_correct.max_balance
        );
        println!(
            "Random order - Bob's max balance: {}",
            bob_wrong.max_balance
        );
        println!(
            "Financial impact: ${} difference in reported max balance!",
            (bob_correct.max_balance - bob_wrong.max_balance).abs()
        );

        Ok(())
    }
}
