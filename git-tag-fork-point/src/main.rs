use anyhow::{Error, Result};
use clap::Parser;
use git2::Repository;
use std::path::Path;

/// Creates a lightweight tag on the branch point
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {}

fn create_brankoff_point(_args: CliArgs) -> Result<()> {
    let repo_path = Path::new(".");
    let repo = Repository::open(repo_path)?;

    // Get the current branch
    let head = repo.head()?;
    if head.shorthand().unwrap().contains("main") {
        return Err(Error::msg("Unable to set branch point on main branch"));
    }

    // Get the commit of the current branch
    let head_commit = head.peel_to_commit()?;

    let head_main_commit = repo
        .find_branch("main", git2::BranchType::Local)?
        .into_reference()
        .peel_to_commit()
        .unwrap();

    let common_ancestor_oid = repo.merge_base(head_commit.id(), head_main_commit.id())?;
    let common_ancestor = repo.find_commit(common_ancestor_oid)?;

    // Tag the commit
    repo.tag_lightweight(
        format!("{}_forked", head.shorthand().unwrap()).as_str(),
        &common_ancestor.as_object(),
        true,
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if let Err(err) = create_brankoff_point(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
