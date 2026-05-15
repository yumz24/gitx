use crate::error::GitxError;
use std::process::Command;

pub fn git_branch(branch_name: &str) -> Result<(), GitxError> {
    let output = Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()
        .map_err(|e| GitxError::GitCommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        Err(GitxError::GitCommandFailed(s.to_string()))
    }
}
