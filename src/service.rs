use crate::branch::{BranchType, build_branch_name};
use crate::cli::HistoryFilter;
use crate::debug;
use crate::error::GitxError;
use crate::git::{git_branch, git_current_branch, git_delete};
use crate::history::{HistoryStatus, append_history, read_history};

fn record_result(
    result: Result<(), GitxError>,
    command: &str,
    target: &str,
) -> Result<(), GitxError> {
    match result {
        Ok(()) => {
            append_history(HistoryStatus::Success, command, target)?;
            Ok(())
        }
        Err(err) => {
            let _ = append_history(HistoryStatus::Fail, command, target);
            Err(err)
        }
    }
}

pub fn execute_branch_create(
    branch_type: &BranchType,
    issue: &str,
    summary: &str,
) -> Result<String, GitxError> {
    let branch_name = build_branch_name(branch_type, issue, summary);

    let result = git_branch(&branch_name);
    record_result(result, "branch", &branch_name)?;
    Ok(branch_name)
}

pub fn execute_branch_delete(branch_name: &str) -> Result<String, GitxError> {
    let current_branch_name = git_current_branch()?;

    debug!("target branch: {}", branch_name);
    debug!("current branch: {}", current_branch_name);

    // 削除対象のブランチがcurrent branchなのかを検証
    if current_branch_name == branch_name {
        return Err(GitxError::CannotDeleteCurrentBranch);
    }

    let is_protected = is_protected_branch(branch_name);

    debug!("is_protected_branch: {}", is_protected);

    if is_protected {
        return Err(GitxError::CannotDeleteProtectedBranch);
    }

    let result = git_delete(branch_name);
    record_result(result, "delete", branch_name)?;

    Ok(branch_name.to_string())
}

fn is_protected_branch(branch_name: &str) -> bool {
    const PROTECTED_BRANCHES: [&str; 3] = ["main", "master", "develop"];
    PROTECTED_BRANCHES.contains(&branch_name)
}

pub fn execute_history(
    limit: Option<usize>,
    filter: Option<HistoryFilter>,
) -> Result<String, GitxError> {
    let history_records = read_history()?;

    let iter = history_records.iter().rev();

    // 引数のfilterに値があるなら、一致するrecordのみを抽出、そうでななら、全てを返却
    let filtered = iter.filter(|record| match &filter {
        Some(history_filter) => record.command == history_filter.to_string(),
        None => true,
    });

    // limitがあるなら、その件数分のみを抽出、そうでないなら全件返却
    let limited = filtered.take(limit.unwrap_or(usize::MAX));

    let output = limited
        .map(|record| record.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let header = format!(
        "| {:<20} | {:<10} | {:<10} | {}",
        "TIMESTAMP", "STATUS", "COMMAND", "TARGET"
    );

    let output = format!("{}\n{}", header, output);

    Ok(output)
}
