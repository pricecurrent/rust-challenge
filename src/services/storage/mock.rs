use crate::model::Transfer;

use super::Storage;

pub struct MockStorage {
    transfers: Vec<Transfer>,
}

impl Storage for MockStorage {
    fn get(&self) -> &Vec<crate::model::Transfer> {
        &self.transfers
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
