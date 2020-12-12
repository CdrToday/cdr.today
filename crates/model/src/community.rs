//! Community
use crate::{Avatar, Model};
use alloc::vec::Vec;

/// Community Model
pub trait Communiy<E, K>: Avatar<E, K>
where
    K: Default,
{
    /// Owner of the community
    fn owner<'n, N>(&self) -> Result<K, <Self as Model<E, K>>::Error> {
        Ok(Default::default())
    }

    /// All channel of the community
    fn channel<'n, N>(&self) -> Result<Vec<K>, <Self as Model<E, K>>::Error> {
        Ok(Vec::new())
    }

    /// All members of the community
    fn member<'n, N>(&self) -> Result<Vec<K>, <Self as Model<E, K>>::Error> {
        Ok(Vec::new())
    }
}
