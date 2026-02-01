[Back to main README](../README.md)

# git-fake-commit

Creates dummy commits on a git branch to try out and experiment with stuff.

Sometimes it's easier to create your own commits to know what's going on, instead of using an unknown repository.

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone the repository containing this tool
2. Run `cargo build` in the root folder of the workspace
3. The binary will be available at `target/debug/git-fake-commit`

## Usage

This tool creates dummy commits in the current branch of a git repository for testing and experimentation purposes.

```none
Creates dummy commits in the current branch of a git repository

Usage: git-fake-commit [OPTIONS]

Options:
  -m <MESSAGE>      Commit message
  -a <AUTHOR>       Commit author
  -e <EMAIL>        Commit email
  -h, --help        Print help
  -V, --version     Print version
```

## Options

- `-m <MESSAGE>`: The commit message
- `-a <AUTHOR>`: The commit author name
- `-e <EMAIL>`: The commit author email
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

- Create a commit with a simple message:
  ```bash
  git-fake-commit -m "(chore) committed file"
  ```

- Create a commit with custom author:
  ```bash
  git-fake-commit -m "(chore) committed file" -a "John Doe" -e "johndoe@email.com"
  ```