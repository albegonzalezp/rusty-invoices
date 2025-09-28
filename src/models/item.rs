use std::fmt::{self, Debug};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub description: String,
    pub quantity: u32,
    pub price: f32,
}

impl Item {
    pub fn new(description: String, quantity: u32, price: f32) -> Self {
        Item {
            description,
            quantity,
            price,
        }
    }

    pub fn total(&self) -> f32 {
        self.quantity as f32 * self.price
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} x {:.2}€ = {:.2}€",
            self.description,
            self.quantity,
            self.price,
            self.total()
        )
    }
}