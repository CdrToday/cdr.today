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
pub trait Instance: AsRef<[u8]> + Sized {
    /// The key of this instance
    const KEY: &'static [u8];
    /// Which model this instance belongs to
    const MODEL: &'static [u8];
}

/// Field Vector Instance, which can push and remove elements
pub trait InstanceVector<T>: Instance {
    /// Push element
    fn push(&mut self, element: &T);

    /// Remove element
    fn remove(&mut self, element: &T);
}

/// This trait abstrats the storage of ct models
pub trait Engine {
    type Error;

    /// Set storage
    fn set(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<(), Self::Error>;

    /// Get storage
    fn get<'e, T>(key: impl AsRef<[u8]>) -> Result<T, Self::Error>
    where
        T: From<&'e [u8]> + Model;

    /// Batch model instances
    fn batch<M>(&self, limit: Option<usize>) -> Result<Vec<M>, Self::Error>
    where
        M: Model;
}

/// The Model
pub trait Model: AsRef<[u8]> + Sized + Engine {
    /// Storage Key
    type Key: Default + AsRef<[u8]> + Sized;

    /// The name space of the model
    const SPACE: &'static [u8];

    /// The key of the instance, maybe a hash
    fn key(&self) -> Result<Self::Key, <Self as Engine>::Error>;

    /// Flatten fields
    fn flatten<F>(&self) -> Result<Option<F>, <Self as Engine>::Error>
    where
        F: Instance,
    {
        Ok(None)
    }
}
