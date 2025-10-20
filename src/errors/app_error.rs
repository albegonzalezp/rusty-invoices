use thiserror::Error;
use crate::errors::ValidationError;
use crate::config::error::ConfigError;

// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("User interface error: {0}")]
    Dialoguer(#[from] dialoguer::Error),
    
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
}

// Result type alias for cleaner code
pub type AppResult<T> = Result<T, AppError>;

