//! Community
use crate::{Avatar, Engine, Model};
use alloc::vec::Vec;

/// Community Model
pub trait Communiy: Avatar {
    /// Owner of the community
    fn owner(&self) -> Result<<Self as Model>::Key, <Self as Engine>::Error> {
        Ok(Default::default())
    }

    /// All channel of the community
    fn channel(&self) -> Result<Vec<<Self as Model>::Key>, <Self as Engine>::Error> {
        Ok(Vec::new())
    }

    /// All members of the community
    fn member(&self) -> Result<Vec<<Self as Model>::Key>, <Self as Engine>::Error> {
        Ok(Vec::new())
    }
}
