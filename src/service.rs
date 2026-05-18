use crate::branch::BranchType;
use crate::branch::build_branch_name;
use crate::error::GitxError;
use crate::git::git_branch;
use crate::git::git_current_branch;
use crate::git::git_delete;
use crate::logger::debug;

pub fn execute_branch_create(
    branch_type: &BranchType,
    issue: &str,
    summary: &str,
) -> Result<String, GitxError> {
    let branch_name: String = build_branch_name(branch_type, issue, summary);

    git_branch(&branch_name)?;

    Ok(branch_name)
}

pub fn execute_branch_delete(branch_name: &str) -> Result<String, GitxError> {
    let current_branch_name = git_current_branch()?;

    debug(&format!("target branch: {}", branch_name));
    debug(&format!("current branch: {}", current_branch_name));

    // 削除対象のブランチがcurrent buranchなのかを検証
    if current_branch_name == branch_name {
        return Err(GitxError::CannotDeleteCurrentBranch);
    }

    let is_protected = is_protected_branch(branch_name);

    debug(&format!("is_protected_branch: {}", is_protected));

    if is_protected {
        return Err(GitxError::CannotDeleteProtectedBranch);
    }

    git_delete(branch_name)?;

    Ok(branch_name.to_string())
}

fn is_protected_branch(branch_name: &str) -> bool {
    const PROTECTED_BRANCHES: [&str; 2] = ["main", "master"];
    PROTECTED_BRANCHES.contains(&branch_name)
}
