use crate::model::Transfer;

pub mod mock;

pub trait Storage {
    fn get(&self) -> &Vec<Transfer>;
    fn insert_all(&mut self, transfers: Vec<Transfer>);
}
