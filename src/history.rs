use crate::error::GitxError;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn history_file_path() -> Result<PathBuf, GitxError> {
    let home_dir = env::var("HOME").map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    Ok(PathBuf::from(home_dir).join(".gitx").join("history.log"))
}

pub fn append_history(command: &str, target: &str) -> Result<(), GitxError> {
    let file_path = history_file_path()?;

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| GitxError::HistoryFailed(e.to_string()))?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    writeln!(file, "{} {}", command, target)
        .map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    Ok(())
}

pub fn read_history() -> Result<String, GitxError> {
    let file_path = history_file_path()?;

    let content =
        std::fs::read_to_string(file_path).map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    Ok(content)
}
