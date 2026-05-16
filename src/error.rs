use std::fmt;

pub enum GitxError {
    InvalidCommand,
    InvalidBranchArgs,
    InvalidDeleteArgs,
    GitCommandFailed(String),
}

impl fmt::Display for GitxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitxError::InvalidCommand => {
                write!(f, "Usage: gitx <command> [args]")
            }
            GitxError::InvalidBranchArgs => {
                write!(f, "Usage: gitx <command> <type> <issue> <summary>")
            }
            GitxError::InvalidDeleteArgs => {
                write!(f, "Usage: gitx <branch_name>")
            }
            GitxError::GitCommandFailed(err) => write!(f, "{}", err),
        }
    }
}
