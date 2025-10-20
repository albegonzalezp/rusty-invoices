use crate::cli::error::{AppError, AppResult};
use crate::errors::ValidationError;

// Input validation functions

pub fn validate_email(email: &str) -> AppResult<()> {
    if email.is_empty() {
        return Ok(());
    }

    // Email format validation
    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::Validation(ValidationError::InvalidEmail {
            email: email.to_string(),
        }));
    }

    // Check that @ is not at the beginning or end
    if email.starts_with('@') || email.ends_with('@') {
        return Err(AppError::Validation(ValidationError::InvalidEmail {
            email: email.to_string(),
        }));
    }

    // Check that there's at least one character before @
    if let Some(at_pos) = email.find('@') {
        if at_pos == 0 {
            return Err(AppError::Validation(ValidationError::InvalidEmail {
                email: email.to_string(),
            }));
        }
    }

    Ok(())
}

pub fn validate_cif(cif: &str) -> AppResult<()> {
    if cif.is_empty() {
        return Err(AppError::Validation(ValidationError::RequiredFieldEmpty {
            field: "CIF/NIE".to_string(),
        }));
    }

    // CIF/NIE length check
    if cif.len() < 8 || cif.len() > 12 {
        return Err(AppError::Validation(ValidationError::InvalidCif {
            cif: cif.to_string(),
        }));
    }

    Ok(())
}

pub fn validate_date(date_str: &str) -> AppResult<()> {
    if date_str.is_empty() {
        return Ok(());
    }

    // Date format validation
    if chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").is_err() {
        return Err(AppError::Validation(ValidationError::InvalidDate {
            date: date_str.to_string(),
        }));
    }

    Ok(())
}

pub fn validate_iban(iban: &str) -> AppResult<()> {
    if iban.is_empty() {
        return Ok(());
    }

    // IBAN length check
    if iban.len() < 15 || iban.len() > 34 {
        return Err(AppError::Validation(ValidationError::InvalidIban {
            iban: iban.to_string(),
        }));
    }

    // IBAN alphanumeric check
    if !iban.chars().all(|c| c.is_alphanumeric()) {
        return Err(AppError::Validation(ValidationError::InvalidIban {
            iban: iban.to_string(),
        }));
    }

    Ok(())
}

pub fn validate_percentage(value: f32, _name: &str) -> AppResult<()> {
    if !(0.0..=100.0).contains(&value) {
        return Err(AppError::Validation(ValidationError::InvalidPercentage {
            value,
        }));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cif_valid() {
        assert!(validate_cif("12345678A").is_ok());
        assert!(validate_cif("87654321B").is_ok());
        assert!(validate_cif("123456789012").is_ok());
    }

    #[test]
    fn test_validate_cif_invalid() {
        assert!(validate_cif("").is_err());
        assert!(validate_cif("123").is_err());
        assert!(validate_cif("12345678901234567890").is_err());
    }

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.email+tag@domain.co.uk").is_ok());
        assert!(validate_email("").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user.domain.com").is_err());
    }

    #[test]
    fn test_validate_iban_valid() {
        assert!(validate_iban("ES1234567890123456789012").is_ok());
        assert!(validate_iban("GB82WEST12345698765432").is_ok());
        assert!(validate_iban("").is_ok());
    }

    #[test]
    fn test_validate_iban_invalid() {
        assert!(validate_iban("123").is_err());
        assert!(validate_iban("ES1234567890123456789012345678901234567890").is_err());
        assert!(validate_iban("ES12-3456-7890-1234-5678-9012").is_err());
    }

    #[test]
    fn test_validate_date_valid() {
        assert!(validate_date("2024-01-01").is_ok());
        assert!(validate_date("2023-12-31").is_ok());
        assert!(validate_date("").is_ok());
    }

    #[test]
    fn test_validate_date_invalid() {
        assert!(validate_date("01-01-2024").is_err());
        assert!(validate_date("2024/01/01").is_err());
        assert!(validate_date("2024-13-01").is_err());
        assert!(validate_date("2024-01-32").is_err());
    }

    #[test]
    fn test_validate_percentage_valid() {
        assert!(validate_percentage(0.0, "test").is_ok());
        assert!(validate_percentage(50.0, "test").is_ok());
        assert!(validate_percentage(100.0, "test").is_ok());
    }

    #[test]
    fn test_validate_percentage_invalid() {
        assert!(validate_percentage(-1.0, "test").is_err());
        assert!(validate_percentage(101.0, "test").is_err());
        assert!(validate_percentage(150.0, "test").is_err());
    }
}
