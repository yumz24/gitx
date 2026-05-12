use std::process::Command;

// Result<GitOutput, GitError>
pub fn git_branch(branch_name: String) -> Result<String, String> {
    let output = Command::new("git")
        .args(["checkout", "-b", &branch_name.to_string()])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(branch_name)
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        Err(s.to_string())
    }
}
