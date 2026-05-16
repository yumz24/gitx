use std::env::Args;

use crate::branch::BranchType;
use crate::error::GitxError;

pub struct BranchArgs {
    pub branch_type: BranchType,
    pub issue: String,
    pub summary: String,
}

pub struct DeleteArgs {
    pub branch_name: String,
}

#[derive(Debug)]
pub enum Command {
    Branch, // ブランチ作成とチェックアウト
    Delete, // ブランチ削除
}

// 文字列からenumへの変換処理
impl std::str::FromStr for Command {
    type Err = GitxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "branch" => Ok(Command::Branch),
            "delete" => Ok(Command::Delete),
            _ => Err(GitxError::InvalidCommand),
        }
    }
}

pub fn parse_command(args: &mut Args) -> Result<Command, GitxError> {
    let raw_command = args.next().ok_or(GitxError::InvalidCommand)?;

    Ok(raw_command.parse()?)
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
