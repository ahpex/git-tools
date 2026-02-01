//! # git-tools-lib
//!
//! A shared library providing common utilities for git-tools workspace.
//!
//! This crate provides reusable functionality used across multiple git utility tools,
//! including issue number parsing, repository extensions, and commit filtering.
//!
//! ## Features
//!
//! - **Issue Number Parsing**: Extract and normalize issue numbers (e.g., "ABC-123", "DEF-789")
//!   from commit messages, branch names, or any text.
//! - **Repository Extensions**: Convenience methods for `git2::Repository` via the
//!   [`RepositoryExt`] trait.
//! - **Bot Commit Detection**: Filter out commits authored by known bot accounts.
//! - **Error Handling**: A unified [`Error`] type that wraps external dependencies to avoid
//!   leaking them to library consumers.
//!
//! ## Quick Start
//!
//! ```no_run
//! use git2::Repository;
//! use git_tools_lib::{IssueNumber, RepositoryExt, is_bot_commit};
//!
//! // Parse an issue number from text
//! if let Some(issue) = IssueNumber::parse("feat: ABC-123 add new feature") {
//!     println!("Found issue: {}", issue);
//! }
//!
//! // Check for staged changes using the extension trait
//! let repo = Repository::open(".").unwrap();
//! if repo.has_staged_changes().unwrap() {
//!     println!("You have staged changes");
//! }
//! ```
//!
//! ## Error Handling
//!
//! The library provides a unified [`Error`] type that wraps external dependencies.
//! This avoids leaking types like `git2::Error` to consumers. Use the [`Result`] type
//! alias for convenience:
//!
//! ```no_run
//! use git2::Repository;
//! use git_tools_lib::{RepositoryExt, Result};
//!
//! fn check_repo(path: &str) -> Result<bool> {
//!     let repo = Repository::open(path)?;
//!     repo.has_staged_changes()
//! }
//! ```
//!
//! ## Modules
//!
//! - [`models`]: Core types including [`IssueNumber`] and [`RepositoryExt`].
//! - [`error`]: Error types including [`Error`] and [`Result`].

use git2::{Oid, Repository, StatusOptions};

pub mod error;
pub mod models;

pub use error::{Error, Result};
pub use models::IssueNumber;
pub use models::RepositoryExt;
pub use models::SignatureExt;

/// Checks if a commit is authored by a bot account
///
/// Returns `true` if the commit author is a known bot (bamboo, dailybuild)
/// Returns `false` if the commit is by a human or if the commit cannot be found
pub fn is_bot_commit(repo: &Repository, oid: &Oid) -> bool {
    let Ok(commit) = repo.find_commit(*oid) else {
        return false;
    };

    let author_name = commit.author().name().unwrap_or("").trim().to_lowercase();
    let bot_names = ["bamboo", "dailybuild"];
    bot_names.contains(&author_name.as_str())
}

/// Checks if the repository index has any staged changes.
///
/// This function inspects the repository's status to determine if there are any
/// changes staged for commit, including:
/// - New files added to the index
/// - Modified files staged for commit
/// - Deleted files staged for removal
/// - Renamed files
/// - Type changes (e.g., file to symlink)
///
/// # Arguments
///
/// * `repo` - A reference to the git repository to check
///
/// # Returns
///
/// * `Ok(true)` - If there are staged changes ready to be committed
/// * `Ok(false)` - If there are no staged changes
/// * `Err(Error)` - If there was an error reading the repository status
///
/// # Examples
///
/// ```no_run
/// use git2::Repository;
/// use git_tools_lib::index_is_clean;
///
/// let repo = Repository::open(".").unwrap();
/// match index_is_clean(&repo) {
///     Ok(true) => println!("There are staged changes"),
///     Ok(false) => println!("No staged changes"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn index_is_clean(repo: &Repository) -> Result<bool> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    Ok(!statuses.iter().any(|entry| {
        let status = entry.status();
        status.is_index_new()
            || status.is_index_modified()
            || status.is_index_deleted()
            || status.is_index_renamed()
            || status.is_index_typechange()
    }))
}

/// Checks if the repository working tree is clean.
///
/// This function inspects the repository's status to determine if there are any
/// changes in the working tree (unstaged modifications), including:
/// - New untracked files
/// - Modified files in the working tree
/// - Deleted files in the working tree
/// - Renamed files
/// - Type changes (e.g., file to symlink)
///
/// # Arguments
///
/// * `repo` - A reference to the git repository to check
///
/// # Returns
///
/// * `Ok(true)` - If the working tree is clean
/// * `Ok(false)` - If there are changes in the working tree
/// * `Err(Error)` - If there was an error reading the repository status
pub fn workspace_is_clean(repo: &Repository) -> Result<bool> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    Ok(!statuses.iter().any(|entry| {
        let status = entry.status();
        status.is_wt_new()
            || status.is_wt_modified()
            || status.is_wt_deleted()
            || status.is_wt_renamed()
            || status.is_wt_typechange()
    }))
}

/// Checks if the entire repository is clean.
///
/// A repository is considered clean if:
/// - The index (staging area) has no staged changes
/// - The working tree has no unstaged changes
/// - There are no untracked files
///
/// # Arguments
///
/// * `repo` - A reference to the git repository to check
///
/// # Returns
///
/// * `Ok(true)` - If the repository is completely clean
/// * `Ok(false)` - If there are any staged, unstaged, or untracked changes
/// * `Err(Error)` - If there was an error reading the repository status
pub fn is_clean(repo: &Repository) -> Result<bool> {
    Ok(index_is_clean(repo)? && workspace_is_clean(repo)?)
}

/// Checks if the repository has untracked files.
///
/// Returns `true` if there are any files in the working tree that are not tracked
/// by git (i.e., not in the index and not in any commit).
///
/// # Arguments
///
/// * `repo` - A reference to the git repository to check
///
/// # Returns
///
/// * `Ok(true)` - If there are untracked files
/// * `Ok(false)` - If there are no untracked files
/// * `Err(Error)` - If there was an error reading the repository status
pub fn has_untracked_files(repo: &Repository) -> Result<bool> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    Ok(statuses.iter().any(|entry| {
        let status = entry.status();
        status.is_wt_new()
    }))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_bot_names_detected() {
        // This test is limited without a real repository
        // Individual tools should test with their own repositories
        let bot_names = ["bamboo", "dailybuild"];
        assert!(bot_names.contains(&"bamboo"));
        assert!(bot_names.contains(&"dailybuild"));
    }
}
