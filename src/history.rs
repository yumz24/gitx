use crate::error::GitxError;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn append_history(command: &str, target: &str) -> Result<(), GitxError> {
    // 1. 環境変数 "HOME" を取得
    let home_dir = env::var("HOME").map_err(|e| GitxError::HistoryFailed(e.to_string()))?;
    let dir_path = PathBuf::from(home_dir).join(".gitx");

    std::fs::create_dir_all(&dir_path).map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    let file_path = dir_path.join("history.log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    writeln!(file, "{} {}", command, target)
        .map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    Ok(())
}
