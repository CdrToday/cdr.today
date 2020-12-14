//! Post
use crate::{Instance, Model};

/// Post Model
pub trait Post<E>: Model<E> {
    /// Title of the post
    fn title<'n, N>(&self) -> Result<Option<N>, <Self as Model<E>>::Error>
    where
        N: Instance<'n>,
    {
        Ok(None)
    }

    /// Context of the post
    fn context<'n, N>(&self) -> Result<Option<N>, <Self as Model<E>>::Error>
    where
        N: Instance<'n>,
    {
        Ok(None)
    }
}
