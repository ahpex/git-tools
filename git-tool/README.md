# git-tool

A unified git utility tool with multiple subcommands.

## Usage

```bash
git-tool [OPTIONS] <COMMAND>
```

### Global Options

- `-r, --repo <PATH>` - Path to the git repository (default: current directory)

### Commands

#### `branch`

Commands related to branch operations.

```bash
# Show current branch name
git-tool branch

# Extract issue number from current branch name
git-tool branch --issue

# List all local branches
git-tool branch list
```

##### Options

- `--issue` - Extract and print the issue number from the current branch name

##### Subcommands

- `list` - List all local branches

## Examples

```bash
# If current branch is "feature/ABC-123-add-new-feature"
$ git-tool branch --issue
ABC-123

# Show current branch
$ git-tool branch
feature/ABC-123-add-new-feature

# List all branches
$ git-tool branch list
main
feature/ABC-123-add-new-feature
bugfix/DS-456-fix-crash
```

## Building

```bash
cargo build --release
```

The binary will be available at `target/release/git-tool`.
