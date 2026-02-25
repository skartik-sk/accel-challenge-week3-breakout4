use std::fmt;
use std::io;
use std::path::PathBuf;
use colored::*;

#[derive(Debug)]
pub enum ItError {
    NotARepository,

    BranchExists(String),

    InvalidRef(String),

    BranchNotFound(String),

    Io(io::Error),

    NothingToCommit,
}
impl fmt::Display for ItError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItError::NotARepository => {
                write!(
                    f,
                    "{}",
                   "fatal: not a it repository. Run 'it init' to create a new repository.".red().bold()
                )
            }
            ItError::BranchExists(name) => {
                write!(f, "{}", format!(" fatal: A branch named '{name}' already exists.").red().bold())
            }
            ItError::InvalidRef(ref_path) => {
                write!(f,"{}", format!("fatal: not a valid ref '{ref_path}'").red().bold())
            }
            ItError::BranchNotFound(name) => {
                write!(f, "{}",format!("fatal: branch '{name}' does not exist").red().bold())
            }
            ItError::Io(e) => write!(f,"{}", format!("fatal: {e}").red().bold()),
            ItError::NothingToCommit => {
                write!(f, "{}","NothingToCommit".red().bold())
            }
        }
    }
}
impl std::error::Error for ItError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ItError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ItError {
    fn from(err: io::Error) -> Self {
        ItError::Io(err)
    }
}
