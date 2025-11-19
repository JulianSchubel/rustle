mod cli;
mod commands;

use ::rustle::*;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    extract::init();
    transform::init();
    load::init();

    match cli.command {
        Commands::Bootstrap { db_path, drop, force } => {
            commands::bootstrap::bootstrap(&db_path, drop, force)?;
        },
        Commands::GenerateDataset { output, records } => {
            commands::generate::generate_dataset(&output, records)?;
        },
        _ => {
            println!("Unknown command");
        }
    }

    Ok(())
}
