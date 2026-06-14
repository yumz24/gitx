use std::fmt;

pub enum GitxError {
    InvalidCommand,
    InvalidBranchArgs,
    InvalidDeleteArgs,
    MissingLimitValue,
    InvalidLimitValue,
    MissingHistoryFilterValue,
    InvalidHistoryFilter,
    UnknownHistoryOption(String),
    UnknownHistoryArguments(String),
    DuplicateHistoryOption(String),
    HistoryFailed(String),
    CannotDeleteCurrentBranch,
    CannotDeleteProtectedBranch,
    GitCommandFailed { command: String, stderr: String },
}

impl fmt::Display for GitxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitxError::InvalidCommand => {
                write!(f, "Usage: gitx <command> [args]")
            }
            GitxError::InvalidBranchArgs => {
                write!(f, "Usage: gitx branch <type> <issue> <summary>")
            }
            GitxError::InvalidDeleteArgs => {
                write!(f, "Usage: gitx delete <branch_name>")
            }
            GitxError::MissingLimitValue => {
                write!(f, "Missing value for --limit, -l")
            }
            GitxError::InvalidLimitValue => {
                write!(f, "--limit expexts a number")
            }
            GitxError::MissingHistoryFilterValue => {
                write!(f, "Missing value for --filter, -f")
            }
            GitxError::InvalidHistoryFilter => {
                write!(f, "Invalid history filter")
            }
            GitxError::UnknownHistoryOption(option) => {
                write!(f, "Unknown history option: {}", option)
            }
            GitxError::UnknownHistoryArguments(args) => {
                write!(f, "Unknown history argument: {}", args)
            }
            GitxError::DuplicateHistoryOption(option) => {
                write!(f, "option '{}' cannot be used multiple times", option)
            }
            GitxError::CannotDeleteCurrentBranch => {
                write!(f, "Cannot delete current branch")
            }
            GitxError::CannotDeleteProtectedBranch => {
                write!(f, "Cannot delete protected branch")
            }
            GitxError::GitCommandFailed { command, stderr } => {
                write!(f, "git command failed: {}\n{}", command, stderr,)
            }
            GitxError::HistoryFailed(err) => {
                write!(f, "{}", err)
            }
        }
    }
}
