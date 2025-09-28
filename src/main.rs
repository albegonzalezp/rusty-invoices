// Rusty Invoices - A CLI tool for invoice management for self-employed professionals in Spain
// Main application entry point and CLI interface

mod models;
mod repository;
mod services;

use std::io;
use console::style;
use dialoguer::{Input, Select, Confirm};
use dirs;
use models::{Client, Item, Rule, User};
use repository::Storage;
use services::{ClientService, InvoiceService, PdfService};

// Application data directory name
const APP_DIR: &str = ".rusty-invoices";

// Custom error type to handle different error types
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    DialoguerError(dialoguer::Error),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<dialoguer::Error> for AppError {
    fn from(error: dialoguer::Error) -> Self {
        AppError::DialoguerError(error)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "IO error: {}", err),
            AppError::DialoguerError(err) => write!(f, "Dialog error: {}", err),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::IoError(err) => Some(err),
            AppError::DialoguerError(err) => Some(err),
        }
    }
}

// Application result type alias for convenience
type AppResult<T> = Result<T, AppError>;

// Main application entry point
fn main() -> AppResult<()> {
    // Welcome message
    println!("{}", style("Welcome to Rusty Invoices").bold().green());
    println!("{}", style("The tool that helps you generate and manage your invoices").italic());
    
    // Create app directory in user's home
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let app_dir = home_dir.join(APP_DIR);
    
    // Initialize services
    let storage = Storage::new(app_dir.to_str().unwrap());
    let pdf_service = PdfService::new(app_dir.join("pdfs").to_str().unwrap().to_string());
    
    let client_service = ClientService::new(storage.clone());
    let invoice_service = InvoiceService::new(storage.clone(), pdf_service);
    
    // Check if user exists, if not, create one
    let mut user = match storage.get_user().map_err(AppError::from)? {
        Some(user) => user,
        None => create_user(&storage)?,
    };
    
    // Main menu loop
    loop {
        let options = &[
            "Create invoice",
            "List invoices",
            "Create client",
            "List clients",
            "Update user profile",
            "Exit",
        ];
        
        let selection = Select::new()
            .with_prompt("Select an option")
            .items(options)
            .default(0)
            .interact()?;
        
        match selection {
            0 => create_invoice(&client_service, &invoice_service, &user)?,
            1 => list_invoices(&invoice_service)?,
            2 => create_client(&client_service)?,
            3 => list_clients(&client_service)?,
            4 => {
                user = update_user(&storage, &user)?;
            },
            5 => {
                println!("Thank you for using Rusty Invoices!");
                break;
            }
            _ => unreachable!(),
        }
    }
    
    Ok(())
}

// Create a new user profile
fn create_user(storage: &Storage) -> AppResult<User> {
    println!("{}", style("Let's set up your user profile").bold());
    
    let name: String = Input::new()
        .with_prompt("Enter your name")
        .interact_text()?;
    
    let cif: String = Input::new()
        .with_prompt("Enter your CIF/NIE")
        .interact_text()?;
    
    let address: String = Input::new()
        .with_prompt("Enter your address")
        .interact_text()?;
    
    let email: String = Input::new()
        .with_prompt("Enter your email (optional, press enter to skip)")
        .allow_empty(true)
        .interact_text()?;
    
    let iban: String = Input::new()
        .with_prompt("Enter your IBAN (optional, press enter to skip)")
        .allow_empty(true)
        .interact_text()?;
    
    let user = User::new(
        name,
        address,
        cif,
        if email.is_empty() { None } else { Some(email) },
        if iban.is_empty() { None } else { Some(iban) },
    );
    
    storage.save_user(&user).map_err(AppError::from)?;
    println!("{}", style("User profile created successfully!").green());
    
    Ok(user)
}

// Update an existing user profile
fn update_user(storage: &Storage, user: &User) -> AppResult<User> {
    println!("{}", style("Update your user profile").bold());
    println!("Current profile:");
    println!("{}", user);
    
    let name: String = Input::new()
        .with_prompt("Enter your name")
        .default(user.name.clone())
        .interact_text()?;
    
    let cif: String = Input::new()
        .with_prompt("Enter your CIF/NIE")
        .default(user.cif.clone())
        .interact_text()?;
    
    let address: String = Input::new()
        .with_prompt("Enter your address")
        .default(user.address.clone())
        .interact_text()?;
    
    let email: String = Input::new()
        .with_prompt("Enter your email (optional, press enter to skip)")
        .default(user.email.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    
    let iban: String = Input::new()
        .with_prompt("Enter your IBAN (optional, press enter to skip)")
        .default(user.iban.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    
    let updated_user = User::new(
        name,
        address,
        cif,
        if email.is_empty() { None } else { Some(email) },
        if iban.is_empty() { None } else { Some(iban) },
    );
    
    storage.save_user(&updated_user).map_err(AppError::from)?;
    println!("{}", style("User profile updated successfully!").green());
    
    Ok(updated_user)
}

// Create a new client
fn create_client(client_service: &ClientService) -> AppResult<()> {
    println!("{}", style("Create a new client").bold());
    
    let name: String = Input::new()
        .with_prompt("Enter client name")
        .interact_text()?;
    
    let cif: String = Input::new()
        .with_prompt("Enter client CIF")
        .interact_text()?;
    
    let address: String = Input::new()
        .with_prompt("Enter client address")
        .interact_text()?;
    
    let email: String = Input::new()
        .with_prompt("Enter client email (optional, press enter to skip)")
        .allow_empty(true)
        .interact_text()?;
    
    let client = client_service.create_client(
        name,
        cif,
        address,
        if email.is_empty() { None } else { Some(email) },
    ).map_err(AppError::from)?;
    
    println!("{}", style("Client created successfully!").green());
    println!("{}", client);
    
    Ok(())
}

// List all clients
fn list_clients(client_service: &ClientService) -> AppResult<()> {
    println!("{}", style("Clients").bold());
    
    let clients = client_service.list_clients().map_err(AppError::from)?;
    
    if clients.is_empty() {
        println!("No clients found.");
        return Ok(());
    }
    
    for (i, client) in clients.iter().enumerate() {
        println!("{}. {}", i + 1, style(&client.name).bold());
        println!("   CIF: {}", client.cif);
        println!("   Address: {}", client.address);
        if let Some(email) = &client.email {
            println!("   Email: {}", email);
        }
        println!();
    }
    
    Ok(())
}

// Select a client from the list of available clients
fn select_client(client_service: &ClientService) -> AppResult<Option<Client>> {
    let clients = client_service.list_clients().map_err(AppError::from)?;
    
    if clients.is_empty() {
        println!("No clients found. Please create a client first.");
        return Ok(None);
    }
    
    let client_names: Vec<String> = clients.iter().map(|c| c.name.clone()).collect();
    
    let selection = Select::new()
        .with_prompt("Select a client")
        .items(&client_names)
        .default(0)
        .interact()?;
    
    Ok(Some(clients[selection].clone()))
}

// Create a new invoice
fn create_invoice(client_service: &ClientService, invoice_service: &InvoiceService, user: &User) -> AppResult<()> {
    println!("{}", style("Create a new invoice").bold());
    
    // Select client
    let client = match select_client(client_service)? {
        Some(client) => client,
        None => return Ok(()),
    };
    
    // Custom invoice number
    let custom_invoice_number: String = Input::new()
        .with_prompt("Enter invoice number (leave empty for auto-generated)")
        .allow_empty(true)
        .interact_text()?;
    
    let invoice_number = if custom_invoice_number.is_empty() {
        None
    } else {
        Some(custom_invoice_number)
    };
    
    // Custom date
    let custom_date: String = Input::new()
        .with_prompt("Enter invoice date (YYYY-MM-DD, leave empty for today)")
        .allow_empty(true)
        .interact_text()?;
    
    let invoice_date = if custom_date.is_empty() {
        None
    } else {
        Some(custom_date)
    };
    
    // Custom due date
    let custom_due_date: String = Input::new()
        .with_prompt("Enter due date (YYYY-MM-DD, leave empty for 30 days from invoice date)")
        .allow_empty(true)
        .interact_text()?;
    
    let invoice_due_date = if custom_due_date.is_empty() {
        None
    } else {
        Some(custom_due_date)
    };
    
    // Set rules
    println!("{}", style("Set invoice rules").bold());
    
    let iva: f32 = Input::new()
        .with_prompt("Enter IVA percentage")
        .default(21.0)
        .interact_text()?;
    
    let irpf: f32 = Input::new()
        .with_prompt("Enter IRPF percentage")
        .default(15.0)
        .interact_text()?;
    
    let rule = Rule::new(iva, irpf);
    
    // Add items
    println!("{}", style("Add items to the invoice").bold());
    
    let mut items = Vec::new();
    loop {
        let description: String = Input::new()
            .with_prompt("Enter item description")
            .interact_text()?;
        
        let quantity: u32 = Input::new()
            .with_prompt("Enter quantity")
            .default(1)
            .interact_text()?;
        
        let price: f32 = Input::new()
            .with_prompt("Enter price per unit (€)")
            .interact_text()?;
        
        items.push(Item::new(description, quantity, price));
        
        if !Confirm::new()
            .with_prompt("Add another item?")
            .default(true)
            .interact()?
        {
            break;
        }
    }
    
    // Create invoice
    let invoice = invoice_service.create_invoice(
        invoice_number,
        invoice_date,
        invoice_due_date,
        user.clone(),
        client,
        rule,
        items
    ).map_err(AppError::from)?;
    
    // Display invoice
    println!("\n{}", style("Invoice created successfully!").green());
    println!("{}", invoice);
    
    // Generate PDF
    if Confirm::new()
        .with_prompt("Generate PDF?")
        .default(true)
        .interact()?
    {
        let pdf_path = invoice_service.generate_pdf(&invoice).map_err(AppError::from)?;
        println!("PDF generated: {}", pdf_path);
    }
    
    Ok(())
}

// List all invoices
fn list_invoices(invoice_service: &InvoiceService) -> AppResult<()> {
    println!("{}", style("Invoices").bold());
    
    let invoices = invoice_service.list_invoices().map_err(AppError::from)?;
    
    if invoices.is_empty() {
        println!("No invoices found.");
        return Ok(());
    }
    
    for (i, invoice) in invoices.iter().enumerate() {
        println!("{}. Invoice #{} - {}", i + 1, style(&invoice.id).bold(), invoice.date);
        println!("   Client: {}", invoice.client.name);
        println!("   Total: {:.2}€", invoice.total);
        println!();
    }
    
    // Option to view details
    if Confirm::new()
        .with_prompt("View invoice details?")
        .default(false)
        .interact()?
    {
        let invoice_ids: Vec<String> = invoices.iter().map(|i| i.id.clone()).collect();
        
        let selection = Select::new()
            .with_prompt("Select an invoice")
            .items(&invoice_ids)
            .default(0)
            .interact()?;
        
        println!("\n{}", invoices[selection]);
        
        // Option to generate PDF
        if Confirm::new()
            .with_prompt("Generate PDF?")
            .default(false)
            .interact()?
        {
            let pdf_path = invoice_service.generate_pdf(&invoices[selection]).map_err(AppError::from)?;
            println!("PDF generated: {}", pdf_path);
        }
    }
    
    Ok(())
}
