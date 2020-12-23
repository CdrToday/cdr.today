//! cdr-today-service
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
mod auth;

/// This service trait abstracts the common service ct provies
pub trait Service {
    /// Service Name
    const NAME: dyn AsRef<[u8]>;

    /// Service Error
    type Error;
}
