//! Post
use crate::{Engine, Instance, Model};

/// Post Model
pub trait Post: Model {
    type Title: Instance;
    type Context: Instance;

    /// Title of the post
    fn title(&self) -> Result<Option<Self::Title>, <Self as Engine>::Error> {
        Ok(None)
    }

    /// Context of the post
    fn context(&self) -> Result<Option<Self::Context>, <Self as Engine>::Error> {
        Ok(None)
    }
}
