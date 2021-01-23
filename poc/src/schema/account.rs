//! Account Schema
use super::{Schema, TableContext};
use diesel::Queryable;
use juniper::GraphQLObject;

table! {
    accounts (addr) {
        addr -> Text,
    }
}

/// Account
#[derive(GraphQLObject, Queryable)]
pub struct Account {
    /// Account address
    pub addr: String,
}

impl Schema for Account {
    fn table() -> TableContext {
        ("accounts", vec!["addr Text NOT NULL"])
    }
}
