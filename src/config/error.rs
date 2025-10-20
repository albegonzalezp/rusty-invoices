use thiserror::Error;

// Configuration errors with automatic error handling
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration parse error: {message}")]
    ParseError { message: String },
    
    #[error("Configuration validation error: {message}")]
    ValidationError { message: String },
    
    #[error("Configuration not found: {message}")]
    NotFound { message: String },
    
}
