use crate::debug;
use crate::error::GitxError;
use std::process::Command;

pub struct GitOutput {
    pub stdout: String,
}

pub struct GitClient;

impl GitClient {
    pub fn run(&self, args: &[&str]) -> Result<GitOutput, GitxError> {
        debug!("git {:?}", args);

        let command = format!("git {}", args.join(" "));

        let output =
            Command::new("git")
                .args(args)
                .output()
                .map_err(|e| GitxError::GitCommandFailed {
                    command: command.clone(),
                    stderr: e.to_string(),
                })?;

        if output.status.success() {
            Ok(GitOutput {
                stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
            })
        } else {
            let s = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(GitxError::GitCommandFailed { command, stderr: s })
        }
    }
}

pub fn git_branch(branch_name: &str) -> Result<(), GitxError> {
    GitClient::run(&["checkout", "-b", branch_name])?;
    Ok(())
}

pub fn git_delete(branch_name: &str) -> Result<(), GitxError> {
    GitClient::run(&["branch", "-d", branch_name])?;
    Ok(())
}

pub fn git_current_branch() -> Result<String, GitxError> {
    let output = GitClient::run(&["branch", "--show-current"])?;
    Ok(output.stdout)
}
