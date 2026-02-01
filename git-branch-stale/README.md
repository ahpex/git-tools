[Back to main README](../README.md)

# git-branch-stale

Lists git branches sorted by how long ago they had their last commit.

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone the repository containing this tool
2. Run `cargo build` in the root folder of the workspace
3. The binary will be available at `target/debug/git-branch-stale`

## Usage

This tool analyzes all branches in the repository and displays them sorted by the number of days since their last commit, with the most stale branches first.

```none
Shows stale git branches without commits for n days

Usage: git-branch-stale.exe [OPTIONS] [REPO]

Arguments:
  [REPO]  Path to repository [default: .]

Options:
  -v      Be verbose
  -r      Show only remote branches
  -h, --help     Print help
  -V, --version  Print version
```

## Options

- `[REPO]`: Path to the git repository (default: current directory)
- `-v, --verbose`: Show additional details including author and commit SHA
- `-r, --remote`: Show only remote branches
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

- List all branches sorted by staleness:
  ```bash
  git-branch-stale
  ```

- Show verbose output with author and SHA:
  ```bash
  git-branch-stale -v
  ```

- Check remote branches only:
  ```bash
  git-branch-stale -r
  ```

- Analyze a specific repository:
  ```bash
  git-branch-stale /path/to/repo
  ```