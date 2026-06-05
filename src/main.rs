mod search;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    /// the regex flag
    #[arg(short = 'F', long)]
    fixed: bool,
    /// the return number of line instead of line
    #[arg(short = 'n', long)]
    line_number: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    search::find_match(&args.pattern, &args.path, args.fixed, args.line_number)
        .context("could not find match")?;
    Ok(())
}
