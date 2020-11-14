//! Cdr.today result

mod err;

pub use err::Error;

/// Cdr.Today Result
pub type Result<T> = std::result::Result<T, Error>;
