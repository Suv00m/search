mod search;

use anyhow::Result;
use clap::Parser;
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
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    search::dispatch_file_path(&args.path)
        .flat_map(|path| search::find_match(&args.pattern, &path, args.fixed).unwrap_or_default())
        .try_for_each(|(idx, line)| {
            if args.line_number {
                writeln!(out, "{}.{}", idx + 1, line)
            } else {
                writeln!(out, "{}", line)
            }
        })?;
    Ok(())
}
