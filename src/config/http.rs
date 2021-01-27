//! Http config
use serde::{Deserialize, Serialize};

/// Http config
#[derive(Clone, Deserialize, Serialize)]
pub struct Http {
    pub hostname: String,
    pub port: u16,
}

impl Default for Http {
    fn default() -> Http {
        Http {
            hostname: "0.0.0.0".into(),
            port: 3000,
        }
    }
}

impl Http {
    /// Convertw hostname and port to url
    pub fn url(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}
