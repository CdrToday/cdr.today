//! cdr-today storage
/// This trait abstrats the storage of ct models
pub trait Storage {
    /// Custom Error
    type Error;

    /// Set storage
    fn set(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<(), Self::Error>;

    /// Get storage
    fn get<'e, T>(key: impl AsRef<[u8]>) -> Result<T, Self::Error>
    where
        T: From<&'e [u8]>;
}
