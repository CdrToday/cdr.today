//! cdr.today address
use crate::{Error, Result};
use core::{
    fmt::{self, Display, Formatter},
    result,
};
use ed25519_dalek::{PublicKey, Signature};

/// cdr.today address
pub struct Address(PublicKey);

impl Display for Address {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(fmt, "{:#x?}", self.0.to_bytes())?;
        Ok(())
    }
}

impl Address {
    /// From base58 string
    pub fn from_str(b58: &str) -> Result<Self> {
        let bytes = bs58::decode(b58).into_vec()?;

        if bytes.len() != 32 {
            Err(Error::InvalidAddressLength)
        } else {
            Ok(Address(PublicKey::from_bytes(&bytes)?))
        }
    }

    /// Verify signature and the given data
    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool> {
        if signature.len() != 64 {
            Err(Error::InvalidSignatureLength)
        } else {
            let mut sig = [0; 64];
            sig.copy_from_slice(&signature);
            Ok(self.0.verify_strict(msg, &Signature::new(sig)).is_ok())
        }
    }
}
