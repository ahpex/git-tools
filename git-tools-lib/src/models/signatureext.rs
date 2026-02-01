use git2::Signature;

/// Extension trait for [`Signature`] providing additional convenience methods.
///
/// This trait extends `git2::Signature` with helper methods for comparison
/// and other common operations.
///
/// # Examples
///
/// ```no_run
/// use git2::Signature;
/// use git_tools_lib::SignatureExt;
///
/// let sig1 = Signature::now("Alice", "alice@example.com").unwrap();
/// let sig2 = Signature::now("Alice", "alice@example.com").unwrap();
///
/// // Compare signatures by name and email (ignores timestamp)
/// assert!(sig1.is_same_identity(&sig2));
/// ```
pub trait SignatureExt {
    /// Checks if two signatures have the same identity (name and email).
    ///
    /// This comparison ignores the timestamp, focusing only on the author's
    /// name and email address.
    ///
    /// # Arguments
    ///
    /// * `other` - The other signature to compare against
    ///
    /// # Returns
    ///
    /// `true` if both signatures have the same name and email, `false` otherwise.
    fn is_same_identity(&self, other: &Signature<'_>) -> bool;
}

impl SignatureExt for Signature<'_> {
    fn is_same_identity(&self, other: &Signature<'_>) -> bool {
        self.name() == other.name() && self.email() == other.email()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_identity_equal() {
        let sig1 = Signature::now("Alice", "alice@example.com").unwrap();
        let sig2 = Signature::now("Alice", "alice@example.com").unwrap();
        assert!(sig1.is_same_identity(&sig2));
    }

    #[test]
    fn test_is_same_identity_different_name() {
        let sig1 = Signature::now("Alice", "alice@example.com").unwrap();
        let sig2 = Signature::now("Bob", "alice@example.com").unwrap();
        assert!(!sig1.is_same_identity(&sig2));
    }

    #[test]
    fn test_is_same_identity_different_email() {
        let sig1 = Signature::now("Alice", "alice@example.com").unwrap();
        let sig2 = Signature::now("Alice", "bob@example.com").unwrap();
        assert!(!sig1.is_same_identity(&sig2));
    }
}
