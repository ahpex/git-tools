# git-squash

A command-line tool to squash the last N commits into a single commit.

## Usage

```bash
# Squash the last 3 commits
git-squash --count 3

# Squash with a new commit message
git-squash --count 3 --message "Combined feature implementation"

# Squash in a specific repository
git-squash --count 5 --repo /path/to/repo
```

## Options

- `--count <N>`: Number of commits to squash (required)
- `--message <MSG>`: New commit message for the squashed commit (optional, defaults to first commit's message)
- `--repo <PATH>`: Path to the git repository (default: current directory)

## Description

This tool squashes the last N commits on the current branch into a single commit. It:

1. Verifies the repository has enough commits to squash
2. Resets the branch to N commits back (soft reset)
3. Creates a new commit with all the changes
4. Preserves the original author of the first commit

**Warning**: This operation modifies commit history. Use with caution on shared branches.

## Example

```bash
# Before: 3 commits
# - commit 3: "Add tests"
# - commit 2: "Fix bug"  
# - commit 1: "Add feature"

git-squash --count 3 --message "Complete feature with tests and bugfix"

# After: 1 commit
# - commit 1: "Complete feature with tests and bugfix"
```
