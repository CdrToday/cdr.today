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
pub trait Instance<'i>: From<&'i [u8]> + Sized {
    /// Which model this instance belongs to
    const MODEL: &'i [u8];
}

/// Field Vector Instance, which can push and remove elements
pub trait InstanceVector<'i, T>: Instance<'i>
where
    T: Instance<'i>,
{
    /// Which model this vector belongs to
    const MODEL: &'i [u8];

    /// Push element
    fn push(&mut self, element: &T);

    /// Remove element
    fn remove(&mut self, element: &T);

    /// To vector
    fn to_vec(&self) -> Vec<T>;

    /// From vector
    fn from_vec(v: Vec<T>) -> Self;
}

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
    fn batch<M>(&self, limit: Option<usize>) -> Result<Vec<M>, Self::Error>
    where
        M: Model<Self::Error>;
}

/// The Model
pub trait Model<E>: AsRef<[u8]> + Sized {
    /// Model Error
    type Error: Into<E>;

    /// Storage Key
    type Key: Default + AsRef<[u8]>;

    /// The name space of the model
    const SPACE: Vec<u8>;

    /// The key of the instance, maybe a hash
    fn key<'k>(&self) -> Result<Self::Key, Self::Error>;

    /// Flatten fields
    fn flatten<'f, F>(&self) -> Result<Option<F>, Self::Error>
    where
        F: Instance<'f>,
    {
        Ok(None)
    }
}
