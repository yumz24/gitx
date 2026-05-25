use crate::error::GitxError;
use chrono::Local;
use std::env;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub enum HistoryStatus {
    Success,
    Fail,
}

struct HistoryRecord {
    timestamp: String, // RFC3339
    status: HistoryStatus,
    command: String,
    target: String,
}

// enumから文字列への変換処理
impl fmt::Display for HistoryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HistoryStatus::Success => write!(f, "success"),
            HistoryStatus::Fail => write!(f, "fail"),
        }
    }
}

impl HistoryRecord {
    fn new(status: HistoryStatus, command: &str, target: &str) -> Self {
        let timestamp = Local::now().to_rfc3339();

        HistoryRecord {
            timestamp,
            status,
            command: command.to_string(),
            target: target.to_string(),
        }
    }
}

impl fmt::Display for HistoryRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.timestamp, self.status, self.command, self.target
        )
    }
}

fn history_file_path() -> Result<PathBuf, GitxError> {
    let home_dir = env::var("HOME").map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    Ok(PathBuf::from(home_dir).join(".gitx").join("history.log"))
}

pub fn append_history(
    history_status: HistoryStatus,
    command: &str,
    target: &str,
) -> Result<(), GitxError> {
    let file_path = history_file_path()?;

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| GitxError::HistoryFailed(e.to_string()))?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    let record = HistoryRecord::new(history_status, command, target);

    writeln!(file, "{}", record).map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    Ok(())
}

pub fn read_history() -> Result<String, GitxError> {
    let file_path = history_file_path()?;

    let content =
        std::fs::read_to_string(file_path).map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    Ok(content)
}
