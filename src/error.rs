use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ItError {
    NotARepository,

    BranchExists(String),

    InvalidRef(String),

    BranchNotFound(String),

    Io(io::Error),
}
impl fmt::Display for ItError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItError::NotARepository => {
                write!(
                    f,
                    "fatal: not a it repository. Run 'it init' to create a new repository."
                )
            }
            ItError::BranchExists(name) => {
                write!(f, "fatal: A branch named '{name}' already exists.")
            }
            ItError::InvalidRef(ref_path) => {
                write!(f, "fatal: not a valid ref '{ref_path}'")
            }
            ItError::BranchNotFound(name) => {
                write!(f, "fatal: branch '{name}' does not exist")
            }
            ItError::Io(e) => write!(f, "fatal: {e}"),
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
