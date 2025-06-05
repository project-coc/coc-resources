use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use coc_resources::parse_file;

#[derive(Parser)]
#[command(name = "coc-resources", version, about = "CLI for coc-resources", propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a file and output resources as JSON
    Parse { file: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Parse { file } => {
            let resources = parse_file(&file)?;
            println!("{}", serde_json::to_string_pretty(&resources)?);
        }
    }
    Ok(())
}
