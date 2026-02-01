[Back to main README](../README.md)

# git-merge-driver-packagejson

A Git merge driver for `package.json` files that intelligently merges properties from both branches while preserving the `version` field from your current branch.

**Merge Behavior:**
- The `version` property always stays as it is in your branch (ours)
- All other properties are merged using three-way merge logic
- Changes from both branches are incorporated
- New properties from either side are added to the result

This is useful when merging branches where both sides may have added or modified dependencies, but you want to keep control over the version number in your branch.

## Installation

### Prerequisites

[rustup](https://rustup.rs) needs to be installed.

### Steps

1. Clone the repository containing this tool
2. Run `cargo build` in the root folder of the workspace
3. The binary will be available at `target/debug/git-merge-driver-packagejson`

## Usage

```none
Git merge driver for package.json files that intelligently merges dependencies

Usage: git-merge-driver-packagejson <BASE> <CURRENT> <OTHER> <MARKER_SIZE> <PATHNAME>

Arguments:
  <BASE>         Path to the base version (common ancestor) - %O
  <CURRENT>      Path to the current version (ours) - %A
  <OTHER>        Path to the other version (theirs) - %B
  <MARKER_SIZE>  Conflict marker size - %L
  <PATHNAME>     Pathname where merged result will be stored - %P

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Setup

To configure this merge driver for your repository, add the following to your `.git/config` or `.gitconfig`:

```ini
[merge "packagejson"]
    name = package.json merge driver
    driver = git-merge-driver-packagejson %O %A %B %L %P
```

Then create or update `.gitattributes` in your repository:

```
package.json merge=packagejson
```

[Git - gitattributes Documentation](https://git-scm.com/docs/gitattributes#_defining_a_custom_merge_driver) states:

Defining a custom merge driver
The definition of a merge driver is done in the .git/config file, not in the gitattributes file, so strictly speaking this manual page is a wrong place to talk about it. However…​

To define a custom merge driver filfre, add a section to your $GIT_DIR/config file (or $HOME/.gitconfig file) like this:

[merge "filfre"]
	name = feel-free merge driver
	driver = filfre %O %A %B %L %P
	recursive = binary
The merge.*.name variable gives the driver a human-readable name.

The merge.*.driver variable’s value is used to construct a command to run to common ancestor’s version (%O), current version (%A) and the other branches' version (%B). These three tokens are replaced with the names of temporary files that hold the contents of these versions when the command line is built. Additionally, %L will be replaced with the conflict marker size (see below).

The merge driver is expected to leave the result of the merge in the file named with %A by overwriting it, and exit with zero status if it managed to merge them cleanly, or non-zero if there were conflicts. When the driver crashes (e.g. killed by SEGV), it is expected to exit with non-zero status that are higher than 128, and in such a case, the merge results in a failure (which is different from producing a conflict).

The merge.*.recursive variable specifies what other merge driver to use when the merge driver is called for an internal merge between common ancestors, when there are more than one. When left unspecified, the driver itself is used for both internal merge and the final merge.

The merge driver can learn the pathname in which the merged result will be stored via placeholder %P. The conflict labels to be used for the common ancestor, local head and other head can be passed by using %S, %X and %Y respectively.