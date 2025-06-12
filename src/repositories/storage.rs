use anyhow::Result;
use async_trait::async_trait;

use crate::models::transfer::{Transfer, TransferOrdering};

#[async_trait]
pub trait Storage {
    async fn get_sorted(&self, transfer_ordering: TransferOrdering) -> Result<Vec<Transfer>>;
    async fn insert_all(&mut self, transfers: &[Transfer]) -> Result<()>;
}

#[async_trait]
impl<T: Storage + Send + Sync> RetrievesTransfersChronologically for T {
    async fn get_chronologically(&self) -> Result<Vec<Transfer>> {
        self.get_sorted(TransferOrdering::Chronological).await
    }
}

#[async_trait]
pub trait RetrievesTransfersChronologically {
    async fn get_chronologically(&self) -> Result<Vec<Transfer>>;
}
