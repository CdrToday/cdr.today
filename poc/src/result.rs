//! result
use thiserror::Error as E;

/// Result for this App
pub type Result<T> = core::result::Result<T, Error>;

/// Error for this App
#[derive(E, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
}
