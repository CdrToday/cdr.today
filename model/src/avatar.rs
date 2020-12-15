//! Account
use crate::{Engine, Instance, Model};
use ct_primitive::PublicKey;

/// Account Model
pub trait Avatar: Model + PublicKey {
    type Avatar: Instance;
    type Name: Instance;

    /// Name of the account
    fn session(&self) -> Result<Option<<Self as PublicKey>::Signature>, <Self as Engine>::Error> {
        Ok(None)
    }

    /// Name of the account
    fn name(&self) -> Result<Option<Self::Name>, <Self as Engine>::Error> {
        Ok(None)
    }

    /// Avator of the account
    fn avator(&self) -> Result<Option<Self::Avatar>, <Self as Engine>::Error> {
        Ok(None)
    }
}
