//! config
use serde::{Deserialize, Serialize};

mod conn;
mod http;

pub use conn::Connection;

/// cdr.today Config
#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    /// Http Config
    pub http: http::Http,
    /// Postgres config
    pub pg: conn::Connection,
    /// Redis config
    pub redis: conn::Connection,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            redis: conn::Connection::new("redis".to_string()),
            pg: conn::Connection::default(),
            http: http::Http::default(),
        }
    }
}
