use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::{fs, path::PathBuf};

/// Git merge driver for package.json files that intelligently merges dependencies
#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// Path to the base version (common ancestor) - %O
    #[arg(value_name = "BASE")]
    base: PathBuf,

    /// Path to the current version (ours) - %A
    #[arg(value_name = "CURRENT")]
    current: PathBuf,

    /// Path to the other version (theirs) - %B
    #[arg(value_name = "OTHER")]
    other: PathBuf,

    /// Conflict marker size - %L
    #[arg(value_name = "MARKER_SIZE")]
    marker_size: usize,

    /// Pathname where merged result will be stored - %P
    #[arg(value_name = "PATHNAME")]
    pathname: PathBuf,
}

fn merge_text(current_content: &str, other_content: &str) -> Result<String> {
    // Extract version from current (ours) using regex - capture the entire line
    let version_regex = Regex::new(r#"(?m)^\s*"version"\s*:\s*"[^"]+"\s*,?\s*$"#)?;
    let current_version_line = version_regex
        .find(current_content)
        .map(|m| m.as_str().to_owned());

    // Start with the other file's content
    let mut merged_content = other_content.to_owned();

    // Replace the first occurrence of version in merged content with our version
    if let Some(our_version_line) = current_version_line {
        if let Some(first_match) = version_regex.find(&merged_content) {
            let start = first_match.start();
            let end = first_match.end();
            merged_content.replace_range(start..end, &our_version_line);
        }
    }

    Ok(merged_content)
}

fn merge_package_json(current: &PathBuf, other: &PathBuf) -> Result<()> {
    // Read both versions as text
    let current_content = fs::read_to_string(current)
        .with_context(|| format!("Failed to read current file: {:?}", current))?;
    let other_content = fs::read_to_string(other)
        .with_context(|| format!("Failed to read other file: {:?}", other))?;

    // Perform the merge
    let merged_content = merge_text(&current_content, &other_content)?;

    // Write the merged result back to current file
    fs::write(current, merged_content)
        .with_context(|| format!("Failed to write merged result to {:?}", current))?;

    Ok(())
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Git expects the result to be written to the current version file (%A)
    merge_package_json(&args.current, &args.other)?;

    // Exit with 0 to indicate successful merge (no conflicts)
    std::process::exit(0);
}

#[cfg(test)]
#[cfg_attr(
    test,
    expect(clippy::unwrap_used, reason = "Tests, using unwrap is acceptable")
)]
mod tests {
    use super::*;

    #[test]
    fn test_version_preserved_from_current() {
        let current = r#"{
  "name": "my-package",
  "version": "1.2.3",
  "dependencies": {
    "foo": "^1.0.0"
  }
}"#;

        let other = r#"{
  "name": "my-package",
  "version": "3.4.5",
  "dependencies": {
    "foo": "^1.0.0",
    "bar": "^2.0.0"
  }
}"#;

        let result = merge_text(current, other).unwrap();

        // Version should be from current (1.2.3), not other (3.4.5)
        assert!(
            result.contains(r#""version": "1.2.3""#),
            "Version should be 1.2.3 from current"
        );
        assert!(
            !result.contains(r#""version": "3.4.5""#),
            "Version should not be 3.4.5 from other"
        );
        // Other content should be from other file
        assert!(
            result.contains(r#""bar": "^2.0.0""#),
            "Should include bar dependency from other"
        );
    }

    #[test]
    fn test_version_with_trailing_comma() {
        let current = r#"{
  "version": "1.0.0",
  "name": "test"
}"#;

        let other = r#"{
  "version": "2.0.0",
  "name": "test",
  "description": "updated"
}"#;

        let result = merge_text(current, other).unwrap();

        assert!(
            result.contains(r#""version": "1.0.0""#),
            "Version should be preserved from current"
        );
        assert!(
            result.contains(r#""description": "updated""#),
            "Should include description from other"
        );
    }

    #[test]
    fn test_no_version_in_current() {
        let current = r#"{
  "name": "my-package"
}"#;

        let other = r#"{
  "name": "my-package",
  "version": "1.0.0"
}"#;

        let result = merge_text(current, other).unwrap();

        // When current has no version, keep other's version
        assert!(
            result.contains(r#""version": "1.0.0""#),
            "Should keep version from other when current has none"
        );
    }
}
