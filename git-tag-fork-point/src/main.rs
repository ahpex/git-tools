use anyhow::{Context, Error, Result};
use clap::Parser;
use git2::Repository;
use std::path::Path;

/// Creates a lightweight tag on the branch point
#[derive(Parser, Debug)]
#[command(version = include_str!("../../target/git_commit.txt").trim(), about, long_about = None)]
struct CliArgs {
    /// Tag name to use, defaults to 'fork-point/&lt;branch-name&gt;'
    #[arg(short, long)]
    tag: Option<String>,
}

fn generate_fork_tag(args: CliArgs) -> Result<()> {
    let repo_path = Path::new(".");
    let repo = Repository::open(repo_path)?;

    // Get the current branch
    let head = repo.head()?;
    if head.shorthand().unwrap_or_default().contains("main") {
        return Err(Error::msg("Unable to set branch point on main branch"));
    }

    let branch_name = head.shorthand().unwrap_or_default();
    let tag_name = args
        .tag
        .unwrap_or_else(|| format!("fork-point/{}", branch_name));

    // Get the commit of the current branch
    let head_commit = head.peel_to_commit()?;

    let head_main_commit = repo
        .find_branch("main", git2::BranchType::Local)?
        .into_reference()
        .peel_to_commit()
        .context("Failed to peel main branch to commit")?;

    let common_ancestor_oid = repo.merge_base(head_commit.id(), head_main_commit.id())?;
    let common_ancestor = repo.find_commit(common_ancestor_oid)?;

    // Tag the commit
    repo.tag_lightweight(&tag_name, common_ancestor.as_object(), true)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if let Err(err) = generate_fork_tag(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
