//! Public key in asymmetric cryptography
//!
//! As server of Apps, here we only handle the public key things.

/// Public key in asymmetric cryptography
pub trait PublicKey {
    /// The raw bytes of the public key
    type Raw: AsRef<[u8]> + Sized;

    /// The raw bytes of the public key
    type Signature: AsRef<[u8]> + Sized;

    /// Verify a signature
    fn verify(&self, signature: dyn AsRef<Self::Signature>) -> bool;
}
