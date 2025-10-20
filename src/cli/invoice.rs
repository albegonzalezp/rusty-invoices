use super::client::select_client;
use super::error::AppError;
use super::validation::{validate_date, validate_percentage};
use crate::models::{Item, Rule, User};
use crate::services::{ClientService, InvoiceService};
use console::style;
use dialoguer::{Confirm, Input, Select};

// Create a new invoice with validation and PDF generation
pub fn create_invoice(
    client_service: &ClientService,
    invoice_service: &InvoiceService,
    user: &User,
) -> Result<(), AppError> {
    println!("{}", style("Create a new invoice").bold());

    // Check if user wants to continue
    if !dialoguer::Confirm::new()
        .with_prompt("Do you want to create a new invoice?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }

    let client = match select_client(client_service)? {
        Some(client) => client,
        None => return Ok(()),
    };
    let custom_invoice_number: String = Input::new()
        .with_prompt("Enter invoice number (leave empty for auto-generated)")
        .allow_empty(true)
        .interact_text()?;

    let invoice_number = if custom_invoice_number.is_empty() {
        None
    } else {
        Some(custom_invoice_number)
    };
    let custom_date: String = loop {
        let input: String = Input::new()
            .with_prompt("Enter invoice date (YYYY-MM-DD, leave empty for today)")
            .allow_empty(true)
            .interact_text()?;

        match validate_date(&input) {
            Ok(()) => break input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                continue;
            }
        }
    };

    let invoice_date = if custom_date.is_empty() {
        None
    } else {
        Some(custom_date)
    };
    let custom_due_date: String = loop {
        let input: String = Input::new()
            .with_prompt("Enter due date (YYYY-MM-DD, leave empty for 30 days from invoice date)")
            .allow_empty(true)
            .interact_text()?;

        match validate_date(&input) {
            Ok(()) => break input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                continue;
            }
        }
    };

    let invoice_due_date = if custom_due_date.is_empty() {
        None
    } else {
        Some(custom_due_date)
    };
    println!("{}", style("Set invoice rules").bold());

    let iva: f32 = loop {
        let input: f32 = Input::new()
            .with_prompt("Enter IVA percentage")
            .default(21.0)
            .interact_text()?;

        match validate_percentage(input, "IVA percentage") {
            Ok(()) => break input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                continue;
            }
        }
    };

    let irpf: f32 = loop {
        let input: f32 = Input::new()
            .with_prompt("Enter IRPF percentage")
            .default(15.0)
            .interact_text()?;

        match validate_percentage(input, "IRPF percentage") {
            Ok(()) => break input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                continue;
            }
        }
    };

    let rule = Rule::new(iva, irpf);
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
    let invoice = invoice_service
        .create_invoice(
            invoice_number,
            invoice_date,
            invoice_due_date,
            user.clone(),
            client,
            rule,
            items,
        )
        .map_err(AppError::from)?;
    println!("\n{}", style("Invoice created successfully!").green());
    println!("{}", invoice);
    if Confirm::new()
        .with_prompt("Generate PDF?")
        .default(true)
        .interact()?
    {
        let pdf_path = invoice_service
            .generate_pdf(&invoice)
            .map_err(AppError::from)?;
        println!("PDF generated: {}", pdf_path);
    }

    Ok(())
}

pub fn list_invoices(invoice_service: &InvoiceService) -> Result<(), AppError> {
    println!("{}", style("Invoices").bold());

    // Check if user wants to continue
    if !dialoguer::Confirm::new()
        .with_prompt("Do you want to view invoices?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }

    let invoices = invoice_service.list_invoices().map_err(AppError::from)?;

    if invoices.is_empty() {
        println!("No invoices found.");
        return Ok(());
    }

    for (i, invoice) in invoices.iter().enumerate() {
        println!(
            "{}. Invoice #{} - {}",
            i + 1,
            style(&invoice.id).bold(),
            invoice.date
        );
        println!("   Client: {}", invoice.client.name);
        println!("   Total: {:.2}€", invoice.total);
        println!();
    }
    if Confirm::new()
        .with_prompt("View invoice details?")
        .default(false)
        .interact()?
    {
        // Add "Go Back" option to invoice selection
        let mut invoice_options: Vec<String> = invoices
            .iter()
            .map(|i| format!("Invoice #{} - {}", i.id, i.date))
            .collect();
        invoice_options.push("← Go Back".to_string());

        let selection = Select::new()
            .with_prompt("Select an invoice")
            .items(&invoice_options)
            .default(0)
            .interact()?;

        // Check if user selected "Go Back"
        if selection == invoice_options.len() - 1 {
            return Ok(());
        }

        println!("\n{}", invoices[selection]);
        if Confirm::new()
            .with_prompt("Generate PDF?")
            .default(false)
            .interact()?
        {
            let pdf_path = invoice_service
                .generate_pdf(&invoices[selection])
                .map_err(AppError::from)?;
            println!("PDF generated: {}", pdf_path);
        }
    }

    Ok(())
}
