//! config
use serde::{Deserialize, Serialize};

/// cdr.today Config
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Config {
    /// Postgres config
    pub pg: Pg,
}

/// Postgres config
#[derive(Clone, Deserialize, Serialize)]
pub struct Pg {
    /// databse name
    pub name: String,
    /// postgres addr
    pub addr: String,
    /// postgres port
    pub port: Option<u16>,
    /// If has username
    pub user: Option<String>,
    /// If has password
    pub secret: Option<String>,
    /// If override the connection url of postgresql
    pub r#override: Option<String>,
    /// Connection numbers
    pub conn: u8,
}

impl<'c> Default for Pg {
    fn default() -> Pg {
        Pg {
            name: "cdr_today".into(),
            addr: "localhost".into(),
            port: None,
            user: None,
            secret: None,
            r#override: None,
            conn: 3,
        }
    }
}

impl Pg {
    /// PostgresQL url
    ///
    /// https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
    pub fn url(&self) -> String {
        let mut url = String::from("postgresql://");

        // Username and password
        match (&self.user, &self.secret) {
            (Some(user), Some(secret)) => {
                url.push_str(&format!("{}@{}", user, secret));
            }
            (Some(user), None) => {
                url.push_str(&format!("{}@", user));
            }
            _ => {}
        }

        // Push addr
        url.push_str(&self.addr);

        // Push port
        if let Some(port) = self.port {
            url.push_str(&format!(":{}", port));
        }

        // If with name
        url.push_str(&format!("/{}", self.name));

        url
    }
}
