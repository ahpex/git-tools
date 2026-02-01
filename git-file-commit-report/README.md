[Back to main README](../README.md)

# git-file-commit-report

Lists all commits from a starting commit to an ending commit (inclusive) and shows the modified files within those commits.

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone the repository containing this tool
2. Run `cargo build` in the root folder of the workspace
3. The binary will be available at `target/debug/git-file-commit-report`

## Usage

This tool generates a report of commits between two commit references, showing which files were modified in each commit.

```none
Lists all commits from a commit to another commit (inclusive) and shows modified files

Usage: git-file-commit-report [OPTIONS] <FROM_REF> <TO_REF> [PATHS]...

Arguments:
  <FROM_REF>  Starting commit (older commit) - can be a SHA, reference like HEAD, or HEAD~4
  <TO_REF>    Ending commit (newer commit, inclusive) - can be a SHA, reference like HEAD, or HEAD~4
  [PATHS]...  None, one or multiple paths to include

Options:
  -r, --repo <REPO>      Path to repository [default: .]
  -v, --verbose          Show files for each commit
  -h, --help             Print help
  -V, --version          Print version
```

## Options

- `<FROM_REF>`: Starting commit reference (the older commit) - can be a SHA, reference like HEAD, branch name, or relative reference like HEAD~4
- `<TO_REF>`: Ending commit reference (the newer commit, inclusive) - can be a SHA, reference like HEAD, branch name, or relative reference like HEAD~4
- `[PATHS]...`: Optional paths to include in the report
- `-r, --repo <REPO>`: Path to the git repository (default: current directory)
- `-v, --verbose`: Show files modified in each commit
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

- Generate a basic commit report using SHAs:
  ```bash
  git-file-commit-report abc123 def456
  ```

- Use HEAD references:
  ```bash
  git-file-commit-report HEAD~10 HEAD
  ```

- Use branch names and relative references:
  ```bash
  git-file-commit-report main feature-branch
  git-file-commit-report HEAD~4 HEAD~1
  ```

- Show files for each commit (verbose mode):
  ```bash
  git-file-commit-report abc123 def456 -v
  ```

- Include only specific paths:
  ```bash
  git-file-commit-report abc123 def456 modules/testprograms modules/platform
  ```

- Analyze a specific repository:
  ```bash
  git-file-commit-report abc123 def456 --repo /path/to/repo
  ```