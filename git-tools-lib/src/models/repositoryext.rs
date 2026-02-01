use git2::{Repository, StatusOptions};

use crate::error::Result;

/// Extension trait for [`Repository`] providing additional convenience methods.
///
/// This trait extends `git2::Repository` with helper methods commonly used
/// across the git-tools workspace.
///
/// # Examples
///
/// ```no_run
/// use git2::Repository;
/// use git_tools_lib::RepositoryExt;
///
/// let repo = Repository::open(".").unwrap();
/// if repo.has_staged_changes().unwrap() {
///     println!("There are staged changes");
/// }
/// ```
pub trait RepositoryExt {
    /// Checks if the repository index has any staged changes.
    ///
    /// This method inspects the repository's status to determine if there are any
    /// changes staged for commit, including:
    /// - New files added to the index
    /// - Modified files staged for commit
    /// - Deleted files staged for removal
    /// - Renamed files
    /// - Type changes (e.g., file to symlink)
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If there are staged changes ready to be committed
    /// * `Ok(false)` - If there are no staged changes
    /// * `Err(Error)` - If there was an error reading the repository status
    fn has_staged_changes(&self) -> Result<bool>;
}

impl RepositoryExt for Repository {
    fn has_staged_changes(&self) -> Result<bool> {
        let mut opts = StatusOptions::new();
        let statuses = self.statuses(Some(&mut opts))?;

        Ok(statuses.iter().any(|entry| {
            let status = entry.status();
            status.is_index_new()
                || status.is_index_modified()
                || status.is_index_deleted()
                || status.is_index_renamed()
                || status.is_index_typechange()
        }))
    }
}
