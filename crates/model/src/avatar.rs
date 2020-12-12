//! Account
use crate::{Instance, Model};

/// Account Model
pub trait Avatar<E, K>: Model<E, K>
where
    K: Default,
{
    /// Name of the account
    fn name<'n, N>(&self) -> Result<Option<N>, <Self as Model<E, K>>::Error>
    where
        N: Instance<'n>,
    {
        Ok(None)
    }

    /// Avator of the account
    fn avator<'a, A>(&self) -> Result<Option<A>, <Self as Model<E, K>>::Error>
    where
        A: Instance<'a>,
    {
        Ok(None)
    }
}
