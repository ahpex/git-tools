# git-tag-fork-point

Creates lightweight tags at branch fork points from the main branch.

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone the repository containing this tool
2. Run `cargo build` in the root folder of the workspace
3. The binary will be available at `target/debug/git-tag-fork-point`

## Usage

This tool creates a lightweight Git tag at the point where the current branch forked from the main branch. It finds the common ancestor (merge-base) between the current branch and the main branch, and tags that commit.

The tool must be run from within a Git repository, and the current branch must not be the main branch.

```none
Creates a lightweight tag on the branch point

Usage: git-tag-fork-point.exe [OPTIONS]

Options:
  -t, --tag <TAG>  Tag name to use
  -h, --help       Print help
  -V, --version    Print version
```

## Options

- `-t, --tag <TAG>`: Specify the tag name to use. If not provided, defaults to `fork/<branch_name>` where `<branch_name>` is the name of the current branch.
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

- Create a tag with the default name on the current branch:
  ```bash
  git-tag-fork-point
  ```
  This will create a tag like `fork/feature-branch` if you're on the `feature-branch`.

- Create a tag with a custom name:
  ```bash
  git-tag-fork-point --tag my-custom-tag
  ```

- Create a tag with a custom name using short option:
  ```bash
  git-tag-fork-point -t release/v1.0-fork
  ```

## Notes

- The tool assumes the main branch is named `main`. If your repository uses a different name (e.g., `master`), the tool will fail.
- Running this tool on the main branch will result in an error.
- The tag is created as a lightweight tag (no annotation).
