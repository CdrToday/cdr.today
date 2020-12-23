//! Account
use crate::{Instance, Model, Storage};
use ct_primitive::PublicKey;

/// Account Model
pub trait Avatar: Model + PublicKey {
    type Avatar: Instance;
    type Name: Instance;

    /// Name of the account
    fn session(&self) -> Result<Option<<Self as PublicKey>::Signature>, <Self as Storage>::Error> {
        Ok(None)
    }

    /// Name of the account
    fn name(&self) -> Result<Option<Self::Name>, <Self as Storage>::Error> {
        Ok(None)
    }

    /// Avator of the account
    fn avator(&self) -> Result<Option<Self::Avatar>, <Self as Storage>::Error> {
        Ok(None)
    }
}
