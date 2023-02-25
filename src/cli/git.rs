use std::error::Error;
use std::process::Command;

use crate::cli::output;

pub fn execute(path: &str, command: String) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .args(["-C", path])
        .args(command.split(' '))
        .output()?;

    if !output.stdout.is_empty() {
        output::git(String::from_utf8(output.stdout).unwrap());
    };
    if !output.stderr.is_empty() {
        output::error(String::from_utf8(output.stderr).unwrap());
    };

    Ok(())
}

pub fn sync(repo_path: &str, remote: String) -> Result<(), Box<dyn Error>> {
    execute(
        repo_path,
        format!("pull --ff --no-rebase --no-edit --commit {remote}"),
    )?;
    execute(repo_path, format!("push {remote}"))?;

    Ok(())
}
