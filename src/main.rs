mod commands;
mod pacman;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump installed packages
    Dump {
        /// File to dump installed packages
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dump { file } => {
            crate::commands::Dump::exec(file)?;
        }
    }

    Ok(())
}
