use crate::ModifiedFile;
use anyhow::{Context, Result};
use git2::Oid;
use nu_ansi_term::Color;
use std::fs;
use std::fs::File;
use std::path::Path;

/// Trait for generating output reports
pub trait Output {
    /// Generate the report and write to the specified output path
    ///
    /// # Arguments
    /// * `modified_files` - Slice of ModifiedFile structs to include in the report
    /// * `output_path` - Path to write the generated report    
    ///
    /// # Returns
    /// Returns Result indicating success or failure
    fn generate(&self, modified_files: &[ModifiedFile], output_path: &Path) -> Result<()>;
}

/// Markdown output generator
pub struct MarkdownOutput;

/// Confluence wiki output generator
pub struct ConfluenceOutput;

/// Console output generator
pub struct ConsoleOutput;

/// JSON output generator
pub struct JsonOutput;

impl Output for ConsoleOutput {
    fn generate(&self, modified_files: &[ModifiedFile], _output_path: &Path) -> Result<()> {
        println!("\n{}", Color::Cyan.bold().paint("Modified Files Report"));
        println!("{}", Color::Cyan.paint("=".repeat(80)));

        if modified_files.is_empty() {
            println!(
                "\n{}",
                Color::Yellow.paint("No modified files found with issues.")
            );
            return Ok(());
        }

        for file in modified_files {
            println!(
                "\n{}",
                Color::Green
                    .bold()
                    .paint(format!("File: {}", file.file.display()))
            );

            for reference in &file.references {
                let sha_short = &reference.sha.to_string();
                let issues_str = if reference.issues.is_empty() {
                    String::new()
                } else {
                    format!(" - {}", reference.issues.join(", "))
                };

                println!(
                    "  {}{}",
                    Color::White.paint(sha_short),
                    Color::Magenta.paint(issues_str)
                );
            }
        }

        println!("\n{}", Color::Cyan.paint("=".repeat(80)));
        println!(
            "{} files modified",
            Color::Yellow.paint(modified_files.len().to_string())
        );

        Ok(())
    }
}

impl Output for MarkdownOutput {
    fn generate(&self, modified_files: &[ModifiedFile], output_path: &Path) -> Result<()> {
        let mut content = String::new();

        content.push_str("# Checkins Report\n\n");

        if modified_files.is_empty() {
            content.push_str("No modified files found with issues.\n");
        } else {
            // Number files starting at 1
            for (i, file) in modified_files.iter().enumerate() {
                let index = i + 1;
                content.push_str(&format!("{}. {}\n\n", index, file.file.display()));

                for reference in &file.references {
                    let sha_link = format_sha_link(&reference.sha);

                    if reference.issues.is_empty() {
                        content.push_str(&format!("- {}\n", sha_link));
                    } else {
                        let issue_links: Vec<String> = reference
                            .issues
                            .iter()
                            .map(|issue| format_issue_link(issue))
                            .collect();
                        content.push_str(&format!("- {} - {}\n", sha_link, issue_links.join(", ")));
                    }
                }

                content.push('\n');
            }

            content.push_str(&format!(
                "\n---\n\n**Total**: {} files modified\n",
                modified_files.len()
            ));
        }

        fs::write(output_path, content)
            .with_context(|| format!("Unable to write report to {:?}", output_path))?;

        Ok(())
    }
}

impl Output for ConfluenceOutput {
    fn generate(&self, modified_files: &[ModifiedFile], output_path: &Path) -> Result<()> {
        let mut content = String::new();
        // Header: | No. | Department | filename | shas | issues | (/) |
        content.push_str("| No. | Department | Filename | SHAs | Issues | (/) |\n");

        for (i, file) in modified_files.iter().enumerate() {
            let index = i + 1;
            // Collect unique SHAs and issues for this file
            let mut sha_set: Vec<Oid> = Vec::new();
            let mut issue_set: Vec<String> = Vec::new();

            for reference in &file.references {
                let sha_full = reference.sha;
                if !sha_set.contains(&sha_full) {
                    sha_set.push(sha_full);
                }

                for issue in &reference.issues {
                    if !issue_set.contains(issue) {
                        issue_set.push(issue.clone());
                    }
                }
            }

            // Format as comma-separated lists (links below will be used in cells)

            // Build Fisheye link(s) and Jira links inline
            let fisheye_links: Vec<String> = sha_set
                .iter()
                .map(|s| format!("[{}|https://example.com/orga/project/commit/{}]", s, s))
                .collect();

            let jira_links: Vec<String> = issue_set
                .iter()
                .map(|i| format!("[{}|https://jira.example.com/browse/{}]", i, i))
                .collect();

            let fisheye_cell = if fisheye_links.is_empty() {
                String::new()
            } else {
                fisheye_links.join(", ")
            };
            let jira_cell = if jira_links.is_empty() {
                String::new()
            } else {
                jira_links.join(", ")
            };

            content.push_str(&format!(
                "| {} |  | {} | {} | {} |   |\n",
                index,
                file.file.display(),
                fisheye_cell,
                jira_cell
            ));
        }

        fs::write(output_path, content)
            .with_context(|| format!("Unable to write confluence report to {:?}", output_path))?;

        println!(
            "{}",
            Color::Green.paint(format!("Confluence report written to: {:?}", output_path))
        );

        Ok(())
    }
}

impl Output for JsonOutput {
    fn generate(&self, modified_files: &[ModifiedFile], output_path: &Path) -> Result<()> {
        let file = File::create(output_path)
            .with_context(|| format!("Unable to create JSON report file {:?}", output_path))?;

        serde_json::to_writer(file, modified_files)
            .context("Failed to serialize modified files to JSON")?;

        println!(
            "{}",
            Color::Green.paint(format!("JSON report written to: {:?}", output_path))
        );

        Ok(())
    }
}

/// Format an issue ID as a clickable Jira link
fn format_issue_link(issue: &str) -> String {
    format!("[{}](https://jira.example.com/browse/{})", issue, issue)
}

/// Format a commit SHA as a clickable FishEye link
fn format_sha_link(oid: &Oid) -> String {
    let sha_str = oid.to_string();
    let sha_short = &sha_str[..8];
    format!(
        "[{}](https://example.com/orga/project/commit/{})",
        sha_short, sha_str
    )
}
