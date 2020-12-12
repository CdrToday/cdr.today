//! Post
use crate::{Instance, Model};

/// Post Model
pub trait Post<E, K>: Model<E, K>
where
    K: Default,
{
    /// Title of the post
    fn title<'n, N>(&self) -> Result<Option<N>, <Self as Model<E, K>>::Error>
    where
        N: Instance<'n>,
    {
        Ok(None)
    }

    /// Context of the post
    fn context<'n, N>(&self) -> Result<Option<N>, <Self as Model<E, K>>::Error>
    where
        N: Instance<'n>,
    {
        Ok(None)
    }
}
