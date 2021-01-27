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
    R2d2(#[from] r2d2::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ActixWeb(#[from] actix_web::Error),
    #[error(transparent)]
    Reid(#[from] redis::RedisError),
    #[error(transparent)]
    Base58Decode(#[from] bs58::decode::Error),
    #[error(transparent)]
    OpenSSLStack(#[from] openssl::error::ErrorStack),
    #[error("{0}")]
    Custom(&'static str),
    /// Invalid Address Length
    #[error("invalid address length")]
    InvalidAddressLength,
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Error {
        Error::Custom(s)
    }
}

/// Result for this App
pub type Result<T> = core::result::Result<T, Error>;
