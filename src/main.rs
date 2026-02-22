mod search;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    search::find_match(&args.pattern, &args.path)
        .context("could not find match")?;
    Ok(())
}
