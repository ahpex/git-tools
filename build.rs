#![allow(
    clippy::expect_used,
    reason = "Build script, simple error handling is sufficient"
)]

use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Run `git log` to get the latest commit message
    let output = Command::new("git")
        .args(["rev-list", "--max-count=1", "HEAD"])
        .output()
        .expect("Failed to execute Git command");

    let commit_msg = String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence");

    // Write the commit message to a file that can be included in the build
    let mut file = File::create("../target/git_commit.txt").expect("Could not create file");
    write!(file, "{}", commit_msg.trim()).expect("Could not write to file");
}
