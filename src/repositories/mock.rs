use crate::models::transfer::{Transfer, TransferOrdering};

use super::storage::Storage;

pub struct MockStorage {
    transfers: Vec<Transfer>,
}

impl Storage for MockStorage {
    fn get(&self) -> Vec<Transfer> {
        self.transfers.clone()
    }

    fn get_sorted(&self, transfer_ordering: TransferOrdering) -> Vec<Transfer> {
        match transfer_ordering {
            TransferOrdering::Raw => self.get(),
            TransferOrdering::Chronological => {
                let mut transfers = self.get();
                transfers.sort_unstable_by_key(|i| i.ts);
                transfers
            }
            TransferOrdering::ByVolume => {
                let mut transfers = self.get();
                transfers.sort_unstable_by_key(|i| i.amount as u64);
                transfers
            }
        }
    }

    fn insert_all(&mut self, transfers: Vec<Transfer>) {
        self.transfers = transfers
    }
}

impl Default for MockStorage {
    fn default() -> Self {
        MockStorage { transfers: vec![] }
    }
}
