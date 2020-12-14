//! Member
use crate::{Avatar, Model};
use alloc::vec::Vec;

/// Post Model
pub trait Member<E>: Avatar<E> {
    /// All community of the member belongs to
    fn community(&self) -> Result<Vec<<Self as Model<E>>::Key>, <Self as Model<E>>::Error> {
        Ok(Vec::new())
    }
}
