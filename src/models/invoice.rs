// Invoice model representing a complete invoice with all its components
// Contains user (issuer), client, tax rules, items, and calculated totals

use super::client::Client;
use super::item::Item;
use super::rules::Rule;
use super::user::User;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

// Invoice struct containing all invoice data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,       // Unique invoice identifier
    pub date: String,     // Invoice issue date (YYYY-MM-DD)
    pub due_date: String, // Payment due date (YYYY-MM-DD)
    pub user: User,       // Invoice issuer details
    pub client: Client,   // Client details
    pub rule: Rule,       // Tax rules (IVA, IRPF)
    pub items: Vec<Item>, // Line items on the invoice
    pub subtotal: f32,    // Sum of all items before taxes
    pub iva_amount: f32,  // IVA tax amount
    pub irpf_amount: f32, // IRPF tax amount
    pub total: f32,       // Final amount after taxes
}

impl Invoice {
    // Create a new invoice with automatic calculation of totals
    pub fn new(
        id: String,
        date: String,
        due_date: String,
        user: User,
        client: Client,
        rule: Rule,
        items: Vec<Item>,
    ) -> Self {
        // Calculate subtotal from all items
        let subtotal = items.iter().map(|item| item.total()).sum();

        // Calculate tax amounts
        let iva_amount = subtotal * (rule.iva / 100.0);
        let irpf_amount = subtotal * (rule.irpf / 100.0);

        // Calculate total (subtotal + IVA - IRPF)
        let total = subtotal + iva_amount - irpf_amount;

        Invoice {
            id,
            date,
            due_date,
            user,
            client,
            rule,
            items,
            subtotal,
            iva_amount,
            irpf_amount,
            total,
        }
    }
}

// Display implementation for console output
impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "INVOICE #{} - {}", self.id, self.date)?;
        writeln!(f, "Due Date: {}", self.due_date)?;

        writeln!(f, "\nISSUER:")?;
        writeln!(f, "{}", self.user)?;

        writeln!(f, "\nCLIENT:")?;
        writeln!(f, "{}", self.client)?;

        writeln!(f, "\nITEMS:")?;
        for (i, item) in self.items.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, item)?;
        }

        writeln!(f, "\nSUMMARY:")?;
        writeln!(f, "Subtotal: {:.2}€", self.subtotal)?;
        writeln!(f, "IVA ({}%): {:.2}€", self.rule.iva, self.iva_amount)?;
        writeln!(f, "IRPF ({}%): -{:.2}€", self.rule.irpf, self.irpf_amount)?;
        writeln!(f, "TOTAL: {:.2}€", self.total)?;

        Ok(())
    }
}
