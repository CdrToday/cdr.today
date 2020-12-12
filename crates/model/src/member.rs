//! Member
use crate::{Avatar, Model};
use alloc::vec::Vec;

/// Post Model
pub trait Member<E, K>: Avatar<E, K>
where
    K: Default,
{
    /// All community of the member belongs to
    fn community<'n, N>(&self) -> Result<Vec<K>, <Self as Model<E, K>>::Error> {
        Ok(Vec::new())
    }
}
