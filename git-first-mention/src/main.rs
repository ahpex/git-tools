use anyhow::Result;
use clap::Parser;
use colored::*;
use git2::{Commit, DiffDelta, DiffOptions, Repository, Sort};
use humantime::{format_duration, format_rfc3339_seconds};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Find commits where a string first appears
#[derive(Parser, Debug)]
#[command(version = include_str!("../../target/git_commit.txt").trim(), about, long_about = None)]
struct CliArgs {
    /// search for string
    #[arg(short = 's')]
    needle: String,
}

fn first_mention(args: CliArgs) -> Result<()> {
    let repo_path = Path::new(".");
    let repo = Repository::open(repo_path)?;

    // Configure the revwalk (commit iterator)
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::REVERSE)?;
    revwalk.simplify_first_parent()?;

    let mut found = false;
    let needle = args.needle.as_bytes();

    let mut diff_options = DiffOptions::new();

    // Iterate over the commits
    for commit_id in revwalk {
        let oid = commit_id?;
        let commit = repo.find_commit(oid)?;
        let tree = commit.tree()?;

        // Get the parent commit tree for the diff (skip if no parents, i.e., initial commit)
        if let Ok(parent) = commit.parent(0) {
            let parent_tree = parent.tree()?;

            let diff =
                repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_options))?;

            // Check each delta in the diff for the search string
            if let Err(err) = diff.foreach(
                &mut |_, _| true,
                None,
                None,
                Some(&mut |delta, _, diffline| {
                    if diffline.origin() == '+'
                        && diffline
                            .content()
                            .windows(needle.len())
                            .any(|w| w == needle)
                    // String::from_utf8_lossy(diffline.content()).contains(needle)
                    {
                        print_info(&commit, &delta);
                        found = true;
                    }
                    !found // Stop iterating if false
                }),
            ) {
                // Callbacks can return an error to stop the iteration, but be only care about
                // the error code not being a user error
                if err.code() != git2::ErrorCode::User {
                    eprintln!("{}", err);
                }
            };
        }

        if found {
            break;
        }
    }

    Ok(())
}

fn print_info(commit: &Commit<'_>, delta: &DiffDelta<'_>) {
    let commit_time = UNIX_EPOCH + Duration::from_secs(commit.time().seconds() as u64);
    let now = SystemTime::now();
    let duration = now
        .duration_since(commit_time)
        .unwrap_or(Duration::new(0, 0));

    println!("commit:  {}", commit.id().to_string().cyan());
    println!(
        "date:    {}, {} ago ",
        format_rfc3339_seconds(commit_time).to_string().magenta(),
        format_duration(duration).to_string().magenta()
    );
    println!(
        "author:  {}",
        commit
            .author()
            .name()
            .unwrap_or("Unknown")
            .to_owned()
            .green()
    );
    println!(
        "file:    {}",
        delta
            .new_file()
            .path()
            .map_or_else(|| "No file".into(), |p| p.to_string_lossy().blue())
    );
    println!();
    println!("{}", commit.message().unwrap_or("No message"));
    println!();
    print!("For more information run: git show {}", commit.id());
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if let Err(err) = first_mention(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
