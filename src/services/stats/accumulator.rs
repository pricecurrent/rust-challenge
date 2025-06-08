#[derive(Default, Debug)]
pub struct PriceAccumulator {
    pub weight_sell_amount: f64, // total usd payed when selling
    pub weight_buy_amount: f64,  // total usd payed when buying
    pub buy_volume: f64,
    pub sell_volume: f64,
    pub max_balance: f64,
    pub balance: f64,
}

impl PriceAccumulator {
    pub fn accumulate(&mut self, amount: f64, usd_price: f64) {
        self.balance += amount;

        if amount > 0.0 {
            self.weight_buy_amount += amount * usd_price;
            self.buy_volume += amount;
        } else if amount < 0.0 {
            self.weight_sell_amount += amount.abs() * usd_price;
            self.sell_volume += amount.abs();
        }

        if self.balance > self.max_balance {
            self.max_balance = self.balance;
        }
    }

    pub fn avg_buy_price(&self) -> f64 {
        if self.buy_volume == 0.0 {
            return 0.0;
        }
        self.weight_buy_amount / self.buy_volume
    }

    pub fn avg_sell_price(&self) -> f64 {
        if self.sell_volume == 0.0 {
            return 0.0;
        }
        self.weight_sell_amount / self.sell_volume
    }

    pub fn total_volume(&self) -> f64 {
        self.sell_volume + self.buy_volume
    }

    pub fn max_balance(&self) -> f64 {
        self.max_balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_accumulator_tracks_buy() {
        let mut accumulator = PriceAccumulator::default();

        accumulator.accumulate(10.0, 20.0);

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
            balance: 10.0,
        };

        accumulator.accumulate(-20.0, 5.0);

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
            balance: 10.0,
        };

        assert_eq!(10.0, accumulator.avg_buy_price());
        assert_eq!(8.0, accumulator.avg_sell_price());
        assert_eq!(35.0, accumulator.total_volume());
        assert_eq!(33.0, accumulator.max_balance());
    }
}
