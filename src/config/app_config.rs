use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::error::ConfigError;

// Application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // Storage configuration
    pub storage: StorageConfig,
    
    // PDF configuration
    pub pdf: PdfConfig,
    
    // Tax configuration
    pub tax: TaxConfig,
    
    // UI configuration
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_path: PathBuf,
    pub clients_dir: String,
    pub invoices_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfConfig {
    pub output_dir: PathBuf,
    pub font_size: f32,
    pub page_width: f32,
    pub page_height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxConfig {
    pub default_iva: f32,
    pub default_irpf: f32,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub welcome_message: String,
    pub confirm_prompts: bool,
    pub show_debug_info: bool,
}

impl AppConfig {
    // Load configuration from file, environment variables, or defaults
    pub fn load() -> Result<Self, ConfigError> {
        // Try to load from config file first
        if let Ok(config) = Self::load_from_file() {
            return Ok(config);
        }
        
        // Fall back to environment variables
        if let Ok(config) = Self::load_from_env() {
            return Ok(config);
        }
        
        // Use defaults as last resort
        Ok(Self::default())
    }
    
    // Load configuration from file
    fn load_from_file() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            return Err(ConfigError::NotFound {
                message: format!("Configuration file not found at: {}", config_path.display())
            });
        }
        
        let config_content = std::fs::read_to_string(&config_path)?;
        let config: AppConfig = serde_json::from_str(&config_content)
            .map_err(|e| ConfigError::ParseError { 
                message: format!("Invalid JSON: {}", e) 
            })?;
        
        // Validate the loaded configuration
        config.validate()?;
        Ok(config)
    }
    
    // Load configuration from environment variables
    fn load_from_env() -> Result<Self, ConfigError> {
        let mut config = Self::default();
        
        // Override with environment variables if they exist
        if let Ok(storage_path) = std::env::var("RUSTY_INVOICES_STORAGE_PATH") {
            config.storage.base_path = PathBuf::from(storage_path);
        }
        
        if let Ok(pdf_dir) = std::env::var("RUSTY_INVOICES_PDF_DIR") {
            config.pdf.output_dir = PathBuf::from(pdf_dir);
        }
        
        if let Ok(iva) = std::env::var("RUSTY_INVOICES_DEFAULT_IVA") {
            config.tax.default_iva = iva.parse()
                .map_err(|e| ConfigError::ParseError { 
                    message: format!("Invalid IVA value: {}", e) 
                })?;
        }
        
        if let Ok(irpf) = std::env::var("RUSTY_INVOICES_DEFAULT_IRPF") {
            config.tax.default_irpf = irpf.parse()
                .map_err(|e| ConfigError::ParseError { 
                    message: format!("Invalid IRPF value: {}", e) 
                })?;
        }
        
        config.validate()?;
        Ok(config)
    }
    
    // Get the configuration file path
    fn get_config_path() -> Result<PathBuf, ConfigError> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| ConfigError::NotFound { 
                message: "Home directory not found".to_string() 
            })?;
        
        Ok(home_dir.join(".rusty-invoices").join("config.json"))
    }
    
    
    // Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate tax percentages
        if self.tax.default_iva < 0.0 || self.tax.default_iva > 100.0 {
            return Err(ConfigError::ValidationError {
                message: format!("IVA percentage must be between 0 and 100, got: {}", self.tax.default_iva)
            });
        }
        
        if self.tax.default_irpf < 0.0 || self.tax.default_irpf > 100.0 {
            return Err(ConfigError::ValidationError {
                message: format!("IRPF percentage must be between 0 and 100, got: {}", self.tax.default_irpf)
            });
        }
        
        // Validate PDF dimensions
        if self.pdf.page_width <= 0.0 || self.pdf.page_height <= 0.0 {
            return Err(ConfigError::ValidationError {
                message: "PDF page dimensions must be positive".to_string()
            });
        }
        
        if self.pdf.font_size <= 0.0 {
            return Err(ConfigError::ValidationError {
                message: "Font size must be positive".to_string()
            });
        }
        
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        
        AppConfig {
            storage: StorageConfig {
                base_path: home_dir.join(".rusty-invoices"),
                clients_dir: "clients".to_string(),
                invoices_dir: "invoices".to_string(),
            },
            pdf: PdfConfig {
                output_dir: home_dir.join(".rusty-invoices").join("pdfs"),
                font_size: 12.0,
                page_width: 210.0,  // A4 width in mm
                page_height: 297.0, // A4 height in mm
            },
            tax: TaxConfig {
                default_iva: 21.0,
                default_irpf: 15.0,
                currency: "EUR".to_string(),
            },
            ui: UiConfig {
                welcome_message: "Welcome to Rusty Invoices".to_string(),
                confirm_prompts: true,
                show_debug_info: false,
            },
        }
    }
}
