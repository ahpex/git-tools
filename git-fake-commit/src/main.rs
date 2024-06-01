use anyhow::Result;
use clap::Parser;
use git2::Repository;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

/// Creates dummy commits in the current branch of a git repository
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Commit message
    #[arg(short = 'm')]
    message: Option<String>,

    /// Commit author
    #[arg(short = 'a')]
    author: Option<String>,

    /// Commit email
    #[arg(short = 'e')]
    email: Option<String>,
}

fn create_fake_commit(args: CliArgs) -> Result<()> {
    let repo_path = Path::new(".");
    let repo = Repository::open(repo_path)?;

    // Path to the file to be committed
    let file_path = repo_path.join("fakefile.txt");
    let mut file = File::create(&file_path)?;
    writeln!(file, "{}", Uuid::new_v4())?;

    // Stage the file
    let mut index = repo.index()?;
    index.add_path(Path::new("fakefile.txt"))?;
    index.write()?;

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let parent_commit = repo.head().expect("Failed to get HEAD").peel_to_commit()?;

    let cmt = git_fake_commit::CommitBuilder::new()
        .message(args.message)
        .author(args.author)
        .email(args.email)
        .build()?;

    repo.commit(
        Some("HEAD"),
        &cmt.author,
        &cmt.author,
        &cmt.message,
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if let Err(err) = create_fake_commit(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
