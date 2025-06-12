use async_trait::async_trait;
use clickhouse::error::Result;

use crate::models::transfer::{Transfer, TransferOrdering};

use super::storage::Storage;

pub struct MockStorage {
    transfers: Vec<Transfer>,
}

#[async_trait]
impl Storage for MockStorage {
    async fn get_sorted(&self, transfer_ordering: TransferOrdering) -> Result<Vec<Transfer>> {
        let mut transfers = self.transfers.clone();

        match transfer_ordering {
            TransferOrdering::Raw => Ok(transfers),
            TransferOrdering::Chronological => {
                transfers.sort_unstable_by_key(|i| i.ts);
                Ok(transfers)
            }
            TransferOrdering::ByVolume => {
                transfers.sort_unstable_by_key(|i| i.amount as u64);
                Ok(transfers)
            }
        }
    }

    async fn insert_all(&mut self, transfers: &[Transfer]) -> Result<()> {
        self.transfers = transfers.to_vec();
        Ok(())
    }
}

impl Default for MockStorage {
    fn default() -> Self {
        MockStorage { transfers: vec![] }
    }
}
