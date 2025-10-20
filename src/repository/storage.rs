use crate::models::{Client, Invoice, User};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

// File-based storage
#[derive(Clone)]
pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new(base_path: &str) -> io::Result<Self> {
        let path = PathBuf::from(base_path);

        // Create main storage directory
        fs::create_dir_all(&path).map_err(|e| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Failed to create storage directory '{}': {}", base_path, e),
            )
        })?;

        // Create subdirectories for organized storage
        let clients_dir = path.join("clients");
        let invoices_dir = path.join("invoices");
        fs::create_dir_all(&clients_dir).map_err(|e| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Failed to create clients directory: {}", e),
            )
        })?;
        fs::create_dir_all(&invoices_dir).map_err(|e| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Failed to create invoices directory: {}", e),
            )
        })?;

        Ok(Storage { base_path: path })
    }

    fn ensure_directory_exists(&self, dir_name: &str) -> io::Result<PathBuf> {
        let dir_path = self.base_path.join(dir_name);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }
        Ok(dir_path)
    }

    // Client storage methods
    pub fn save_client(&self, client: &Client) -> io::Result<()> {
        let clients_dir = self.ensure_directory_exists("clients")?;
        let filename = format!("{}.json", client.cif);
        let file_path = clients_dir.join(filename);

        let json = serde_json::to_string_pretty(&client)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn list_clients(&self) -> io::Result<Vec<Client>> {
        let clients_dir = self.ensure_directory_exists("clients")?;
        let mut clients = Vec::new();

        for entry in fs::read_dir(clients_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                let mut file = File::open(path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                match serde_json::from_str::<Client>(&contents) {
                    Ok(client) => clients.push(client),
                    Err(_) => continue, // Skip invalid files
                }
            }
        }

        Ok(clients)
    }

    // Invoice storage methods
    pub fn save_invoice(&self, invoice: &Invoice) -> io::Result<()> {
        let invoices_dir = self.ensure_directory_exists("invoices")?;
        let filename = format!("{}.json", invoice.id);
        let file_path = invoices_dir.join(filename);

        let json = serde_json::to_string_pretty(&invoice)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn list_invoices(&self) -> io::Result<Vec<Invoice>> {
        let invoices_dir = self.ensure_directory_exists("invoices")?;
        let mut invoices = Vec::new();

        for entry in fs::read_dir(invoices_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                let mut file = File::open(path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                match serde_json::from_str::<Invoice>(&contents) {
                    Ok(invoice) => invoices.push(invoice),
                    Err(_) => continue, // Skip invalid files
                }
            }
        }

        Ok(invoices)
    }

    // User storage methods
    pub fn save_user(&self, user: &User) -> io::Result<()> {
        let user_file = self.base_path.join("user.json");
        let json = serde_json::to_string_pretty(&user)?;
        let mut file = File::create(user_file)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn get_user(&self) -> io::Result<Option<User>> {
        let user_file = self.base_path.join("user.json");

        if !user_file.exists() {
            return Ok(None);
        }

        let mut file = File::open(user_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let user: User = serde_json::from_str(&contents)?;
        Ok(Some(user))
    }
}
