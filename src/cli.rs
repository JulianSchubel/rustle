use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "Rustle: A lightweight streaming ETL tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ///  Boostrap the SQLite database
    Bootstrap {
        db_path: String,

        /* Drop tables before recreating them */
        #[arg(long)]
        drop: bool,

        /* Skip drop confirmation */
        #[arg(long)]
        force: bool,
    },

    ///  Run the ETL process on a CSV or NDJSON file
    Run {
        /* Input file - CSV or NDJSON */
        input: String,

        /* Output SQLite database */   
        db_path: String,
        
        /* Number of worker threads */
        #[arg(short, long, default_value = "4")]
        threads: usize,
    },  

    /// Generate a sample dataset
    GenerateDataset {
        /* Output file - .csv or .ndjson */
        output: String,

        /* Number of records to generate */
        #[arg(short, long, default_value = "100000")]
        records: usize,
    }
}
