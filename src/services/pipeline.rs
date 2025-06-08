use crate::model::{Transfer, UserStats};
use std::collections::HashMap;

#[derive(Default, Debug)]
struct PriceAccumulator {
    weight_sell_amount: f64,
    weight_buy_amount: f64, // total usd payed
    buy_volume: f64,
    sell_volume: f64,
    max_balance: f64,
}

impl PriceAccumulator {
    pub fn add(&mut self, amount: f64, usd_price: f64) {
        if amount > 0.0 {
            self.weight_buy_amount += amount * usd_price;
            self.buy_volume += amount;
            self.max_balance += amount;
        } else if amount < 0.0 {
            self.weight_sell_amount += amount.abs() * usd_price;
            self.sell_volume += amount.abs();
        }
    }

    pub fn avg_buy_price(&self) -> f64 {
        self.weight_buy_amount / self.buy_volume
    }

    pub fn avg_sell_price(&self) -> f64 {
        self.weight_sell_amount / self.sell_volume
    }

    pub fn total_volume(&self) -> f64 {
        self.sell_volume + self.buy_volume
    }

    pub fn max_balance(&self) -> f64 {
        self.max_balance
    }
}

pub fn calculate_user_stats(transfers: &[Transfer]) -> Vec<UserStats> {
    let mut accumulators: HashMap<&str, PriceAccumulator> = HashMap::new();
    for t in transfers {
        let buy_accumulator = accumulators.entry(&t.to).or_default();
        buy_accumulator.add(t.amount, t.usd_price);

        let sell_accumulator = accumulators.entry(&t.from).or_default();
        sell_accumulator.add(-t.amount, t.usd_price);
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

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use crate::{defaults::generator, services::generator::TransferGenConfig};

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

        let transfers = generator().with_config(config).build().generate(1);

        // Act: calculate user stats over the transfers
        let stats = calculate_user_stats(&transfers);
        assert_eq!(
            stats.len(),
            2,
            "One trasnfer should generate 2 stats for sender and receiver"
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
        assert_eq!(&sender_stat.max_balance, &0.0, "The sender balance was 0 before trasnfer, it ended up being -1 afterwards, hence the max balance is 0");

        // Assert Receiver
        assert_eq!(&receiver_stat.address, receiver);
        assert_eq!(&receiver_stat.max_balance, &1.0);

        Ok(())
    }

    #[test]
    fn price_accumulator_tracks_buy() {
        let mut accumulator = PriceAccumulator::default();

        accumulator.add(10.0, 20.0);

        assert_eq!(accumulator.buy_volume, 10.0);
        assert_eq!(accumulator.sell_volume, 0.0);
        assert_eq!(accumulator.weight_buy_amount, 200.0);
        assert_eq!(accumulator.weight_sell_amount, 0.0);
        assert_eq!(accumulator.max_balance, 10.0);
    }

    #[test]
    fn price_accumulator_tracks_sell() {
        let mut accumulator = PriceAccumulator {
            weight_sell_amount: 0.0,
            weight_buy_amount: 100.0,
            buy_volume: 10.0,
            sell_volume: 0.0,
            max_balance: 10.0,
        };

        accumulator.add(-20.0, 5.0);

        assert_eq!(accumulator.weight_sell_amount, 100.0);
        assert_eq!(accumulator.weight_buy_amount, 100.0);
        assert_eq!(accumulator.buy_volume, 10.0);
        assert_eq!(accumulator.sell_volume, 20.0);
        assert_eq!(accumulator.max_balance, 10.0);
    }

    #[test]
    fn accumulator_internals() {
        let accumulator = PriceAccumulator {
            weight_buy_amount: 100.0,
            buy_volume: 10.0,
            weight_sell_amount: 200.0,
            sell_volume: 25.0,
            max_balance: 33.0,
        };

        assert_eq!(10.0, accumulator.avg_buy_price());
        assert_eq!(8.0, accumulator.avg_sell_price());
        assert_eq!(35.0, accumulator.total_volume());
        assert_eq!(33.0, accumulator.max_balance());
    }
}
