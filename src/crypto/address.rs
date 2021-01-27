//! cdr.today address
use crate::{Error, Result};
use core::{
    fmt::{self, Display, Formatter},
    result,
};
use openssl::{
    pkey::{PKey, Public},
    sign::Verifier,
};

/// cdr.today address
pub struct Address(PKey<Public>);

impl Display for Address {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(
            fmt,
            "{:#x?}",
            self.0.public_key_to_der().map_err(|_| fmt::Error)
        )?;
        Ok(())
    }
}

impl Address {
    fn verifier(&self) -> Result<Verifier> {
        Ok(Verifier::new_without_digest(&self.0)?)
    }

    /// From base58 string
    pub fn from_str(b58: &str) -> Result<Self> {
        let bytes = bs58::decode(b58).into_vec()?;

        if bytes.len() != 32 {
            Err(Error::InvalidAddressLength)
        } else {
            // let mut dst = [0; 32];
            // dst.copy_from_slice(&bytes);
            Ok(Address(PKey::public_key_from_der(&bytes)?))
        }
    }

    /// Verify signature
    pub fn verify(&self, signature: &[u8]) -> Result<bool> {
        Ok(self.verifier()?.verify(signature)?)
    }

    /// Verify signature and the given data
    pub fn verify_oneshot(&self, signature: &[u8], buf: &[u8]) -> Result<bool> {
        Ok(self.verifier()?.verify_oneshot(signature, buf)?)
    }
}
