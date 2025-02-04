# git-fake-commit Tools

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone this repository
2. Run `cargo build` in the root folder

## Provided Tools

## git-branch-stale

Shows a list of stale git branches.

### git-fake-commit

#### Description

Creates dummy commits on a git branch to try out and experiment with stuff.

Sometimes its easier to create your own commits to know whats going on, instead of using an unknown repository.

#### Usage

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

**Examples:**

- `git-fake-commit -m "(chore) committed file`
- `git-fake-commit -m "(chore) committed file -m "John Doe" -e "johndoe@email.com"`

### git-tag-fork-point

#### Description

Creates a lightweight tag for the branch point where a branch has been forked from _main_ branch.

#### Usage

```none
Creates a lightweight tag on the branch point

Usage: git-tag-fork-point

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## License

This project is licensed under the MIT License.
