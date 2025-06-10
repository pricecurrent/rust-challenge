use super::storage::Storage;

pub struct ClickhouseStorage<Client> {
    client: Client,
}

impl<Client> ClickhouseStorage<Client> {
    pub fn new(client: Client) -> ClickhouseStorage<Client> {
        ClickhouseStorage { client }
    }
}

impl<C> Storage for ClickhouseStorage<C> {
    fn get(&self) -> Vec<crate::models::transfer::Transfer> {
        todo!()
    }

    fn get_sorted(
        &self,
        transfer_ordering: crate::models::transfer::TransferOrdering,
    ) -> Vec<crate::models::transfer::Transfer> {
        todo!()
    }

    fn insert_all(&mut self, transfers: Vec<crate::models::transfer::Transfer>) {
        todo!()
    }
}
