use crate::models::{Client, Invoice, Item, Rule, User};
use crate::repository::storage::Storage;
use crate::services::pdf::PdfService;
use std::io;
use chrono::{Local, NaiveDate};
use uuid::Uuid;

pub struct InvoiceService {
    storage: Storage,
    pdf_service: PdfService,
}

impl InvoiceService {
    pub fn new(storage: Storage, pdf_service: PdfService) -> Self {
        InvoiceService {
            storage,
            pdf_service,
        }
    }

    pub fn create_invoice(
        &self,
        invoice_number: Option<String>,
        date: Option<String>,
        due_date: Option<String>,
        user: User,
        client: Client,
        rule: Rule,
        items: Vec<Item>,
    ) -> io::Result<Invoice> {
        // Generate invoice ID if not provided
        let id = invoice_number.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        // Use current date if not provided
        let today = Local::now().format("%Y-%m-%d").to_string();
        let invoice_date = date.unwrap_or_else(|| today.clone());
        
        // Default due date is 30 days after invoice date
        let invoice_due_date = match due_date {
            Some(due) => due,
            None => {
                // Try to parse the invoice date
                if let Ok(parsed_date) = NaiveDate::parse_from_str(&invoice_date, "%Y-%m-%d") {
                    // Add 30 days
                    let due = parsed_date.checked_add_days(chrono::Days::new(30))
                        .unwrap_or_else(|| parsed_date);
                    due.format("%Y-%m-%d").to_string()
                } else {
                    // If parsing fails, use today + 30 days
                    let today_parsed = Local::now().naive_local().date();
                    let due = today_parsed.checked_add_days(chrono::Days::new(30))
                        .unwrap_or_else(|| today_parsed);
                    due.format("%Y-%m-%d").to_string()
                }
            }
        };
        
        let invoice = Invoice::new(id, invoice_date, invoice_due_date, user, client, rule, items);
        self.storage.save_invoice(&invoice)?;
        
        Ok(invoice)
    }

    pub fn list_invoices(&self) -> io::Result<Vec<Invoice>> {
        self.storage.list_invoices()
    }

    pub fn generate_pdf(&self, invoice: &Invoice) -> io::Result<String> {
        self.pdf_service.generate_invoice_pdf(invoice)
    }
} 