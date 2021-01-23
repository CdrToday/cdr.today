//! result
use thiserror::Error as E;

/// Error for this App
#[derive(E, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    #[error(transparent)]
    R2d2(#[from] diesel::r2d2::PoolError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Custom(&'static str),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Error {
        Error::Custom(s)
    }
}

/// Result for this App
pub type Result<T> = core::result::Result<T, Error>;
