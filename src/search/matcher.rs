use anyhow::{Context, Result};
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn find_match(
    pattern: &str,
    path: &Path,
    fixed: bool,
    ignore_case: bool,
) -> Result<Vec<(usize, String)>> {
    let file =
        File::open(path).with_context(|| format!("could not open file '{}'", path.display()))?;
    let content_reader = BufReader::new(file);
    let mut matches = vec![];
    enum SearchMode {
        Plain(String),
        Regex(Regex),
    }
    let mode = if !fixed {
        let re = RegexBuilder::new(pattern)
            .case_insensitive(ignore_case) // automatically handles requirements
            .build();
        SearchMode::Regex(re?)
    } else if ignore_case {
        SearchMode::Plain(pattern.to_lowercase())
    } else {
        SearchMode::Plain(pattern.to_string())
    };

    for (i, line) in content_reader.lines().enumerate() {
        let line = line.context("could not read line")?;
        let is_match = match &mode {
            SearchMode::Plain(text) => {
                if ignore_case {
                    line.to_lowercase().contains(text.as_str())
                } else {
                    line.contains(text.as_str())
                }
            }
            SearchMode::Regex(re) => re.is_match(&line),
        };
        if is_match {
            matches.push((i, line));
        }
    }

    Ok(matches)
}
