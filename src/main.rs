mod search;

use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
    /// the regex flag
    #[arg(short = 'F', long)]
    fixed: bool,
    /// the return number of line instead of line
    #[arg(short = 'n', long)]
    line_number: bool,
    /// flag for case insensitive search
    #[arg(short = 'i', long)]
    ignore_case: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let paths: Vec<_> = search::dispatch_file_path(&args.path).collect();
    let results: Vec<_> = paths
        .par_iter()
        .map(|path| {
            search::find_match(&args.pattern, path, args.fixed, args.ignore_case)
                .unwrap_or_default()
        })
        .collect();
    for matches in results {
        for (idx, line) in matches {
            if args.line_number {
                writeln!(out, "{}.{}", idx + 1, line)?;
            } else {
                writeln!(out, "{}", line)?;
            }
        }
    }
    Ok(())
}
