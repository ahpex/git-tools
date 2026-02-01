use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use git2::Repository;
use git_tools_lib::IssueNumber;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-tool")]
#[command(about = "A unified git utility tool with multiple subcommands", long_about = None)]
struct Cli {
    /// Path to the git repository
    #[arg(short, long, default_value = ".")]
    repo: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Commands related to branch operations
    Branch {
        #[command(subcommand)]
        action: Option<BranchAction>,

        /// Extract and print the issue number from the current branch name
        #[arg(long)]
        issue: bool,
    },
}

#[derive(Subcommand)]
enum BranchAction {
    /// List branches (placeholder for future functionality)
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let repo = Repository::open(&cli.repo)
        .with_context(|| format!("Unable to open git repository at '{:?}'", cli.repo))?;

    match cli.command {
        Commands::Branch { action, issue } => {
            if issue {
                handle_branch_issue(&repo)?;
            } else if let Some(action) = action {
                match action {
                    BranchAction::List => {
                        handle_branch_list(&repo)?;
                    }
                }
            } else {
                // Default behavior: show current branch name
                let branch_name = get_current_branch_name(&repo)?;
                println!("{}", branch_name);
            }
        }
    }

    Ok(())
}

/// Get the current branch name
fn get_current_branch_name(repo: &Repository) -> Result<String> {
    let head = repo.head().context("Failed to get HEAD reference")?;
    let branch_name = head
        .shorthand()
        .context("Failed to get branch name")?
        .to_string();
    Ok(branch_name)
}

/// Handle the `branch --issue` command
fn handle_branch_issue(repo: &Repository) -> Result<()> {
    let branch_name = get_current_branch_name(repo)?;

    match IssueNumber::parse(&branch_name) {
        Some(issue) => {
            println!("{}", issue.as_str());
        }
        None => {
            eprintln!("No issue number found in branch name: {}", branch_name);
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle the `branch list` command
fn handle_branch_list(repo: &Repository) -> Result<()> {
    let branches = repo
        .branches(Some(git2::BranchType::Local))
        .context("Failed to list branches")?;

    for branch in branches {
        let (branch, _) = branch.context("Failed to get branch")?;
        if let Some(name) = branch.name()? {
            println!("{}", name);
        }
    }

    Ok(())
}
