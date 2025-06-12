use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Row)]
pub struct Transfer {
    pub ts: u64,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub usd_price: f64,
}

#[derive(PartialEq, Eq)]
pub enum TransferOrdering {
    Raw,
    Chronological,
    ByVolume,
}
