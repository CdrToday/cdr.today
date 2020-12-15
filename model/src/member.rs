//! Member
use crate::{Avatar, Engine, Model};
use alloc::vec::Vec;

/// Post Model
pub trait Member: Avatar {
    /// All community of the member belongs to
    fn community(&self) -> Result<Vec<<Self as Model>::Key>, <Self as Engine>::Error> {
        Ok(Vec::new())
    }
}
