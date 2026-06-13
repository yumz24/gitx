use crate::error::GitxError;
use chrono::DateTime;
use chrono::Local;
use std::env;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

pub enum HistoryStatus {
    Success,
    Fail,
}

pub struct HistoryRecord {
    pub timestamp: String, // RFC3339
    pub status: HistoryStatus,
    pub command: String,
    pub target: String,
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
// 文字列からenumへの変換処理
impl std::str::FromStr for HistoryStatus {
    type Err = GitxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "success" => Ok(HistoryStatus::Success),
            "fail" => Ok(HistoryStatus::Fail),
            _ => Err(GitxError::HistoryFailed(format!(
                "invalid history status: {}",
                s
            ))),
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

    fn from_line(line: &str) -> Option<Self> {
        let split_record = line.split_whitespace().collect::<Vec<_>>();

        if split_record.len() != 4 {
            return None;
        }

        let record = HistoryRecord {
            timestamp: split_record[0].to_string(),
            status: HistoryStatus::from_str(split_record[1]).ok()?,
            command: split_record[2].to_string(),
            target: split_record[3].to_string(),
        };

        Some(record)
    }
}

impl fmt::Display for HistoryRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = DateTime::parse_from_rfc3339(&self.timestamp).unwrap();

        write!(
            f,
            "| {:<20} | {:<10} | {:<10} | {}",
            dt.format("%Y-%m-%d %H:%M:%S"),
            self.status.to_string(),
            self.command,
            self.target
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

pub fn read_history() -> Result<Vec<HistoryRecord>, GitxError> {
    let file_path = history_file_path()?;

    let content =
        std::fs::read_to_string(file_path).map_err(|e| GitxError::HistoryFailed(e.to_string()))?;

    let records = content
        .lines()
        .filter_map(HistoryRecord::from_line)
        .collect::<Vec<_>>();

    Ok(records)
}
