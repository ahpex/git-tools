use anyhow::{Context, Result};
use clap::Parser;
use git2::{DiffOptions, Oid, Repository};
use nu_ansi_term::Color;
use regex::Regex;
use serde::{Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::reports::{ConfluenceOutput, ConsoleOutput, JsonOutput, MarkdownOutput, Output};

mod reports;

static ISSUE_ID_REGEX: OnceLock<Regex> = OnceLock::new();
static IGNORED_FILES: OnceLock<HashSet<&'static str>> = OnceLock::new();
static IGNORED_EXTENSIONS: OnceLock<HashSet<&'static str>> = OnceLock::new();

#[expect(
    clippy::expect_used,
    reason = "Regex is compile-time constant and panicking is appropriate if it fails"
)]
fn issue_id_regex() -> &'static Regex {
    ISSUE_ID_REGEX.get_or_init(|| {
        Regex::new(r"(TX(R[A-Z]?|PIV)|DS)-\d+").expect("Failed to compile issue ID regex")
    })
}

fn ignored_files() -> &'static HashSet<&'static str> {
    IGNORED_FILES.get_or_init(|| HashSet::from_iter(["package.json", "sdks.json", "dev.json"]))
}

fn ignored_extensions() -> &'static HashSet<&'static str> {
    IGNORED_EXTENSIONS.get_or_init(|| HashSet::from_iter([".spec.ts", ".spec.js", ".xml"]))
}

/// Serialize an Oid as its string representation
fn serialize_oid<S>(oid: &Oid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&oid.to_string())
}

/// Reference to a commit that modified a file
#[derive(Debug, Clone, Serialize)]
struct CommitReference {
    #[serde(serialize_with = "serialize_oid")]
    sha: Oid,
    issues: Vec<String>,
}

/// A file that was modified between two commits with all associated commit references
#[derive(Debug, Serialize)]
struct ModifiedFile {
    file: PathBuf,
    references: Vec<CommitReference>,
}

#[derive(Parser)]
#[command(name = "git-file-commit-report")]
#[command(about = "Lists all commits and modified files between two commits")]
#[command(version = include_str!("../../target/git_commit.txt").trim(), about, long_about = None)]
struct CliArgs {
    /// Starting commit (from this commit, exclusive) - can be a SHA, reference like HEAD, or HEAD~4
    from_oid: String,

    /// Ending commit (to this commit, inclusive) - can be a SHA, reference like HEAD, or HEAD~4
    to_oid: String,

    /// Directory paths to filter files (e.g., "modules/testinglibrary")
    paths: Vec<String>,

    /// Path to git repository
    #[arg(short, long, default_value = ".")]
    repo: PathBuf,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let repo = Repository::open(&args.repo)
        .with_context(|| format!("Unable to open git repository at '{:?}'", args.repo))?;

    let from_oid = resolve_reference(&repo, &args.from_oid)
        .with_context(|| format!("Unable to resolve from reference: {}", args.from_oid))?;

    let to_oid = resolve_reference(&repo, &args.to_oid)
        .with_context(|| format!("Unable to resolve to reference: {}", args.to_oid))?;

    println!("{}", Color::Cyan.paint("Analyzing commits..."));

    // Get all files in to_oid
    let files_in_to_oid = get_files_in_commit(&repo, to_oid)?;

    // Filter files based on criteria
    let filtered_files = filter_files(files_in_to_oid, &args.paths);

    println!(
        "{}",
        Color::Cyan.paint(format!("Found {} files in scope", filtered_files.len()))
    );

    // Get commits between from_oid and to_oid
    let commits = get_commits_between(&repo, from_oid, to_oid)?;

    println!(
        "{}",
        Color::Cyan.paint(format!("Analyzing {} commits...", commits.len()))
    );

    // Build modified files map
    let modified_files = build_modified_files_map(&repo, &filtered_files, &commits, &args.paths)?;

    // Create vector of all outputters with their respective paths
    let outputters: Vec<(Box<dyn Output>, &Path)> = vec![
        (Box::new(ConsoleOutput), Path::new("")),
        (
            Box::new(MarkdownOutput),
            Path::new("git-file-commit-report.md"),
        ),
        (
            Box::new(ConfluenceOutput),
            Path::new("git-file-commit-report.txt"),
        ),
        (
            Box::new(JsonOutput),
            Path::new("git-file-commit-report.json"),
        ),
    ];

    // Output results using all outputters
    for (outputter, path) in &outputters {
        outputter.generate(&modified_files, path)?;
    }

    Ok(())
}

/// Resolve a reference (SHA, HEAD, HEAD~4, etc.) to an Oid
fn resolve_reference(repo: &Repository, reference: &str) -> Result<Oid> {
    // First try to parse as a direct SHA
    if let Ok(oid) = Oid::from_str(reference) {
        return Ok(oid);
    }

    // Try to resolve as a reference (HEAD, HEAD~4, branch name, etc.)
    let object = repo
        .revparse_single(reference)
        .with_context(|| format!("Unable to resolve reference: {}", reference))?;

    Ok(object.id())
}

/// Get all files present in a specific commit
fn get_files_in_commit(repo: &Repository, oid: Oid) -> Result<Vec<PathBuf>> {
    let commit = repo
        .find_commit(oid)
        .with_context(|| format!("Unable to find commit {}", oid))?;

    let tree = commit.tree().context("Unable to get tree from commit")?;

    let mut files = Vec::new();

    tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
        if entry.kind() == Some(git2::ObjectType::Blob) {
            let path = PathBuf::from(root).join(entry.name().unwrap_or(""));
            files.push(path);
        }
        git2::TreeWalkResult::Ok
    })?;

    Ok(files)
}

/// Filter files based on directory requirements and exclusion rules
fn filter_files(files: Vec<PathBuf>, _paths: &[String]) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|path| {
            // Check if it's an ignored file
            let is_ignored = path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|file_name| {
                    ignored_files().contains(file_name)
                        || ignored_extensions()
                            .iter()
                            .any(|ext| file_name.ends_with(ext))
                });

            !is_ignored
        })
        .collect()
}

/// Get all commits between from_oid (exclusive) and to_oid (inclusive)
fn get_commits_between(repo: &Repository, from_oid: Oid, to_oid: Oid) -> Result<Vec<Oid>> {
    let mut revwalk = repo.revwalk().context("Unable to create revwalk")?;

    revwalk
        .push(to_oid)
        .with_context(|| format!("Unable to push to_oid {} to revwalk", to_oid))?;

    revwalk
        .hide(from_oid)
        .with_context(|| format!("Unable to hide from_oid {} in revwalk", from_oid))?;

    let commits: Result<Vec<Oid>> = revwalk
        .map(|oid_result| oid_result.context("Error walking commits"))
        .collect();

    commits
}

/// Build a map of modified files with their commit references
fn build_modified_files_map(
    repo: &Repository,
    filtered_files: &[PathBuf],
    commits: &[Oid],
    paths: &[String],
) -> Result<Vec<ModifiedFile>> {
    let mut file_map: HashMap<PathBuf, Vec<CommitReference>> = HashMap::new();
    let filtered_set: HashSet<_> = filtered_files.iter().collect();

    for &commit_oid in commits {
        let commit = repo
            .find_commit(commit_oid)
            .with_context(|| format!("Unable to find commit {}", commit_oid))?;

        // Extract issues from commit message
        let issues = extract_issues(commit.message().unwrap_or(""));

        // Get changed files in this commit
        let changed_files = get_changed_files(repo, commit_oid, paths)?;

        // Filter to only files we care about
        for file in changed_files {
            if filtered_set.contains(&file) {
                let reference = CommitReference {
                    sha: commit_oid,
                    issues: issues.clone(),
                };

                file_map.entry(file.clone()).or_default().push(reference);
            }
        }
    }

    // Convert to ModifiedFile vec
    let mut modified_files: Vec<ModifiedFile> = file_map
        .into_iter()
        .map(|(file, references)| ModifiedFile { file, references })
        .collect();

    // Sort by file path for consistent output
    modified_files.sort_by(|a, b| a.file.cmp(&b.file));

    Ok(modified_files)
}

/// Get all files changed in a specific commit
fn get_changed_files(repo: &Repository, commit_oid: Oid, paths: &[String]) -> Result<Vec<PathBuf>> {
    let commit = repo
        .find_commit(commit_oid)
        .with_context(|| format!("Unable to find commit {}", commit_oid))?;

    let tree = commit.tree().context("Unable to get tree from commit")?;

    let parent_tree = if commit.parent_count() > 0 {
        Some(commit.parent(0)?.tree()?)
    } else {
        None
    };

    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(0);
    diff_opts.ignore_submodules(true);
    // Limit diff to specified paths
    for path in paths {
        diff_opts.pathspec(path);
    }

    let diff = repo
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))
        .context("Unable to create diff")?;

    let mut files = Vec::new();

    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                files.push(path.to_path_buf());
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(files)
}

/// Extract issue IDs from commit message using regex
fn extract_issues(message: &str) -> Vec<String> {
    let mut issues = Vec::new();
    let mut seen = HashSet::new();

    for cap in issue_id_regex().captures_iter(message) {
        if let Some(issue) = cap.get(0) {
            let issue_str = issue.as_str().to_owned();
            if seen.insert(issue_str.clone()) {
                issues.push(issue_str);
            }
        }
    }

    issues
}
