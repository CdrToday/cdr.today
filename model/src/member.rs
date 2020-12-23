//! Member
use crate::{Avatar, Model, Storage};
use alloc::vec::Vec;

/// Post Model
pub trait Member: Avatar {
    /// All community of the member belongs to
    fn community(&self) -> Result<Vec<<Self as Model>::Key>, <Self as Storage>::Error> {
        Ok(Vec::new())
    }
}
