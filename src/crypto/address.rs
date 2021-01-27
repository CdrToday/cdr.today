//! cdr.today address
use crate::{Error, Result};
use core::{
    fmt::{self, Display, Formatter},
    result,
};
use openssl::{pkey::PKey, sign::Verifier};

/// cdr.today address
pub struct Address([u8; 32]);

impl Display for Address {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(fmt, "{:#x?}", self.0)?;
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
            let mut dst = [0; 32];
            dst.copy_from_slice(&bytes);
            Ok(Address(dst))
        }
    }

    /// Verify signature
    pub fn verify(&self, signature: &[u8]) -> Result<bool> {
        let pk = PKey::public_key_from_der(&self.0)?;
        let verifier = Verifier::new_without_digest(&pk)?;

        Ok(verifier.verify(signature)?)
    }
}
