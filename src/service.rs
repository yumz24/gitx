use crate::branch::BranchType;
use crate::error::GitxError;
use crate::git::git_delete;
use crate::{branch::build_branch_name, git::git_branch};

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
    git_delete(branch_name)?;

    Ok(branch_name.to_string())
}
