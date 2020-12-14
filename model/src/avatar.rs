//! Account
use crate::{Instance, Model};
use ct_primitive::PublicKey;

/// Account Model
pub trait Avatar<E>: Model<E> + PublicKey {
    type Avatar: Instance;
    type Name: Instance;

    /// Name of the account
    fn session(&self) -> Result<Option<<Self as PublicKey>::Signature>, <Self as Model<E>>::Error> {
        Ok(None)
    }

    /// Name of the account
    fn name(&self) -> Result<Option<Self::Name>, <Self as Model<E>>::Error> {
        Ok(None)
    }

    /// Avator of the account
    fn avator(&self) -> Result<Option<Self::Avatar>, <Self as Model<E>>::Error> {
        Ok(None)
    }
}
