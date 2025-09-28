use std::fmt::{self, Debug};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub address: String,
    pub cif: String,
    pub email: Option<String>,
    pub iban: Option<String>,
}

impl User {
    pub fn new(name: String, address: String, cif: String, email: Option<String>, iban: Option<String>) -> Self {
        User {
            name,
            address,
            cif,
            email,
            iban,
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nCIF/NIE: {}\nAddress: {}", self.name, self.cif, self.address)?;
        if let Some(email) = &self.email {
            write!(f, "\nEmail: {}", email)?;
        }
        if let Some(iban) = &self.iban {
            write!(f, "\nIBAN: {}", iban)?;
        }
        Ok(())
    }
}