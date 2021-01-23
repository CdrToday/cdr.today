//! Schema
mod account;

pub use self::account::{accounts, Account};

/// Table info for generating SQL
pub type TableContext = (&'static str, Vec<&'static str>);

/// Schema which is queryable and could be made into graphql object
pub trait Schema {
    /// For example:
    ///
    /// ```rust
    /// (
    ///   "account", [
    ///      "name TEXT NOT NULL",
    ///      "address TEXT NOT NULL",
    ///   ]
    /// )
    /// ```
    fn table() -> TableContext;
}
