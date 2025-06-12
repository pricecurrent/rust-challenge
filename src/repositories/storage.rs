use async_trait::async_trait;
use clickhouse::error::Result;

use crate::models::transfer::{Transfer, TransferOrdering};

#[async_trait]
pub trait Storage {
    async fn get_sorted(&self, transfer_ordering: TransferOrdering) -> Result<Vec<Transfer>>;
    async fn insert_all(&mut self, transfers: &[Transfer]) -> Result<()>;
}
