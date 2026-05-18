use crate::debug;
use crate::error::GitxError;
use std::process::{Command, Output};

fn run_git(args: &[&str]) -> Result<Output, GitxError> {
    debug!("git {:?}", args);

    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| GitxError::GitCommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(output)
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        Err(GitxError::GitCommandFailed(s.to_string()))
    }
}

pub fn git_branch(branch_name: &str) -> Result<(), GitxError> {
    run_git(&["checkout", "-b", branch_name])?;
    Ok(())
}

pub fn git_delete(branch_name: &str) -> Result<(), GitxError> {
    run_git(&["branch", "-d", branch_name])?;
    Ok(())
}

pub fn git_current_branch() -> Result<String, GitxError> {
    let output = run_git(&["branch", "--show-current"])?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}
