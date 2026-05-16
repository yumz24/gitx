use crate::error::GitxError;
use std::process::Command;

fn execute_git(args: &[&str]) -> Result<(), GitxError> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| GitxError::GitCommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        Err(GitxError::GitCommandFailed(s.to_string()))
    }
}

pub fn git_branch(branch_name: &str) -> Result<(), GitxError> {
    execute_git(&["checkout", "-b", branch_name])
}

pub fn git_delete(branch_name: &str) -> Result<(), GitxError> {
    execute_git(&["branch", "-d", branch_name])
}
