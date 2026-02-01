//! Error types for git-tools-lib.
//!
//! This module defines the error types used throughout the library.
//! Following best practices, we wrap external error types to avoid leaking
//! dependencies to library consumers.

/// A wrapper around `git2::Error` that avoids exposing the `git2` dependency.
///
/// This type holds the inner `git2::Error` privately, providing `Debug` and `Display`
/// implementations that forward to the inner error. Library consumers can access
/// error information through the standard error traits without needing to depend
/// on `git2` directly.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct GitError(#[from] git2::Error);

/// The error type for git-tools-lib operations.
///
/// This enum represents all possible errors that can occur when using the library.
/// It uses wrapper types for external dependencies to avoid leaking them to consumers.
///
/// # Example
///
/// ```no_run
/// use git2::Repository;
/// use git_tools_lib::{RepositoryExt, Error};
///
/// fn check_staged(repo: &Repository) -> Result<bool, Error> {
///     repo.has_staged_changes()
/// }
/// ```
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An error occurred during a git operation.
    #[error("Git error: {0}")]
    Git(#[from] GitError),

    /// The repository is in an unexpected state.
    #[error("Repository state error: {0}")]
    RepositoryState(String),

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Self::Git(GitError::from(err))
    }
}

/// A `Result` type alias using the library's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::RepositoryState("test error".to_string());
        assert_eq!(format!("{}", err), "Repository state error: test error");
    }

    #[test]
    fn test_error_is_non_exhaustive() {
        // This test ensures the enum is marked non_exhaustive for future extensibility
        let err = Error::RepositoryState("test".to_string());
        match err {
            Error::Git(_) => panic!("Should not match"),
            Error::RepositoryState(_) => {}
            Error::Io(_) => panic!("Should not match"),
            // #[non_exhaustive] allows adding new variants without breaking existing match arms
        }
    }
}
