//! cdr.today practice
// pub mod ipc;
pub mod query;
pub mod result;

use query::Query;
use result::{Error, Result};

/// The template server
pub struct Server;

impl Server {
    /// Handle queries
    pub fn handle(&self, query: Query) -> Result<()> {
        if !query.is_valid() {
            Err(Error::InvalidQueryFormat)
        } else {
            Ok(())
        }
    }
}

/// The template client
pub struct Client;

impl Client {
    pub fn req(query: Query, server: &Server) -> Result<()> {
        server.handle(query)
    }
}
