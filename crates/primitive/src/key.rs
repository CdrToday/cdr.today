//! Public key in asymmetric cryptography
//!
//! As server of Apps, here we only handle the public key things.

/// Public key in asymmetric cryptography
pub trait PublicKey {
    /// The raw bytes of the public key
    type Raw: AsRef<[u8]> + Sized;

    /// Verify a signature
    fn verify(signature: &[u8]) -> bool;
}
