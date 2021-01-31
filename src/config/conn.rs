//! config
use serde::{Deserialize, Serialize};

/// Database protocol config
#[derive(Clone, Deserialize, Serialize)]
pub struct Connection {
    /// protocol type
    #[serde(skip)]
    pub name: String,
    /// databse name
    pub db: Option<String>,
    /// conn hostname
    pub hostname: Option<String>,
    /// conn port
    pub port: Option<u16>,
    /// If has usernamename
    pub username: Option<String>,
    /// If has password
    pub passwd: Option<String>,
    /// If override the connection url of postgresql
    pub r#override: Option<String>,
    /// Connection numbers
    pub conn: u8,
}

impl Default for Connection {
    fn default() -> Connection {
        Connection::new("postgresql".to_string())
    }
}

impl Connection {
    pub fn new(protocol: String) -> Self {
        Connection {
            name: protocol,
            db: None,
            hostname: None,
            port: None,
            username: None,
            passwd: None,
            r#override: None,
            conn: 3,
        }
    }

    /// Encode url
    pub fn url(&self) -> String {
        let mut url = format!("{}://", self.name);

        // Usernamename and password
        match (&self.username, &self.passwd) {
            (Some(username), Some(passwd)) => {
                url.push_str(&format!("{}@{}", username, passwd));
            }
            (Some(username), None) => {
                url.push_str(&format!("{}@", username));
            }
            _ => {}
        }

        // Push hostname
        if let Some(hostname) = &self.hostname {
            url.push_str(hostname);
        } else {
            url.push_str("0.0.0.0");
        }

        // Push port
        if let Some(port) = self.port {
            url.push_str(&format!(":{}", port));
        }

        // If with name
        if let Some(db) = &self.db {
            url.push_str(&format!("/{}", db));
        }

        url
    }
}
