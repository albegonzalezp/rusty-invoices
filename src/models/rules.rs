use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub iva: f32,
    pub irpf: f32,
}

impl Rule {
    pub fn new(iva: f32, irpf: f32) -> Self {
        Rule { iva, irpf }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IVA: {}%, IRPF: {}%", self.iva, self.irpf)
    }
}
