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
        #[arg(short('t'), long, default_value = "4")]
        threads: usize,

        /* Buffer size for bounded channels */
        #[arg(short('b'), long, default_value = "10000")]
        buffer_size: usize,

        /* CSV headers */
        #[arg(long, default_value_t = true)]
        csv_headers: bool,

        /* Batch size */
        #[arg(long, default_value = "1000")]
        batch_size: usize,
    },  

    /// Generate a sample dataset
    Generate {
        /* Output file */
        output: String,

        /* Number of records to generate */
        #[arg(short('n'), long, default_value = "100000")]
        records: usize,

        /* output format */ 
        #[arg(short('f'), long, default_value = "csv")]
        format: String,
    }
}
