# Git Tools - Agent Instructions

## Project Overview

This is a Rust workspace containing multiple git utility command-line tools. Each tool is a separate crate within the workspace.

## Project Structure

- **Workspace Layout**: Cargo workspace with multiple binary crates
- **Language**: Rust (edition 2021)
- **Build System**: Cargo
- **Tools Location**: Each tool has its own directory with `Cargo.toml` and `src/` folder
- **Tools Overview**: All tools shall be documented below and in the main `README.md`

### Available Tools

1. **git-file-commit-report**: Lists all commits from a SHA to another SHA (inclusive) and shows modified files
2. **git-branch-stale**: Lists stale git branches without commits for n days
3. **git-fake-commit**: Creates dummy commits for testing and experimentation
4. **git-first-mention**: Searches for the first mention of a string in repository history
5. **git-issues-in-progress**: Shows in-progress issues from recent commits on remote branches
6. **git-merge-driver-packagejson**: Git merge driver for package.json files that intelligently merges dependencies
7. **git-tag-fork-point**: Creates lightweight tags at branch fork points from main

## Code Conventions

### General Practices

- Use `anyhow` for error handling with context
- Use `clap` with derive macros for CLI argument parsing
- Use `git2` for git repository operations
- Use `nu_ansi_term` for colored terminal output
- Prefer `std::sync::OnceLock` for compile-time regex patterns and static initialization

### Code Style

- **Error Handling**: Use `anyhow::Result` and `.context()` for detailed error messages
- **CLI Structure**: Define a `CliArgs` struct with `#[derive(Parser)]`
- **Repository Access**: Accept `PathBuf` for repository paths, default to "."
- **Output**: Use colored output via `nu_ansi_term::Color` for better readability
- **Commit Filtering**: Treat commits by "bamboo" and "dailybuild" as bot commits
- **Iterator Usage**: Leverage iterators and combinators for processing collections instead of loops and mutable collections

### Typical Function Structure

```rust
fn main() -> Result<()> {
    let args = CliArgs::parse();
    let repo = Repository::open(&args.repo)
        .with_context(|| format!("Unable to open git repository at '{:?}'", args.repo))?;
    // Main logic
}
```

### Bot Detection Pattern

When filtering commits by author, exclude bot accounts:
```rust
fn is_human_commit(repo: &Repository, oid: &git2::Oid) -> bool {
    let commit = repo.find_commit(*oid).unwrap();
    let author_name = commit.author().name().unwrap_or("").trim().to_lowercase();
    let bot_names = ["bamboo", "dailybuild"];
    !bot_names.contains(&author_name.as_str())
}
```

## Report Generation

When asked to generate reports, ensure:

- Confluence Wiki Markup format is used for Confluence output. You may refer to https://confluence.atlassian.com/doc/confluence-wiki-markup-251003035.html for syntax details.

## Building and Testing

- **Build Command**: `cargo build` (runs from workspace root)
- **Build Task**: Use the VS Code task "cargo build" (default build task)
- **Target Directory**: `target/debug/` for debug builds, `target/release/` for release builds

## Development Workflow

1. Make changes to individual tool crates
2. Build the entire workspace with `cargo build`
3. Test individual tools by running the binaries from `target/debug/`
4. Each tool can be developed independently but shares workspace dependencies

## Common Patterns

### Working with Git2 Repository

```rust
let repo = Repository::open(&path)?;
let branches = repo.branches(Some(BranchType::Remote))?;
let mut revwalk = repo.revwalk()?;
```

### Parsing Git Commit Data

- Use `commit.time().seconds()` with `chrono::DateTime::from_timestamp`
- Extract author with `commit.author().name()`
- Get commit message with `commit.message()`

## Error Handling

### Library Crate (`git-tools-lib`)

The library uses `thiserror` with a custom `Error` enum that wraps external dependencies to avoid leaking them to consumers. This follows the best practices from [Designing Error Types in Rust Libraries](https://d34dl0ck.me/rust-bites-designing-error-types-in-rust-libraries/index.html).

**Key principles:**
- **Wrap external errors**: Use wrapper types (e.g., `GitError`) to encapsulate external crate errors like `git2::Error`
- **Don't leak dependencies**: Library consumers should not need to depend on `git2` to handle errors
- **Use `#[non_exhaustive]`**: Allow adding new error variants without breaking changes
- **Provide `From` implementations**: Enable seamless error conversion with `?` operator

```rust
// In git-tools-lib
use git_tools_lib::{Result, Error};

pub fn my_function(repo: &Repository) -> Result<()> {
    // git2::Error automatically converts to Error via From impl
    let status = repo.statuses(None)?;
    Ok(())
}
```

### Binary Crates (Tools)

Binary crates use `anyhow` for convenient error handling with context:

```rust
use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let repo = Repository::open(&args.repo)
        .with_context(|| format!("Unable to open git repository at '{:?}'", args.repo))?;
    
    if some_condition {
        bail!("Descriptive error message");
    }
    
    Ok(())
}
```

**When to use which:**
- `git-tools-lib`: Use `thiserror` with custom `Error` type and `Result` alias
- Binary tools: Use `anyhow::Result` with `.context()` and `bail!()`

## Important Notes

- All tools default to current directory (`.`) for repository path
- Tools are designed for interactive terminal use with colored output
- Error messages should provide clear context about what operation failed
- Keep bot detection centralized and consistent across tools that filter commits
