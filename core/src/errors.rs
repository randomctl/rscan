use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CoreError {
    #[error("error parsing cli argument: {0}")]
    ArgParseError(String),
    #[error("unknown error in core")]
    Unknown,
}

