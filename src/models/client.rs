use std::fmt::{self, Debug};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub cif: String,
    pub address: String,
    pub email: Option<String>,
}

impl Client {
    pub fn new(name: String, cif: String, address: String, email: Option<String>) -> Self {
        Client {
            name,
            cif,
            address,
            email,
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nCIF/NIF: {}\nAddress: {}", self.name, self.cif, self.address)?;
        if let Some(email) = &self.email {
            write!(f, "\nEmail: {}", email)?;
        }
        Ok(())
    }
}