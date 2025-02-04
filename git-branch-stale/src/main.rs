use anyhow::{Context, Ok, Result};
use clap::Parser;
use git2::{BranchType, Repository};
use nu_ansi_term::Color;
use std::path::PathBuf;

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

    /// Show only remote branches
    #[arg(short = 'r')]
    remote: bool,
}

struct StaleBranch {
    name: String,
    days_without_commit: i64,
    author: String,
    sha: String,
}

fn show_stale_branches(repo: &Repository, cli_args: &CliArgs) -> Result<()> {
    let branch_filter = match cli_args.remote {
        true => Some(BranchType::Remote),
        false => None,
    };

    let branches = match repo.branches(branch_filter) {
        std::result::Result::Ok(branches) => branches,
        Err(e) => panic!("branch error: {}", e),
    };

    let mut branch_summary: Vec<StaleBranch> = branches.map(|one_branch| {
        let branch = match one_branch {
            std::result::Result::Ok(val) => val.0,
            Err(e) => panic!("branch error: {}", e),
        };

        let repo_object = repo
            .revparse_single(branch.name().unwrap().unwrap())
            .unwrap();
        let last_commit = match repo_object.as_commit() {
            Some(c) => c,
            None => panic!("is no commit"),
        };

        let commit_time =
            chrono::DateTime::from_timestamp(last_commit.time().seconds(), 0).unwrap();
        let days_without_commit = (chrono::Utc::now() - commit_time).num_days();

        let id = repo_object.short_id().unwrap();
        let sha = std::str::from_utf8(&*id).unwrap();

        StaleBranch {
            name: branch.name().unwrap().unwrap().to_string(),
            days_without_commit,
            author: last_commit.to_owned().author().name().unwrap().to_string(),
            sha: sha.to_string(),
        }
    }).collect();
    branch_summary.sort_by_key(|b| b.days_without_commit);
    branch_summary.reverse();

    for branch in branch_summary {
        if cli_args.verbose {
            println!(
                "{} stale for {} days - {} {}",
                Color::LightBlue.paint(branch.name),
                Color::LightYellow.paint(branch.days_without_commit.to_string()),
                Color::LightGreen.paint(branch.author),
                Color::LightGray.paint(branch.sha),
            );
        } else {
            println!(
                "{} {}",
                Color::LightYellow.paint(branch.days_without_commit.to_string()),
                Color::LightBlue.paint(branch.name),
            );
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let repo = Repository::open(&args.repo)
        .with_context(|| format!("Unable to open git repository at '{}'", args.repo.display()))?;

    show_stale_branches(&repo, &args)?;

    Ok(())
}
