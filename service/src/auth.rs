//! Auth Service
use crate::Service;
use ct_model::Avatar;
use ct_primitive::{PublicKey, Storage};

/// Auth service
///
/// We don't provide fn like `register`, the plan ct recommand is key pair.
pub trait Auth: Service + Avatar
where
    <Self as Service>::Error: From<<Self as Storage>::Error>,
{
    /// Verify the origin
    fn auth(&self, sig: <Self as PublicKey>::Signature) -> Result<bool, <Self as Service>::Error> {
        Ok(self.session()? == Some(sig))
    }
}
