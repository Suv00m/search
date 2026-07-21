<p align="center">
  <img src="./assets/readme/hero.svg" width="100%" alt="srch: parallel regex search across files and directories, shown running against its own source">
</p>

# srch

A fast, grep-inspired search tool built in Rust. Regex or fixed-string matching across files and directories, parallelized across files with [Rayon](https://github.com/rayon-rs/rayon).

---

## Why srch

- **Regex or literal matching** — full regex by default, `-F` switches to a literal fixed-string search.
- **Recursive directory search** — point it at a directory and every file underneath gets searched.
- **Parallel across files** — each file is matched concurrently; results are collected and written once they're all ready.
- **Case-insensitive mode** — `-i` folds case for both regex and literal search.
- **Buffered output** — results are written through a single `BufWriter`, not one syscall per line.

---

## Installation

No pre-built binary is available yet. Build from source using Cargo:

```bash
cargo build --release
./target/release/srch <pattern> <path> [OPTIONS]
```

Or run directly without installing:

```bash
cargo run --release -- <pattern> <path> [OPTIONS]
```

---

## Usage

```
srch <PATTERN> <PATH> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `PATTERN` | The search pattern (regex by default) |
| `PATH` | File or directory to search |

### Options

| Flag | Long | Description |
|------|------|-------------|
| `-F` | `--fixed` | Treat pattern as a literal string (disables regex) |
| `-n` | `--line-number` | Prefix each match with its line number |
| `-i` | `--ignore-case` | Case-insensitive matching |

---

## Examples

**Plain string search:**
```bash
srch "hello" ./src/main.rs
```

**Search with line numbers:**
```bash
srch "fn main" ./src/main.rs -n
```

**Fixed string — disables regex, matches literally:**
```bash
srch "fn.main" ./src/main.rs -F
```

**Regex search:**
```bash
srch "fn\s+\w+" ./src/main.rs
```

**Regex with line numbers:**
```bash
srch "use \w+" ./src/main.rs -n
```

**Search across a directory:**
```bash
srch "TODO" ./src
```

**Case-insensitive search across a directory:**
```bash
srch "todo" ./src -i
```

---

## Status

Active development. Core search, regex support, recursive directory traversal, case-insensitive matching, parallel file search, and buffered output are implemented. Additional features are in progress.
