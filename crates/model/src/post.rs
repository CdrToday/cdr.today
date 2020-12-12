//! Post
use crate::Model;

/// Post Model
pub trait Post: Model {
    /// Title of the post
    fn title<'n, N>(&self) -> Result<Option<N>, <Self as Model<E>>::Error>
    where
        N: Instance<'n>,
    {
        Ok(None)
    }
}
