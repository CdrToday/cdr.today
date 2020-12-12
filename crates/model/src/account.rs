//! Account
use crate::{Instance, Model};

/// Account Model
pub trait Account<E>: Model<E> {
    /// Name of the account
    fn name<'n, N>(&self) -> Result<Option<N>, <Self as Model<E>>::Error>
    where
        N: Instance<'n>,
    {
        Ok(None)
    }

    /// Avator of the account
    fn avator<'a, A>(&self) -> Result<Option<A>, <Self as Model<E>>::Error>
    where
        A: Instance<'a>,
    {
        Ok(None)
    }

    /// Location of the account
    fn location<'l, L>(&self) -> Result<Option<L>, <Self as Model<E>>::Error>
    where
        L: Instance<'l>,
    {
        Ok(None)
    }

    /// Flatten profile
    fn flatten<'f, F>(&self) -> Result<Option<F>, <Self as Model<E>>::Error>
    where
        F: Instance<'f>,
    {
        Ok(None)
    }
}
