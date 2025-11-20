mod cli;
mod commands;

use clap::Parser;
use cli::{Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    commands::dispatch(cli.command)?;
    Ok(())
}
