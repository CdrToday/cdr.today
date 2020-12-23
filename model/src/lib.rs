//! cdr today models
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod avatar;
mod community;
mod member;
mod post;

pub use self::{avatar::Avatar, community::Communiy, member::Member, post::Post};
use core::iter::Iterator;
use ct_primitive::Storage;

/// Storage Instance
pub trait Instance: AsRef<[u8]> + Sized {
    /// The key of this instance
    const KEY: &'static [u8];
    /// Which model this instance belongs to
    const MODEL: &'static [u8];
}

/// Field Vector Instance, which can push and remove elements
pub trait InstanceVector<T>: Instance + Iterator {
    /// Push element
    fn push(&mut self, element: &T);

    /// Remove element
    fn remove(&mut self, element: &T);
}

/// The Model
pub trait Model: AsRef<[u8]> + Sized + Storage {
    /// Storage Key
    type Key: Default + AsRef<[u8]> + Sized;

    /// The name space of the model
    const SPACE: &'static [u8];

    /// The key of the instance, maybe a hash
    fn key(&self) -> Result<Self::Key, <Self as Storage>::Error>;

    /// Flatten fields
    fn flatten<F>(&self) -> Result<Option<F>, <Self as Storage>::Error>
    where
        F: Instance,
    {
        Ok(None)
    }
}
