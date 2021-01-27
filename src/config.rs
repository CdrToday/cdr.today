//! config
use serde::{Deserialize, Serialize};

/// cdr.today Config
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Config {
    pub http: Http,
    /// Postgres config
    pub pg: Connection,
}

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

/// Connection name trait
pub trait ConnectionName {
    fn name() -> &'static str;
    fn database_name() -> Option<String>;
}

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
        Connection {
            name: "postgresql".to_string(),
            db: None,
            hostname: None,
            port: None,
            username: None,
            passwd: None,
            r#override: None,
            conn: 3,
        }
    }
}

impl Connection {
    pub fn new(protocol: String) -> Self {
        let mut conn = Self::default();
        conn.name = protocol;
        conn
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
