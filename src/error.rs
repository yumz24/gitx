use std::fmt;

pub enum GitxError {
    InvalidArgs,
    GitCommandFailed(String),
}

impl fmt::Display for GitxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitxError::InvalidArgs => write!(f, "Usage: gitx <command> <type> <issue> <summary>"),
            GitxError::GitCommandFailed(err) => write!(f, "{}", err),
        }
    }
}
