//! cdr-today primitives
#![deny(missing_docs)]
#![no_std]
mod hash;
mod key;

pub use self::{hash::Hash, key::PublicKey};
