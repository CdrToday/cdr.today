//! config
use serde::{Deserialize, Serialize};

/// cdr.today Config
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pg: Pg,
}

/// Postgres config
#[derive(Serialize, Deserialize)]
pub struct Pg {
    pub addr: String,
    pub port: u16,
}
