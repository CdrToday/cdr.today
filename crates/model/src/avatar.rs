//! Account
use crate::{Instance, Model};

/// Account Model
pub trait Avatar<E>: Model<E> {
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
}
