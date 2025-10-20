use crate::models::Client;
use crate::repository::storage::Storage;
use std::io;

pub struct ClientService {
    storage: Storage,
}

impl ClientService {
    pub fn new(storage: Storage) -> Self {
        ClientService { storage }
    }

    pub fn create_client(
        &self,
        name: String,
        cif: String,
        address: String,
        email: Option<String>,
    ) -> io::Result<Client> {
        let client = Client::new(name, cif, address, email);
        self.storage.save_client(&client)?;
        Ok(client)
    }

    pub fn list_clients(&self) -> io::Result<Vec<Client>> {
        self.storage.list_clients()
    }
}
