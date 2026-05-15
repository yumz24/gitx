use std::env::Args;

use crate::branch::BranchType;
use crate::error::GitxError;

pub struct CliArgs {
    pub command: Command,
    pub branch_type: BranchType,
    pub issue: String,
    pub summary: String,
}

#[derive(Debug)]
pub enum Command {
    Branch, // ブランチ作成とチェックアウト
}

// 文字列からenumへの変換処理
impl std::str::FromStr for Command {
    type Err = GitxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "branch" => Ok(Command::Branch),
            _ => Err(GitxError::InvalidArgs),
        }
    }
}

pub fn parse_args(args: Args) -> Result<CliArgs, GitxError> {
    let mut v_iter = args.into_iter();

    let _program = v_iter.next();

    let raw_command = v_iter.next().ok_or(GitxError::InvalidArgs)?;
    let command = raw_command.parse()?;

    let raw_branch_type = v_iter.next().ok_or(GitxError::InvalidArgs)?;
    let branch_type = raw_branch_type.parse()?;

    let issue = v_iter.next().ok_or(GitxError::InvalidArgs)?;
    let summary = v_iter.next().ok_or(GitxError::InvalidArgs)?;

    // 5つ目の引数がある場合はエラー
    if v_iter.next().is_some() {
        return Err(GitxError::InvalidArgs);
    };

    Ok(CliArgs {
        command,
        branch_type,
        issue,
        summary,
    })
}
