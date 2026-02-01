use anyhow::{bail, Context, Result};
use clap::Parser;
use git2::{Repository, Signature};
use git_tools_lib::RepositoryExt;

/// Git rebase tool that squashes the last N commits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of commits to squash
    #[arg(short, long, required = true)]
    count: usize,

    /// Path to the repository
    #[arg(short, long, default_value = ".")]
    repo: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get the repository path from args
    let repo_path = &args.repo;
    let repo = Repository::open(repo_path)?;

    preflight_checks(&repo)?;

    // Get the last N commits
    println!("Before squashing:");
    print_commits(&repo)?;

    // Perform rebase and squash
    squash_last_commits(&repo, args.count)?;

    println!("\nAfter squashing:");
    print_commits(&repo)?;

    Ok(())
}

/// Performs preflight checks to ensure the repository is in a clean state
fn preflight_checks(repo: &Repository) -> Result<()> {
    if repo.state() != git2::RepositoryState::Clean {
        bail!("Repository is in a non-clean state. Please complete or abort running actions (rebase, merge, ...) first.");
    }

    let index_has_staged_changes = repo.has_staged_changes()?;
    if index_has_staged_changes {
        bail!("You have staged changes. Please commit or stash them before rebasing.");
    }

    Ok(())
}

fn print_commits(repo: &Repository) -> Result<()> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut count = 0;
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        println!(
            " {} - ({})",
            &oid.to_string()[..8],
            commit.author().name().unwrap_or("Unknown")
        );
        count += 1;
        if count >= 5 {
            break;
        }
    }

    Ok(())
}

fn squash_last_commits(repo: &Repository, count: usize) -> Result<()> {
    println!("\nSquashing last {} commits...", count);

    // Get the current HEAD commit
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    // Get the commit that is 'count' commits before HEAD
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut target_oid = head_commit.id();
    let mut commits = vec![];

    // Collect all commit messages for squashed commit
    for (i, oid) in revwalk.enumerate() {
        let oid = oid?;
        if i < count {
            commits.push(oid);
        }
        if i == count {
            target_oid = oid;
            break;
        }
    }

    commits.reverse();

    if commits.len() < count {
        println!(
            "Not enough commits to squash. Found: {}, Requested: {}",
            commits.len(),
            count
        );
        return Ok(());
    }

    // Check that there is no rebase or merge in progress
    // check that working directory is clean
    // check for clean status, because we don't want to mess with uncommitted changes

    // Get the base commit (the one we'll rebase onto)
    let base_commit = repo.find_commit(target_oid)?;

    println!(
        " Base commit: {} - {}",
        &target_oid.to_string()[..8],
        base_commit.message().unwrap_or("").trim()
    );
    println!(" Squashing {} commits onto base...", commits.len());

    let sig = Signature::now("Rebase User", "rebase@example.com")?;

    // Now we need to squash: reset to base and create single commit with final tree
    // Get the current HEAD tree (which has all the changes from all commits)
    let new_head = repo.head()?.peel_to_commit()?;
    let tree = new_head.tree()?;

    let squashed_msg = commits
        .iter()
        .map(|oid| -> Result<String> {
            let commit = repo
                .find_commit(*oid)
                .with_context(|| format!("Failed to find commit {}", oid))?;
            Ok(commit.message().unwrap_or("").trim().to_owned())
        })
        .collect::<Result<Vec<_>>>()?
        .join("\n\n");

    // Reset HEAD to base commit
    repo.reset(base_commit.as_object(), git2::ResetType::Soft, None)?;

    // Create the squashed commit
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &squashed_msg,
        &tree,
        &[&base_commit],
    )?;

    Ok(())
}
