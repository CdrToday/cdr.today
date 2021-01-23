//! Account Schema
use crate::Result;

table! {
    accounts (addr) {
        addr -> Binary,
    }
}

/// Account
pub struct Account {
    addr: Vec<u8>,
}

impl Account {
    /// account address
    pub fn addr(&self) -> Result<[u8; 32]> {
        if self.addr.len() != 32 {
            Err("Invalid address".into())
        } else {
            let mut slice: [u8; 32] = Default::default();
            slice.copy_from_slice(&self.addr);
            Ok(slice)
        }
    }
}
