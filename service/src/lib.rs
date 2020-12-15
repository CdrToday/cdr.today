//! cdr-today-service
#![no_std]
#![deny(missing_docs)]
mod auth;

/// This service trait abstracts the common service ct provies
pub trait Service {
    /// Service Name
    const NAME: dyn AsRef<[u8]>;

    /// Service Error
    type Error;
}
