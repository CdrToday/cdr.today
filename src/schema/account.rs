//! Account Schema
use super::{Schema, TableContext};
use crate::{db::PgConn as Conn, Result};
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl};
use juniper::GraphQLObject;

table! {
    accounts (addr) {
        addr -> Text,
    }
}

// Account OP
use accounts::dsl::{accounts as accounts_table, addr as addr_col};

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

impl Account {
    /// First
    pub fn first(conn: &Conn, addr: String) -> Result<Account> {
        Ok(accounts_table
            .filter(addr_col.eq(&addr))
            .first::<Self>(conn)?)
    }
}
