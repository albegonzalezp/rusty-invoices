// Rusty Invoices - Invoice management CLI

mod cli;
mod config;
mod core;
mod errors;
mod models;
mod repository;
mod services;

use cli::AppResult;
use core::App;
fn main() {
    // Catch any unexpected panics and show user friendly messages
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("{}", console::style("Unexpected Error").bold().red());
        eprintln!("The application encountered an unexpected error:");
        eprintln!("{}", panic_info);
        eprintln!(
            "\n{}",
            console::style("This is likely a bug. Please report this issue.").italic()
        );
    }));

    // Run the application and handle any errors gracefully
    if let Err(e) = run_app() {
        eprintln!("{}", console::style("Application Error").bold().red());
        eprintln!("{}", e);
        eprintln!(
            "\n{}",
            console::style("Please check the error above and try again.").italic()
        );
        std::process::exit(1);
    }
}

fn run_app() -> AppResult<()> {
    // Create and initialize the application with dependency injection
    let mut app = App::new()?;

    // Run the main application loop
    app.run()
}
