use dialoguer::Select;
use crate::models::User;
use crate::services::{ClientService, InvoiceService};
use crate::repository::Storage;
use super::error::AppError;
use super::user::update_user;
use super::client::{create_client, list_clients};
use super::invoice::{create_invoice, list_invoices};

// Main menu options
const MENU_OPTIONS: &[&str] = &[
    "Create invoice",
    "List invoices", 
    "Create client",
    "List clients",
    "Update user profile",
    "Exit",
];


pub fn show_main_menu(
    client_service: &ClientService,
    invoice_service: &InvoiceService,
    storage: &Storage,
    user: &mut User,
) -> Result<bool, AppError> {
    let selection = Select::new()
        .with_prompt("Select an option")
        .items(MENU_OPTIONS)
        .default(0)
        .interact()?;
    
    // Handle menu selection
    match selection {
        0 => create_invoice(client_service, invoice_service, user)?,
        1 => list_invoices(invoice_service)?,
        2 => create_client(client_service)?,
        3 => list_clients(client_service)?,
        4 => {
            *user = update_user(storage, user)?;
        },
        5 => {
            println!("Thank you for using Rusty Invoices!");
            return Ok(true); // Exit application
        }
        _ => unreachable!(),
    }
    
    Ok(false) // Continue application loop
}
