//! cdr today models
#![no_std]
extern crate alloc;
use alloc::vec::Vec;

mod account;

/// Storage Instance
pub trait Instance<'i>: From<&'i [u8]> + Sized {}

/// This trait abstrats the storage of ct models
pub trait Engine {
    type Error;

    /// Set storage
    fn set(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<(), Self::Error>;

    /// Get storage
    fn get<'e, T>(key: impl AsRef<[u8]>) -> Result<T, Self::Error>
    where
        T: From<&'e [u8]>;

    /// Batch model instances
    fn batch<M>(&self, limit: usize) -> Result<Vec<M>, Self::Error>
    where
        M: Model<Self::Error>;
}

/// The Model
pub trait Model<E>: AsRef<[u8]> + Sized {
    /// Error
    type Error: Into<E>;

    /// The key of the account, maybe a hash
    fn key<'k, K>(&self) -> Result<K, Self::Error>
    where
        K: Instance<'k>;
}
