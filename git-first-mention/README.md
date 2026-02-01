[Back to main README](../README.md)

# git-first-mention

Searches for the first mention of a string in the repository.

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone the repository containing this tool
2. Run `cargo build` in the root folder of the workspace
3. The binary will be available at `target/debug/git-first-mention`

## Usage

This tool searches through the git history to find the first commit where a given string appears.

```none
Find commits where a string first appears

Usage: git-first-mention.exe -s <NEEDLE>

Options:
  -s <NEEDLE>      search for string
  -h, --help       Print help
  -V, --version    Print version
```

## Options

- `-s <NEEDLE>`: The string to search for
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

- Search for the first mention of a function name:
  ```bash
  git-first-mention -s "myFunction"
  ```

- Search for a specific error message:
  ```bash
  git-first-mention -s "NullPointerException"
  ```