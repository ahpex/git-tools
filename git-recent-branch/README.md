# git-recent-branch

Lists the 5 most recently checked out branches and allows interactive selection to switch to.

## Usage

```bash
git-recent-branch [OPTIONS]
```

## Options

- `--repo <PATH>` - Path to repository (defaults to current directory)

## Example

```bash
# Show recent branches in current repo
git-recent-branch

# Show recent branches in specific repo
git-recent-branch --repo /path/to/repo
```

## How it works

The tool parses the git reflog to find the most recent branch checkouts, presents the 5 most recent branches using an interactive selector, and checks out the selected branch.
