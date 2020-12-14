//! Hash

/// Hash
pub trait Hash {
    /// Hash output
    type Output: Sized + AsRef<[u8]>;

    /// Hash bytes
    fn hash(input: &[u8]) -> Self::Output;
}
