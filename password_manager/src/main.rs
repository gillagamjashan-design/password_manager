use clap::Parser;
use colored::*;
use std::process;

mod cli;
mod crypto;
mod errors;
mod models;
mod vault;

use cli::{
    handle_add, handle_generate, handle_get, handle_init, handle_list, handle_remove,
    handle_search, handle_update, Cli, Commands,
};
use vault::VaultManager;

fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Get vault path
    let vault_path = match VaultManager::default_vault_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{} {}", "Error:".bright_red(), e);
            process::exit(1);
        }
    };

    // Create vault manager
    let mut vault_manager = VaultManager::new(vault_path);

    // Execute command
    let result = match cli.command {
        Commands::Init => handle_init(&mut vault_manager),

        Commands::Add {
            service,
            username,
            generate,
            length,
        } => handle_add(&mut vault_manager, service, username, generate, length),

        Commands::Get { service, show } => {
            if let Err(e) = cli::prompt_master_password("Master password: ")
                .and_then(|password| vault_manager.unlock(&password))
            {
                eprintln!("{} {}", "Error:".bright_red(), e);
                process::exit(1);
            }
            handle_get(&vault_manager, &service, show)
        }

        Commands::List => {
            if let Err(e) = cli::prompt_master_password("Master password: ")
                .and_then(|password| vault_manager.unlock(&password))
            {
                eprintln!("{} {}", "Error:".bright_red(), e);
                process::exit(1);
            }
            handle_list(&vault_manager)
        }

        Commands::Search { query } => {
            if let Err(e) = cli::prompt_master_password("Master password: ")
                .and_then(|password| vault_manager.unlock(&password))
            {
                eprintln!("{} {}", "Error:".bright_red(), e);
                process::exit(1);
            }
            handle_search(&vault_manager, &query)
        }

        Commands::Update {
            service,
            generate,
            length,
        } => {
            if let Err(e) = cli::prompt_master_password("Master password: ")
                .and_then(|password| vault_manager.unlock(&password))
            {
                eprintln!("{} {}", "Error:".bright_red(), e);
                process::exit(1);
            }
            handle_update(&mut vault_manager, &service, generate, length)
        }

        Commands::Remove { service } => {
            if let Err(e) = cli::prompt_master_password("Master password: ")
                .and_then(|password| vault_manager.unlock(&password))
            {
                eprintln!("{} {}", "Error:".bright_red(), e);
                process::exit(1);
            }
            handle_remove(&mut vault_manager, &service)
        }

        Commands::Generate {
            length,
            no_uppercase,
            no_lowercase,
            no_numbers,
            no_symbols,
        } => handle_generate(length, no_uppercase, no_lowercase, no_numbers, no_symbols),
    };

    // Handle errors
    if let Err(e) = result {
        eprintln!("{} {}", "Error:".bright_red(), e);
        process::exit(1);
    }
}
