use anyhow::{Context, Result};
use clap::Parser;
use git2::{BranchType, Repository};
use inquire::Select;
use std::path::PathBuf;

/// Shows recent branches and allows interactive selection to switch to
#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// Path to repository
    #[arg(default_value = ".")]
    repo: PathBuf,
}

struct BranchInfo {
    name: String,
    commit_time: i64,
}

fn get_recent_branches(repo: &Repository) -> Result<Vec<String>> {
    let mut branch_infos = Vec::new();

    // Get all local branches
    let branches = repo
        .branches(Some(BranchType::Local))
        .context("Unable to list branches")?;

    for branch_result in branches {
        let (branch, _) = branch_result.context("Unable to read branch")?;

        if let Some(branch_name) = branch.name().context("Invalid branch name")? {
            // Get the commit for this branch
            let reference = branch.get();
            if let Ok(commit) = reference.peel_to_commit() {
                branch_infos.push(BranchInfo {
                    name: branch_name.to_owned(),
                    commit_time: commit.time().seconds(),
                });
            }
        }
    }

    // Sort by commit time (most recent first)
    branch_infos.sort_by(|a, b| b.commit_time.cmp(&a.commit_time));

    // Take top 5
    Ok(branch_infos.into_iter().map(|b| b.name).collect())
}

fn checkout_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    let (object, reference) = repo
        .revparse_ext(branch_name)
        .with_context(|| format!("Unable to find branch '{}'", branch_name))?;

    repo.checkout_tree(&object, None)
        .with_context(|| format!("Unable to checkout branch '{}'", branch_name))?;

    match reference {
        Some(reference) => {
            let ref_name = reference.name().context("Unable to get reference name")?;
            repo.set_head(ref_name).context("Unable to set HEAD")?;
        }
        None => {
            repo.set_head_detached(object.id())
                .context("Unable to set detached HEAD")?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let repo = Repository::open(&args.repo)
        .with_context(|| format!("Unable to open git repository at '{}'", args.repo.display()))?;

    let recent_branches = get_recent_branches(&repo)?;

    if recent_branches.is_empty() {
        println!("No recent branches found in reflog");
        return Ok(());
    }

    let selection = Select::new("Select a branch to switch to:", recent_branches)
        .prompt()
        .context("Selection cancelled")?;

    checkout_branch(&repo, &selection)?;

    println!("Switched to branch '{}'", selection);

    Ok(())
}
