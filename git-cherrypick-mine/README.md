[Back to main README](../README.md)

# git-cherrypick-mine

Lists commits authored by the current user on the current branch that are not yet merged to main, and provides a cherry-pick command.

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone the repository containing this tool
2. Run `cargo build` in the root folder of the workspace
3. The binary will be available at `target/debug/git-cherrypick-mine`

## Usage

This tool finds all commits on the current branch (since it forked from main) that were authored by the current Git user, and displays them along with a command to cherry-pick them to the main branch.

Bot commits (by "bamboo" or "dailybuild") are excluded.

```none
Cherry-picks commits by the current user from the current branch

Usage: git-cherrypick-mine.exe [OPTIONS]

Options:
  -r, --repo <REPO>  Path to repository [default: .]
  -h, --help         Print help
  -V, --version      Print version
```

## Options

- `-r, --repo <REPO>`: Path to the git repository (default: current directory)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

- List your commits on the current branch:
  ```bash
  git-cherrypick-mine
  ```

- Analyze a specific repository:
  ```bash
  git-cherrypick-mine --repo /path/to/repo
  ```

## Output

The tool will output something like:

```
Would cherry-pick 2 commit(s) from branch feature-branch:
  abc1234 Initial commit message
  def5678 Another commit message
git checkout main && git cherry-pick -x abc1234 def5678
```

You can then copy and run the provided cherry-pick command.