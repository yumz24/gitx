use crate::error::GitxError;
use std::fmt;

#[derive(Debug)]
pub enum BranchType {
    Feat,     // 新機能の追加
    Fix,      // バグ修正
    Docs,     // ドキュメントの変更
    Style,    // コードのスタイル変更（動作には影響なし）
    Refactor, // リファクタリング（動作の変更なし）
    Perf,     // パフォーマンス改善
    Test,     // テストの追加や修正
    Chore,    // 雑多な作業（依存関係の更新等）
}

// 文字列からenumへの変換処理
impl std::str::FromStr for BranchType {
    type Err = GitxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "feat" => Ok(BranchType::Feat),
            "fix" => Ok(BranchType::Fix),
            "docs" => Ok(BranchType::Docs),
            "style" => Ok(BranchType::Style),
            "refactor" => Ok(BranchType::Refactor),
            "perf" => Ok(BranchType::Perf),
            "test" => Ok(BranchType::Test),
            "chore" => Ok(BranchType::Chore),
            _ => Err(GitxError::InvalidArgs),
        }
    }
}

// enumから文字列への変換処理
impl fmt::Display for BranchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchType::Feat => write!(f, "feat"),
            BranchType::Fix => write!(f, "fix"),
            BranchType::Docs => write!(f, "docs"),
            BranchType::Style => write!(f, "style"),
            BranchType::Refactor => write!(f, "refactor"),
            BranchType::Perf => write!(f, "perf"),
            BranchType::Chore => write!(f, "chore"),
            BranchType::Test => write!(f, "test"),
        }
    }
}

pub fn build_branch_name(branch_type: &BranchType, issue: &str, summary: &str) -> String {
    format!("{}/#{}-{}", branch_type, issue, summary)
}
