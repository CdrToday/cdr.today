//! Post
use crate::{Instance, Model};

/// Post Model
pub trait Post<E>: Model<E> {
    type Title: Instance;
    type Context: Instance;

    /// Title of the post
    fn title(&self) -> Result<Option<Self::Title>, <Self as Model<E>>::Error> {
        Ok(None)
    }

    /// Context of the post
    fn context(&self) -> Result<Option<Self::Context>, <Self as Model<E>>::Error> {
        Ok(None)
    }
}
