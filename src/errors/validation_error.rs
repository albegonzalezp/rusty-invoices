use thiserror::Error;

// Validation specific error types
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Email format is invalid: {email}")]
    InvalidEmail { email: String },

    #[error("CIF/NIE format is invalid: {cif}")]
    InvalidCif { cif: String },

    #[error("IBAN format is invalid: {iban}")]
    InvalidIban { iban: String },

    #[error("Date format is invalid: {date}")]
    InvalidDate { date: String },

    #[error("Percentage value is invalid: {value} (must be 0-100)")]
    InvalidPercentage { value: f32 },

    #[error("Required field is empty: {field}")]
    RequiredFieldEmpty { field: String },
}
