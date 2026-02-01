use std::{fmt::Display, sync::OnceLock};

static ISSUE_ID_REGEX: OnceLock<regex::Regex> = OnceLock::new();

fn issue_id_regex() -> &'static regex::Regex {
    ISSUE_ID_REGEX.get_or_init(|| regex::Regex::new(r"[a-z]+-\d+").unwrap())
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
/// Represents an issue number extracted from a string.
///
/// An issue number follows the pattern of lowercase letters followed by a hyphen and digits
/// (e.g., "ABC-123", "DS-789", "WSQUAK-765"). The `IssueNumber` struct stores the normalized
/// (uppercase) version of the matched issue number.
///
/// # Examples
///
/// Parse an issue number from a string:
///
/// ```
/// # use git_tools_lib::models::IssueNumber;
/// let issue = IssueNumber::new("ABC-123").unwrap();
/// assert_eq!(issue.as_str(), "ABC-123");
/// ```
///
/// Parse an issue number from text containing more than just the issue number:
///
/// ```
/// # use git_tools_lib::models::IssueNumber;
/// let issue = IssueNumber::new("Fix bug in DS-789 feature").unwrap();
/// assert_eq!(issue.as_str(), "DS-789");
/// ```
///
/// The struct automatically normalizes issue numbers to uppercase:
///
/// ```
/// # use git_tools_lib::models::IssueNumber;
/// let issue = IssueNumber::new("wsquak-765").unwrap();
/// assert_eq!(issue.as_str(), "WSQUAK-765");
/// ```
///
/// Return `None` if no valid issue number is found:
///
/// ```
/// # use git_tools_lib::models::IssueNumber;
/// assert!(IssueNumber::new("no issue here").is_none());
/// ```
pub struct IssueNumber {
    inner: String,
}

impl IssueNumber {
    /// Returns a new `IssueNumber` by parsing a string to extract an issue number.
    ///
    /// This method searches for a pattern matching issue number format (e.g., "ABC-123") in the
    /// provided text. If found, the matched issue number is normalized to uppercase and returned
    /// as an `IssueNumber`. If no match is found, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for an issue number. Can be any type that implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// `Some(IssueNumber)` if an issue number is found in the text, `None` otherwise.
    ///
    /// # Examples
    ///
    /// Parse from a string slice:
    ///
    /// ```
    /// # use git_tools_lib::models::IssueNumber;
    /// let issue = IssueNumber::new("ABC-123");
    /// assert!(issue.is_some());
    /// assert_eq!(issue.unwrap().as_str(), "ABC-123");
    /// ```
    ///
    /// Parse from an owned String:
    ///
    /// ```
    /// # use git_tools_lib::models::IssueNumber;
    /// let text = String::from("feat: implement feature DS-789 for testing");
    /// let issue = IssueNumber::new(text);
    /// assert_eq!(issue.unwrap().as_str(), "DS-789");
    /// ```
    ///
    /// Parse from text with surrounding content:
    ///
    /// ```
    /// # use git_tools_lib::models::IssueNumber;
    /// let issue = IssueNumber::new("feat: implement feature DS-789 for testing");
    /// assert_eq!(issue.unwrap().as_str(), "DS-789");
    /// ```
    pub fn new<S: AsRef<str>>(text: S) -> Option<Self> {
        issue_id_regex()
            .find(&text.as_ref().to_lowercase())
            .map(|mat| Self {
                inner: mat.as_str().to_uppercase().to_owned(),
            })
    }

    /// Returns the issue number as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_tools_lib::models::IssueNumber;
    /// let issue = IssueNumber::new("WSQUAK-765").unwrap();
    /// assert_eq!(issue.as_str(), "WSQUAK-765");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl AsRef<str> for IssueNumber {
    /// Converts the `IssueNumber` to a string reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_tools_lib::models::IssueNumber;
    /// use std::convert::AsRef;
    /// let issue = IssueNumber::new("ABC-123").unwrap();
    /// let s: &str = issue.as_ref();
    /// assert_eq!(s, "ABC-123");
    /// ```
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl Display for IssueNumber {
    /// Formats the `IssueNumber` as a string for display.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_tools_lib::models::IssueNumber;
    /// let issue = IssueNumber::new("ABC-123").unwrap();
    /// assert_eq!(format!("{}", issue), "ABC-123");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_number_parsing() {
        assert!(IssueNumber::new("ABC-123").is_some());
        assert!(IssueNumber::new("a-123").is_some());
        assert!(IssueNumber::new("NeVeRmOrE-1").is_some());
        assert!(IssueNumber::new("ABCA-456").is_some());
        assert!(IssueNumber::new("DS-789").is_some());
        assert!(IssueNumber::new("this is DS-789 text").is_some());

        assert!(IssueNumber::new("no issue here").is_none());
        assert!(IssueNumber::new("").is_none());
    }

    #[test]
    fn test_issue_number_as_str() {
        assert_eq!(
            "WSQUAK-765",
            IssueNumber::new("WSQUAK-765").unwrap().as_str()
        );
        assert_eq!(
            "DS-789",
            IssueNumber::new("text DS-789 text").unwrap().as_str()
        );
    }

    #[test]
    fn test_issue_number_uppercased() {
        assert_eq!(
            "WSQUAK-765",
            IssueNumber::new("wsquak-765").unwrap().as_str()
        );
    }

    #[test]
    fn test_issue_number_first_match() {
        assert_eq!(
            "WSQUAK-123",
            IssueNumber::new("WSQUAK-123, ABC-10000").unwrap().as_str()
        );
        assert_eq!(
            "WSQUAK-287",
            IssueNumber::new("text WSQUAK-287 text").unwrap().as_str()
        );
    }
}
