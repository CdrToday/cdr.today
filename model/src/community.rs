//! Community
use crate::{Avatar, Model};
use alloc::vec::Vec;

/// Community Model
pub trait Communiy<E>: Avatar<E> {
    /// Owner of the community
    fn owner(&self) -> Result<<Self as Model<E>>::Key, <Self as Model<E>>::Error> {
        Ok(Default::default())
    }

    /// All channel of the community
    fn channel(&self) -> Result<Vec<<Self as Model<E>>::Key>, <Self as Model<E>>::Error> {
        Ok(Vec::new())
    }

    /// All members of the community
    fn member(&self) -> Result<Vec<<Self as Model<E>>::Key>, <Self as Model<E>>::Error> {
        Ok(Vec::new())
    }
}
