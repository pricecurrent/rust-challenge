use crate::{
    models::{transfer::TransferOrdering, user_stats::UserStats},
    repositories::storage::Storage,
};

use super::stats::pipeline::calculate_user_stats;

pub struct Analytics {
    storage: Box<dyn Storage>,
}

impl Analytics {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Analytics { storage }
    }

    pub fn get_stats(&self) -> Vec<UserStats> {
        calculate_user_stats(&self.storage.get_sorted(TransferOrdering::Chronological))
    }
}
