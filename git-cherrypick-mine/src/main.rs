use anyhow::{Context, Result};
use clap::Parser;
use git2::{BranchType, Commit, Repository};
use git_tools_lib::SignatureExt;
use nu_ansi_term::Color;
use std::path::PathBuf;

/// Cherry-picks commits by the current user from the current branch
#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// Path to repository
    #[arg(short = 'r', long, default_value = ".")]
    repo: PathBuf,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let repo = Repository::open(&args.repo)
        .with_context(|| format!("Unable to open git repository at '{:?}'", args.repo))?;

    // Get current branch (HEAD)
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    let current_branch_name = head
        .shorthand()
        .context("Unable to get current branch name")?;

    // Get the base branch commit
    let base_branch = repo
        .find_branch("main", BranchType::Local)
        .with_context(|| format!("Unable to find base branch '{}'", "main"))?;

    let base_commit = base_branch.get().peel_to_commit()?;

    // Find merge base between HEAD and base branch
    let merge_base = repo.merge_base(head_commit.id(), base_commit.id())?;

    // Walk commits from current branch back to merge base
    let mut revwalk = repo.revwalk()?;
    revwalk.push(head_commit.id())?;
    revwalk.hide(merge_base)?;

    // The current user signature taken from .gitconfig
    let its_me = repo.signature()?;

    let mut my_commits: Vec<Commit<'_>> = revwalk
        .filter_map(Result::ok)
        .filter_map(|oid| repo.find_commit(oid).ok())
        .filter(|commit_opt| its_me.is_same_identity(&commit_opt.author()))
        .collect();

    if my_commits.is_empty() {
        println!(
            "{}",
            Color::Yellow.paint("No commits by you found on the source branch")
        );
        return Ok(());
    }

    // Reverse to get commits in chronological order
    my_commits.reverse();

    println!(
        "{} {} commit(s) from branch {}:",
        Color::Cyan.paint("Would cherry-pick"),
        my_commits.len(),
        Color::Blue.paint(current_branch_name)
    );

    for commit in &my_commits {
        let message = commit.message().unwrap_or("").lines().next().unwrap_or("");
        println!(
            "  {} {} {}",
            Color::Yellow.paint(format!("{:.7}", commit.id())),
            message,
            Color::DarkGray.paint(commit.author().name().unwrap_or("").to_owned()),
        );
    }

    println!(
        "git checkout main && git cherry-pick -x {}",
        my_commits
            .iter()
            .map(|commit| format!("{:.7}", commit.id()))
            .collect::<Vec<_>>()
            .join(" ")
    );

    Ok(())
}
