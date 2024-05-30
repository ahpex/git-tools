# git-fake-commit

## Description

Creates dummy commits on a git branch to try out and experiment with stuff.

Sometimes its easier to create your own commits to know whats going on, instead of using an unknown repository.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [License](#license)

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone this repository
2. Run `cargo build` in the root folder

## Usage

### How to Use

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

### Examples

- `git-fake-commit -m "(chore) committed file`
- `git-fake-commit -m "(chore) committed file -m "John Doe" -e "johndoe@email.com"`

## License

This project is licensed under the MIT License.
