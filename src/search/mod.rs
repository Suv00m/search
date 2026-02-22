use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::path::Path;
use anyhow::{Context, Result};

pub fn find_match(pattern: &str, path: &Path) -> Result<()> {
    let file = File::open(path)
        .with_context(|| format!("could not open file '{}'", path.display()))?;
    let content_reader = BufReader::new(file);
    
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for line in content_reader.lines() {
        let line = line.context("could not read line")?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

