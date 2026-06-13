use crate::branch::BranchType;
use crate::error::GitxError;
use core::fmt;
use std::env::Args;

pub struct BranchArgs {
    pub branch_type: BranchType,
    pub issue: String,
    pub summary: String,
}

pub struct DeleteArgs {
    pub branch_name: String,
}

pub struct ExecuteHistoryArgs {
    pub limit: Option<usize>,
    pub filter: Option<HistoryFilter>,
}

#[derive(Debug)]
pub enum Command {
    Branch,  // ブランチ作成とチェックアウト
    Delete,  // ブランチ削除
    History, // gitxコマンドを実行した履歴を表示
}

#[derive(Debug)]
pub enum HistoryFilter {
    Branch,
    Delete,
}

// 文字列からenumへの変換処理
impl std::str::FromStr for Command {
    type Err = GitxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "branch" => Ok(Command::Branch),
            "delete" => Ok(Command::Delete),
            "history" => Ok(Command::History),
            _ => Err(GitxError::InvalidCommand),
        }
    }
}

impl std::str::FromStr for HistoryFilter {
    type Err = GitxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "branch" => Ok(HistoryFilter::Branch),
            "delete" => Ok(HistoryFilter::Delete),
            _ => Err(GitxError::InvalidHistoryFilter),
        }
    }
}

impl fmt::Display for HistoryFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HistoryFilter::Branch => write!(f, "branch"),
            HistoryFilter::Delete => write!(f, "delete"),
        }
    }
}

pub fn parse_command(args: &mut Args) -> Result<Command, GitxError> {
    let raw_command = args.next().ok_or(GitxError::InvalidCommand)?;
    raw_command.parse()
}

pub fn parse_branch_args(args: &mut Args) -> Result<BranchArgs, GitxError> {
    let raw_branch_type = args.next().ok_or(GitxError::InvalidBranchArgs)?;
    let branch_type = raw_branch_type.parse()?;

    let issue = args.next().ok_or(GitxError::InvalidBranchArgs)?;
    let summary = args.next().ok_or(GitxError::InvalidBranchArgs)?;

    // 3つ目の引数がある場合はエラー
    if args.next().is_some() {
        return Err(GitxError::InvalidBranchArgs);
    };

    Ok(BranchArgs {
        branch_type,
        issue,
        summary,
    })
}

pub fn parse_delete_args(args: &mut Args) -> Result<DeleteArgs, GitxError> {
    let raw_branch_name = args.next().ok_or(GitxError::InvalidDeleteArgs)?;
    let branch_name = raw_branch_name.to_string();

    // 2つ目の引数がある場合はエラー
    if args.next().is_some() {
        return Err(GitxError::InvalidDeleteArgs);
    };

    Ok(DeleteArgs { branch_name })
}

pub fn parse_execute_history(args: &mut Args) -> Result<ExecuteHistoryArgs, GitxError> {
    let mut limit: Option<usize> = None;
    let mut filter: Option<HistoryFilter> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--limit" | "-l" => {
                // --limitの次の値があり、数値変換可能な値かを検証する
                let raw = args.next().ok_or(GitxError::MissingLimitValue)?;

                if limit.is_some() {
                    // すでにlimit変数に値が格納されている場合
                    return Err(GitxError::DuplicateHistoryOption(arg.to_string()));
                }

                limit = Some(
                    raw.parse::<usize>()
                        .map_err(|_| GitxError::InvalidLimitValue)?,
                );
            }
            "--filter" | "-f" => {
                // filterの検証
                let raw = args.next().ok_or(GitxError::MissingHistoryFilterValue)?;

                if filter.is_some() {
                    // すでにfilter変数に値が格納されている場合
                    return Err(GitxError::DuplicateHistoryOption(arg.to_string()));
                }

                filter = Some(
                    raw.parse::<HistoryFilter>()
                        .map_err(|_| GitxError::InvalidHistoryFilter)?,
                )
            }
            _ => {
                if arg.starts_with("-") {
                    return Err(GitxError::UnknownHistoryOption(arg.to_string()));
                } else {
                    return Err(GitxError::UnknownHistoryArguments(arg.to_string()));
                }
            }
        }
    }

    Ok(ExecuteHistoryArgs { limit, filter })
}
