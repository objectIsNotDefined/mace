mod cli;
mod config;

use clap::Parser;
use std::path::Path;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let config_path = Path::new(".mace.toml");

    match &cli.command {
        cli::Commands::Init => {
            match config::generate_default_config(config_path) {
                Ok(_) => println!("✅ Successfully generated default configuration at {:?}", config_path),
                Err(e) => eprintln!("❌ Failed to initialize config: {}", e),
            }
        }
        cli::Commands::Run { prompt, roles, models: _ } => {
            let config = config::load_config(config_path)?;
            println!("Configuration loaded. Roles defined: {:?}", config.roles.keys());
            println!("Task: {}", prompt);
            if let Some(r) = roles {
                println!("Using roles: {}", r);
            }
            // TODO: Execute task
        }
        cli::Commands::Audit { strategy } => {
            println!("Audit mode using strategy: {}", strategy);
            // TODO: Execute audit
        }
        cli::Commands::Resume { task_id } => {
            println!("Resuming task: {}", task_id);
            // TODO: Execute resume
        }
        cli::Commands::Tools | cli::Commands::Doctor => {
            println!("Checking available AI tools...");
            // TODO: Implement tool diagnostics
        }
    }

    Ok(())
}
