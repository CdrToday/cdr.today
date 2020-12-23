//! Results

//! Errors
pub enum Error {
    /// The Query string is not valid
    InvalidQueryFormat,
    /// No results
    SegmentationFault,
}

/// Custom Result
pub type Result<T> = core::result::Result<T, Error>;
