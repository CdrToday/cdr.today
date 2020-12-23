//! Post
use crate::{Instance, Model, Storage};

/// Post Model
pub trait Post: Model {
    type Title: Instance;
    type Context: Instance;

    /// Title of the post
    fn title(&self) -> Result<Option<Self::Title>, <Self as Storage>::Error> {
        Ok(None)
    }

    /// Context of the post
    fn context(&self) -> Result<Option<Self::Context>, <Self as Storage>::Error> {
        Ok(None)
    }
}
