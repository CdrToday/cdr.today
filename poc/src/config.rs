//! config
use serde::{Deserialize, Serialize};

/// cdr.today Config
#[derive(Serialize, Deserialize, Default)]
pub struct Config<'c> {
    /// Postgres config
    #[serde(borrow)]
    pub pg: Pg<'c>,
}

/// Postgres config
#[derive(Serialize, Deserialize)]
pub struct Pg<'c> {
    /// databse name
    pub name: &'c str,
    /// postgres addr
    pub addr: &'c str,
    /// postgres port
    pub port: Option<u16>,
    /// If has username
    pub user: Option<&'c str>,
    /// If has password
    pub secret: Option<&'c str>,
    /// If override the connection url of postgresql
    pub r#override: Option<&'c str>,
    /// Connection numbers
    pub conn: u8,
}

impl<'c> Default for Pg<'c> {
    fn default() -> Pg<'c> {
        Pg {
            name: "cdr_today",
            addr: "localhost",
            port: None,
            user: None,
            secret: None,
            r#override: None,
            conn: 3,
        }
    }
}

impl<'c> Pg<'c> {
    /// PostgresQL url
    ///
    /// https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
    pub fn url(&self) -> String {
        let mut url = String::from("postgresql://");

        // Username and password
        match (self.user, self.secret) {
            (Some(user), Some(secret)) => {
                url.push_str(&format!("{}@{}", user, secret));
            }
            (Some(user), None) => {
                url.push_str(&format!("{}@", user));
            }
            _ => {}
        }

        // Push addr
        url.push_str(self.addr);

        // Push port
        if let Some(port) = self.port {
            url.push_str(&format!(":{}", port));
        }

        // If with name
        url.push_str(&format!("/{}", self.name));

        url
    }
}
