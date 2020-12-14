//! Post
use crate::{Instance, Model};

/// Post Model
pub trait Post<E>: Model<E> {
    /// Title of the post
    fn title<N>(&self) -> Result<Option<N>, <Self as Model<E>>::Error>
    where
        N: Instance,
    {
        Ok(None)
    }

    /// Context of the post
    fn context<N>(&self) -> Result<Option<N>, <Self as Model<E>>::Error>
    where
        N: Instance,
    {
        Ok(None)
    }
}
