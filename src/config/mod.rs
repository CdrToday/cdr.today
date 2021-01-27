//! config
use serde::{Deserialize, Serialize};

mod conn;
mod http;

/// cdr.today Config
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Config {
    /// Http Config
    pub http: http::Http,
    /// Postgres config
    pub pg: conn::Connection,
    /// Redis config
    pub redis: conn::Connection,
}
