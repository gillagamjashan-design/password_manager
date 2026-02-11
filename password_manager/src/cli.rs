use clap::{Parser, Subcommand};
use colored::*;
use rpassword::read_password;
use std::io::{self, Write};

use crate::crypto::generate_password;
use crate::errors::{PasswordManagerError, Result};
use crate::models::Credential;
use crate::vault::VaultManager;

#[derive(Parser)]
#[command(name = "password_manager")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "A secure CLI password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new password vault
    Init,

    /// Add a new credential
    Add {
        /// Service name (e.g., github.com)
        #[arg(short, long)]
        service: Option<String>,

        /// Username
        #[arg(short, long)]
        username: Option<String>,

        /// Generate a random password
        #[arg(short = 'g', long)]
        generate: bool,

        /// Password length for generation (default: 24)
        #[arg(short = 'l', long, default_value = "24")]
        length: usize,
    },

    /// Get a credential (copies password to clipboard)
    Get {
        /// Service name
        service: String,

        /// Show password in terminal instead of copying
        #[arg(short, long)]
        show: bool,
    },

    /// List all stored credentials
    List,

    /// Search credentials by service or username
    Search {
        /// Search query
        query: String,
    },

    /// Update a credential's password
    Update {
        /// Service name
        service: String,

        /// Generate a random password
        #[arg(short = 'g', long)]
        generate: bool,

        /// Password length for generation (default: 24)
        #[arg(short = 'l', long, default_value = "24")]
        length: usize,
    },

    /// Remove a credential
    Remove {
        /// Service name
        service: String,
    },

    /// Generate a secure random password
    Generate {
        /// Password length (default: 24)
        #[arg(short, long, default_value = "24")]
        length: usize,

        /// Exclude uppercase letters
        #[arg(long)]
        no_uppercase: bool,

        /// Exclude lowercase letters
        #[arg(long)]
        no_lowercase: bool,

        /// Exclude numbers
        #[arg(long)]
        no_numbers: bool,

        /// Exclude symbols
        #[arg(long)]
        no_symbols: bool,
    },
}

/// Prompt for master password
pub fn prompt_master_password(prompt: &str) -> Result<String> {
    print!("{}", prompt.bright_cyan());
    io::stdout().flush()?;

    let password = read_password().map_err(|e| {
        PasswordManagerError::InvalidInput(format!("Failed to read password: {}", e))
    })?;

    if password.is_empty() {
        return Err(PasswordManagerError::InvalidInput(
            "Password cannot be empty".to_string(),
        ));
    }

    Ok(password)
}

/// Prompt for input with a message
pub fn prompt_input(prompt: &str) -> Result<String> {
    print!("{}", prompt.bright_cyan());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// Handle the init command
pub fn handle_init(vault_manager: &mut VaultManager) -> Result<()> {
    if vault_manager.vault_exists() {
        println!(
            "{}",
            "Vault already exists. Use other commands to manage it.".yellow()
        );
        return Ok(());
    }

    println!("{}", "Initializing new password vault...".bright_green());
    println!();

    let password = prompt_master_password("Enter master password: ")?;
    let confirm = prompt_master_password("Confirm master password: ")?;

    if password != confirm {
        return Err(PasswordManagerError::InvalidInput(
            "Passwords do not match".to_string(),
        ));
    }

    vault_manager.initialize(&password)?;

    println!();
    println!("{}", "✓ Vault initialized successfully!".bright_green());
    println!("{}", "Your vault is encrypted and stored securely.".green());

    Ok(())
}

/// Handle the add command
pub fn handle_add(
    vault_manager: &mut VaultManager,
    service: Option<String>,
    username: Option<String>,
    generate: bool,
    length: usize,
) -> Result<()> {
    unlock_vault(vault_manager)?;

    // Prompt for service if not provided
    let service = match service {
        Some(s) => s,
        None => prompt_input("Service name (e.g., github.com): ")?,
    };

    if service.is_empty() {
        return Err(PasswordManagerError::InvalidInput(
            "Service name cannot be empty".to_string(),
        ));
    }

    // Prompt for username if not provided
    let username = match username {
        Some(u) => u,
        None => prompt_input("Username: ")?,
    };

    // Get or generate password
    let password = if generate {
        let pwd = generate_password(length, true, true, true, true);
        println!(
            "{} {}",
            "Generated password:".bright_green(),
            pwd.bright_white()
        );
        pwd
    } else {
        prompt_master_password("Password: ")?
    };

    // Optional notes
    let notes_input = prompt_input("Notes (optional, press Enter to skip): ")?;
    let notes = if notes_input.is_empty() {
        None
    } else {
        Some(notes_input)
    };

    let credential = Credential::new(service.clone(), username, password, notes);
    vault_manager.add_credential(credential)?;

    println!();
    println!(
        "{} {}",
        "✓ Credential added for".bright_green(),
        service.bright_white()
    );

    Ok(())
}

/// Handle the get command
pub fn handle_get(vault_manager: &VaultManager, service: &str, show: bool) -> Result<()> {
    let credential = vault_manager.get_credential(service)?;

    if show {
        println!();
        println!("{}: {}", "Service".bright_cyan(), credential.service);
        println!("{}: {}", "Username".bright_cyan(), credential.username);
        println!("{}: {}", "Password".bright_cyan(), credential.password);
        if let Some(notes) = &credential.notes {
            println!("{}: {}", "Notes".bright_cyan(), notes);
        }
        println!(
            "{}: {}",
            "Created".bright_cyan(),
            credential.created_at.format("%Y-%m-%d %H:%M:%S")
        );
        println!(
            "{}: {}",
            "Updated".bright_cyan(),
            credential.updated_at.format("%Y-%m-%d %H:%M:%S")
        );
    } else {
        // Copy to clipboard
        match cli_clipboard::set_contents(credential.password.clone()) {
            Ok(_) => {
                println!(
                    "{} {} {}",
                    "✓ Password for".bright_green(),
                    service.bright_white(),
                    "copied to clipboard".bright_green()
                );
            }
            Err(e) => {
                return Err(PasswordManagerError::ClipboardError(e.to_string()));
            }
        }
    }

    Ok(())
}

/// Handle the list command
pub fn handle_list(vault_manager: &VaultManager) -> Result<()> {
    let credentials = vault_manager.list_all()?;

    if credentials.is_empty() {
        println!("{}", "No credentials stored yet.".yellow());
        return Ok(());
    }

    println!();
    println!("{}", "Stored Credentials:".bright_cyan().bold());
    println!("{}", "─".repeat(80).bright_black());

    for cred in credentials {
        println!(
            "  {} {} ({})",
            "•".bright_green(),
            cred.service.bright_white().bold(),
            cred.username.bright_black()
        );
    }

    println!("{}", "─".repeat(80).bright_black());
    println!("{} credentials found", credentials.len());

    Ok(())
}

/// Handle the search command
pub fn handle_search(vault_manager: &VaultManager, query: &str) -> Result<()> {
    let results = vault_manager.search(query)?;

    if results.is_empty() {
        println!("{} '{}'", "No credentials found matching".yellow(), query);
        return Ok(());
    }

    println!();
    println!(
        "{} '{}':",
        "Search results for".bright_cyan().bold(),
        query.bright_white()
    );
    println!("{}", "─".repeat(80).bright_black());

    for cred in results {
        println!(
            "  {} {} ({})",
            "•".bright_green(),
            cred.service.bright_white().bold(),
            cred.username.bright_black()
        );
    }

    println!("{}", "─".repeat(80).bright_black());

    Ok(())
}

/// Handle the update command
pub fn handle_update(
    vault_manager: &mut VaultManager,
    service: &str,
    generate: bool,
    length: usize,
) -> Result<()> {
    // Verify credential exists
    let _ = vault_manager.get_credential(service)?;

    let new_password = if generate {
        let pwd = generate_password(length, true, true, true, true);
        println!(
            "{} {}",
            "Generated password:".bright_green(),
            pwd.bright_white()
        );
        pwd
    } else {
        prompt_master_password("New password: ")?
    };

    vault_manager.update_credential(service, new_password)?;

    println!();
    println!(
        "{} {}",
        "✓ Password updated for".bright_green(),
        service.bright_white()
    );

    Ok(())
}

/// Handle the remove command
pub fn handle_remove(vault_manager: &mut VaultManager, service: &str) -> Result<()> {
    // Verify credential exists
    let _ = vault_manager.get_credential(service)?;

    let confirmation = prompt_input(&format!(
        "Are you sure you want to remove '{}'? (yes/no): ",
        service
    ))?;

    if confirmation.to_lowercase() != "yes" {
        println!("{}", "Cancelled.".yellow());
        return Ok(());
    }

    vault_manager.remove_credential(service)?;

    println!();
    println!(
        "{} {}",
        "✓ Credential removed for".bright_green(),
        service.bright_white()
    );

    Ok(())
}

/// Handle the generate command
pub fn handle_generate(
    length: usize,
    no_uppercase: bool,
    no_lowercase: bool,
    no_numbers: bool,
    no_symbols: bool,
) -> Result<()> {
    let password = generate_password(
        length,
        !no_uppercase,
        !no_lowercase,
        !no_numbers,
        !no_symbols,
    );

    println!();
    println!("{}", "Generated Password:".bright_cyan().bold());
    println!("{}", "─".repeat(80).bright_black());
    println!("  {}", password.bright_white().bold());
    println!("{}", "─".repeat(80).bright_black());
    println!("Length: {} characters", length);

    // Copy to clipboard
    match cli_clipboard::set_contents(password) {
        Ok(_) => {
            println!("{}", "✓ Password copied to clipboard".bright_green());
        }
        Err(e) => {
            println!("{} {}", "Warning: Could not copy to clipboard:".yellow(), e);
        }
    }

    Ok(())
}

/// Unlock vault by prompting for master password
fn unlock_vault(vault_manager: &mut VaultManager) -> Result<()> {
    if !vault_manager.vault_exists() {
        return Err(PasswordManagerError::VaultNotFound);
    }

    let password = prompt_master_password("Master password: ")?;
    vault_manager.unlock(&password)?;

    Ok(())
}
