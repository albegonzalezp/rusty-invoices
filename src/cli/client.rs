use console::style;
use dialoguer::{Input, Select};
use crate::models::Client;
use crate::services::ClientService;
use super::error::AppError;
use super::validation::{validate_cif, validate_email};

pub fn create_client(client_service: &ClientService) -> Result<(), AppError> {
    println!("{}", style("Create a new client").bold());
    
    // Check if user wants to continue
    if !dialoguer::Confirm::new()
        .with_prompt("Do you want to create a new client?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }
    
    let name: String = Input::new()
        .with_prompt("Enter client name")
        .interact_text()?;
    
    // Validate CIF with retry loop
    let cif = loop {
        let cif_input: String = Input::new()
            .with_prompt("Enter client CIF/NIF")
            .interact_text()?;
        
        match validate_cif(&cif_input) {
            Ok(_) => break cif_input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                println!("{}", style("CIF/NIF must be between 8-12 characters").yellow());
                continue;
            }
        }
    };
    
    let address: String = Input::new()
        .with_prompt("Enter client address")
        .interact_text()?;
    
    // Validate email with retry loop
    let email = loop {
        let email_input: String = Input::new()
            .with_prompt("Enter client email (optional, press enter to skip)")
            .allow_empty(true)
            .interact_text()?;
        
        if email_input.is_empty() {
            break None;
        }
        
        match validate_email(&email_input) {
            Ok(_) => break Some(email_input),
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                println!("{}", style("Please enter a valid email format (user@domain.com)").yellow());
                continue;
            }
        }
    };
    
    let client = client_service.create_client(
        name,
        cif,
        address,
        email,
    ).map_err(AppError::from)?;
    
    println!("{}", style("Client created successfully!").green());
    println!("{}", client);
    
    Ok(())
}

pub fn list_clients(client_service: &ClientService) -> Result<(), AppError> {
    println!("{}", style("Clients").bold());
    
    // Check if user wants to continue
    if !dialoguer::Confirm::new()
        .with_prompt("Do you want to view clients?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }
    
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

pub fn select_client(client_service: &ClientService) -> Result<Option<Client>, AppError> {
    let clients = client_service.list_clients().map_err(AppError::from)?;
    
    if clients.is_empty() {
        println!("No clients found. Please create a client first.");
        return Ok(None);
    }
    
    // Add "Go Back" option to the client list
    let mut client_names: Vec<String> = clients.iter().map(|c| c.name.clone()).collect();
    client_names.push("← Go Back".to_string());
    
    let selection = Select::new()
        .with_prompt("Select a client")
        .items(&client_names)
        .default(0)
        .interact()?;
    
    // Check if user selected "Go Back"
    if selection == client_names.len() - 1 {
        return Ok(None);
    }
    
    Ok(Some(clients[selection].clone()))
}
