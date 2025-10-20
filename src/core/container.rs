use crate::cli::error::{AppError, AppResult};
use crate::config::AppConfig;
use crate::repository::Storage;
use crate::services::{ClientService, InvoiceService, PdfService};

// Service container
pub struct ServiceContainer {
    pub storage: Storage,
    pub client_service: ClientService,
    pub invoice_service: InvoiceService,
}

impl ServiceContainer {
    // Create service container
    pub fn new(config: AppConfig) -> AppResult<Self> {
        // Initialize storage using config
        let storage_path =
            config
                .storage
                .base_path
                .to_str()
                .ok_or_else(|| AppError::InvalidInput {
                    message: "Invalid characters in storage path".to_string(),
                })?;

        let storage = Storage::new(storage_path).map_err(AppError::Io)?;

        // Set up PDF service using config
        let pdfs_dir_str =
            config
                .pdf
                .output_dir
                .to_str()
                .ok_or_else(|| AppError::InvalidInput {
                    message: "Invalid characters in PDF directory path".to_string(),
                })?;

        let pdf_service = PdfService::new(pdfs_dir_str.to_string()).map_err(AppError::Io)?;

        // Initialize services with dependencies
        let client_service = ClientService::new(storage.clone());
        let invoice_service = InvoiceService::new(storage.clone(), pdf_service);

        Ok(ServiceContainer {
            storage,
            client_service,
            invoice_service,
        })
    }

    // Get storage reference
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    // Get client service reference
    pub fn client_service(&self) -> &ClientService {
        &self.client_service
    }

    // Get invoice service reference
    pub fn invoice_service(&self) -> &InvoiceService {
        &self.invoice_service
    }
}
