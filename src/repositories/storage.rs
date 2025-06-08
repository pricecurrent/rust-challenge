use crate::models::transfer::{Transfer, TransferOrdering};

// think about return impl Iterator in get methods
pub trait Storage {
    fn get(&self) -> Vec<Transfer>;
    fn get_sorted(&self, transfer_ordering: TransferOrdering) -> Vec<Transfer>;
    fn insert_all(&mut self, transfers: Vec<Transfer>);
}
