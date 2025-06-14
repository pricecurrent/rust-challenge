use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    models::transfer::Transfer,
    utils::time::{Now, SystemNow},
};

pub trait TransferGenerator {
    fn generate(&self, count: usize) -> Result<Vec<Transfer>>;
}

#[derive(Debug, Clone)]
pub struct TransferGenConfig {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub max_age_secs: u64,
}

impl Default for TransferGenConfig {
    fn default() -> Self {
        Self {
            min_amount: 1.0,
            max_amount: 1000.0,
            min_price: 0.1,
            max_price: 2.0,
            max_age_secs: 86_400 * 30,
        }
    }
}

pub struct DefaultTransferGenerator {
    pub config: TransferGenConfig,
}

impl Default for DefaultTransferGenerator {
    fn default() -> Self {
        Self {
            config: TransferGenConfig::default(),
        }
    }
}

impl TransferGenerator for DefaultTransferGenerator {
    fn generate(&self, count: usize) -> anyhow::Result<Vec<Transfer>> {
        let mut rng = rand::thread_rng();
        let now = SystemNow::now_unix()?;

        let ts = (0..count)
            .map(|_| {
                let from = rand_address(&mut rng);
                let to = rand_address(&mut rng);
                let amount = rng.gen_range(self.config.min_amount..=self.config.max_amount);
                let usd_price = rng.gen_range(self.config.min_price..=self.config.max_price);
                let ts = now - rng.gen_range(0..=self.config.max_age_secs);

                Transfer {
                    ts,
                    from,
                    to,
                    amount,
                    usd_price,
                }
            })
            .collect();

        Ok(ts)
    }
}

fn rand_address(rng: &mut impl Rng) -> String {
    let suffix: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("0x{}", suffix)
}
