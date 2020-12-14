//! Account
use crate::{Instance, Model};
use ct_primitive::PublicKey;

/// Account Model
pub trait Avatar<E>: Model<E> + PublicKey {
    type Name: Instance;

    /// Name of the account
    fn session(&self) -> Result<Option<<Self as PublicKey>::Signature>, <Self as Model<E>>::Error> {
        Ok(None)
    }

    /// Name of the account
    fn name<N>(&self) -> Result<Option<N>, <Self as Model<E>>::Error>
    where
        N: Instance,
    {
        Ok(None)
    }

    /// Avator of the account
    fn avator<A>(&self) -> Result<Option<A>, <Self as Model<E>>::Error>
    where
        A: Instance,
    {
        Ok(None)
    }
}
