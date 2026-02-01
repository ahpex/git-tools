use anyhow::{Context, Result};
use clap::Parser;
use git2::{BranchType, Repository};
use git_tools_lib::{is_bot_commit, IssueNumber};
use nu_ansi_term::Color;
use std::{collections::HashSet, path::PathBuf};

/// Shows stale git branches without commits for n days
#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// Path to repository
    #[arg(default_value = ".")]
    repo: PathBuf,

    /// Be verbose
    #[arg(short = 'v')]
    verbose: bool,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct IssueInProgress {
    name: String,
    author: String,
    issue: Option<IssueNumber>,
}

impl std::fmt::Display for IssueInProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            Color::LightBlue.paint(&self.name),
            self.issue
                .as_ref()
                .map_or_else(|| "<no-issue>", |issue| issue.as_str()),
            Color::LightGreen.paint(&self.author),
        )
    }
}

fn issues_in_progress(repo: &Repository, _cli_args: &CliArgs) -> Result<()> {
    let branches = repo.branches(Some(BranchType::Remote))?;

    let branch_summary = collect_branch_summary(repo, branches);

    for branch in branch_summary.iter().flatten() {
        println!("{}", branch);
    }

    Ok(())
}

fn collect_branch_summary(
    repo: &Repository,
    branches: git2::Branches<'_>,
) -> Vec<HashSet<IssueInProgress>> {
    let branch_summary: Vec<HashSet<IssueInProgress>> = branches
        //.filter(|predicate| predicate.as_ref().is_ok_and(|x| x.0.name().unwrap() == Some("origin/main")))
        .filter_map(|one_branch| one_branch.ok())
        .filter_map(|(branch, _)| {
            let mut revwalk = repo.revwalk().ok()?;
            let commit = branch.get().peel_to_commit().ok()?;
            revwalk.push(commit.id()).ok()?;
            revwalk.simplify_first_parent().ok()?;

            let branch_name = branch.name().ok().flatten()?.to_owned();

            let commits = revwalk
                .filter_map(|commit_id| commit_id.ok())
                .take_while(|oid| {
                    repo.find_commit(*oid)
                        .map(|commit| is_commit_in_range(&commit, 3))
                        .unwrap_or(false)
                })
                .filter(|oid| !is_bot_commit(repo, oid))
                .filter_map(|commit_id| repo.find_commit(commit_id).ok())
                .filter_map(|commit| {
                    Some(IssueInProgress {
                        name: branch_name.clone(),
                        author: commit.author().name()?.to_owned(),
                        issue: IssueNumber::parse(commit.message()?),
                    })
                })
                .collect::<HashSet<_>>();

            Some(commits)
        })
        .collect();
    branch_summary
}

fn is_commit_in_range(commit: &git2::Commit<'_>, days: i64) -> bool {
    let Some(commit_time) = chrono::DateTime::from_timestamp(commit.time().seconds(), 0) else {
        return true; // Returning true here is safer, than false, to avoid missing recent commits
    };
    (chrono::Utc::now() - commit_time).num_days() < days
}

/// Treat commits authored by these names as bot commits and filter them out.
///
/// Parameters:
/// - `repo`: The git repository    
/// - `oid`: The commit OID to check
///
/// Returns:
/// - `true` if the commit is authored by a human, `false` if by a bot
fn main() -> Result<()> {
    let args = CliArgs::parse();

    let repo = Repository::open(&args.repo)
        .with_context(|| format!("Unable to open git repository at '{:?}'", args.repo))?;

    issues_in_progress(&repo, &args)
}
