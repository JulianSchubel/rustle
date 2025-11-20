use crate::cli::{Commands};
use anyhow::{Result};

pub fn dispatch(command: Commands) -> Result<()>{
    match command {
        Commands::Bootstrap { db_path, drop, force } => {
            super::bootstrap::bootstrap(&db_path, drop, force)?;
        },
        Commands::Generate { output, records, format } => {
            super::generate::generate(&output, records, &format)?;
        },
        Commands::Run { input, db_path, threads, buffer, csv_headers} => {
            super::run::run(&input, &db_path, threads, buffer, csv_headers)?;
        },
    }

    Ok(())
}
