//! cdr today models
#![no_std]
extern crate alloc;
use alloc::vec::Vec;

mod avatar;
mod community;
mod member;
mod post;

pub use self::{avatar::Avatar, community::Communiy, member::Member, post::Post};

/// Storage Instance
pub trait Instance<'i>: From<&'i [u8]> + Sized {}

/// This trait abstrats the storage of ct models
pub trait Engine
where
    Self::Key: Default,
{
    type Error;
    type Key;

    /// Set storage
    fn set(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<(), Self::Error>;

    /// Get storage
    fn get<'e, T>(key: impl AsRef<[u8]>) -> Result<T, Self::Error>
    where
        T: From<&'e [u8]>;

    /// Batch model instances
    fn batch<M>(&self, limit: usize) -> Result<Vec<M>, Self::Error>
    where
        M: Model<Self::Error, Self::Key>;
}

/// The Model
pub trait Model<E, K>: AsRef<[u8]> + Sized
where
    K: Default,
{
    /// Model Error
    type Error: Into<E>;

    /// The name space of the model
    const SPACE: Vec<u8>;

    /// The key of the account, maybe a hash
    fn key<'k>(&self) -> Result<K, Self::Error>
    where
        K: Instance<'k>;

    /// Flatten fields
    fn flatten<'f, F>(&self) -> Result<Option<F>, Self::Error>
    where
        F: Instance<'f>,
    {
        Ok(None)
    }
}
