use super::error::AppError;
use super::validation::{validate_cif, validate_email, validate_iban};
use crate::models::User;
use crate::repository::Storage;
use console::style;
use dialoguer::Input;

// Create a new user profile with validation
pub fn create_user(storage: &Storage) -> Result<User, AppError> {
    println!("{}", style("Let's set up your user profile").bold());

    let name: String = Input::new()
        .with_prompt("Enter your name")
        .interact_text()?;

    // Validate CIF/NIE with retry loop
    let cif: String = loop {
        let input: String = Input::new()
            .with_prompt("Enter your CIF/NIE")
            .interact_text()?;

        match validate_cif(&input) {
            Ok(()) => break input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                continue;
            }
        }
    };

    let address: String = Input::new()
        .with_prompt("Enter your address")
        .interact_text()?;

    let email: String = loop {
        let input: String = Input::new()
            .with_prompt("Enter your email (optional, press enter to skip)")
            .allow_empty(true)
            .interact_text()?;

        match validate_email(&input) {
            Ok(()) => break input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                continue;
            }
        }
    };

    let iban: String = loop {
        let input: String = Input::new()
            .with_prompt("Enter your IBAN (optional, press enter to skip)")
            .allow_empty(true)
            .interact_text()?;

        match validate_iban(&input) {
            Ok(()) => break input,
            Err(e) => {
                println!("{}", style(format!("Error: {}", e)).red());
                continue;
            }
        }
    };

    let user = User::new(
        name,
        address,
        cif,
        if email.is_empty() { None } else { Some(email) },
        if iban.is_empty() { None } else { Some(iban) },
    );

    storage.save_user(&user).map_err(AppError::from)?;
    println!("{}", style("User profile created successfully!").green());

    Ok(user)
}

pub fn update_user(storage: &Storage, user: &User) -> Result<User, AppError> {
    println!("{}", style("Update your user profile").bold());
    println!("Current profile:");
    println!("{}", user);

    // Check if user wants to continue
    if !dialoguer::Confirm::new()
        .with_prompt("Do you want to update your profile?")
        .default(true)
        .interact()?
    {
        return Ok(user.clone());
    }

    let name: String = Input::new()
        .with_prompt("Enter your name")
        .default(user.name.clone())
        .interact_text()?;

    let cif: String = Input::new()
        .with_prompt("Enter your CIF/NIE")
        .default(user.cif.clone())
        .interact_text()?;

    let address: String = Input::new()
        .with_prompt("Enter your address")
        .default(user.address.clone())
        .interact_text()?;

    let email: String = Input::new()
        .with_prompt("Enter your email (optional, press enter to skip)")
        .default(user.email.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;

    let iban: String = Input::new()
        .with_prompt("Enter your IBAN (optional, press enter to skip)")
        .default(user.iban.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;

    let updated_user = User::new(
        name,
        address,
        cif,
        if email.is_empty() { None } else { Some(email) },
        if iban.is_empty() { None } else { Some(iban) },
    );

    storage.save_user(&updated_user).map_err(AppError::from)?;
    println!("{}", style("User profile updated successfully!").green());

    Ok(updated_user)
}
