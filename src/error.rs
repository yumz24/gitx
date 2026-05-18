use std::fmt;

pub enum GitxError {
    InvalidCommand,
    InvalidBranchArgs,
    InvalidDeleteArgs,
    CannotDeleteCurrentBranch,
    CannotDeleteProtectedBranch,
    GitCommandFailed(String),
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
            GitxError::CannotDeleteCurrentBranch => {
                write!(f, "Cannot delete current branch")
            }
            GitxError::CannotDeleteProtectedBranch => {
                write!(f, "Cannot delete protected branch")
            }
            GitxError::GitCommandFailed(err) => write!(f, "{}", err),
        }
    }
}
