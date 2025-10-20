use super::container::ServiceContainer;
use crate::cli::{create_user, show_main_menu, AppResult};
use crate::config::AppConfig;
use crate::models::User;

// Main application class
pub struct App {
    services: ServiceContainer,
    user: User,
}

impl App {
    // Initialize the application
    pub fn new() -> AppResult<Self> {
        // Load configuration
        let config = AppConfig::load().map_err(crate::cli::AppError::Config)?;

        // Show welcome message from config
        println!(
            "{}",
            console::style(&config.ui.welcome_message).bold().green()
        );
        println!(
            "{}",
            console::style("The tool that helps you generate and manage your invoices").italic()
        );

        // Initialize service container with configuration
        let services = ServiceContainer::new(config)?;

        // Load or create user profile
        let user = match services
            .storage()
            .get_user()
            .map_err(crate::cli::AppError::from)?
        {
            Some(user) => user,
            None => create_user(services.storage())?,
        };

        Ok(App { services, user })
    }

    // Run the main application loop
    pub fn run(&mut self) -> AppResult<()> {
        loop {
            if show_main_menu(
                self.services.client_service(),
                self.services.invoice_service(),
                self.services.storage(),
                &mut self.user,
            )? {
                break;
            }
        }
        Ok(())
    }
}
