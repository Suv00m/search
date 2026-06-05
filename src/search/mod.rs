use anyhow::{Context, Result};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn find_match(pattern: &str, path: &Path, fixed: bool, is_line_num_flag: bool) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("could not open file '{}'", path.display()))?;
    let content_reader = BufReader::new(file);

    enum SearchMode {
        Plain(String),
        Regex(Regex),
    }
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mode = if !fixed {
        SearchMode::Regex(Regex::new(pattern)?)
    } else {
        SearchMode::Plain(pattern.to_string())
    };

    for (idx, line) in content_reader.lines().enumerate() {
        let line = line.context("could not read line")?;
        let is_match = match &mode {
            SearchMode::Plain(text) => line.contains(text.as_str()),
            SearchMode::Regex(re) => re.is_match(&line),
        };
        if is_match {
            if is_line_num_flag {
                writeln!(writer, "{}.{}", idx + 1, line)?;
            } else {
                writeln!(writer, "{}", line)?;
            }
        }
    }

    Ok(())
}
