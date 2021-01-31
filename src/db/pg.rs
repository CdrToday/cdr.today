//! orm, postgresql here
use crate::{Config, Result};
use diesel::{pg::PgConnection, r2d2::ConnectionManager, RunQueryDsl};
use juniper::Context;
use log::{info, warn};
use r2d2::{Pool, PooledConnection};
use std::process::{Command, Stdio};

/// CREATE TABLE Tempalte
static CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS ${TABLE_NAME} (${TABLE_CTX})";

/// Pooled PostgreSQL Connection
pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

/// Pg operation set
pub struct Pg(Pool<ConnectionManager<PgConnection>>);

impl Pg {
    // only support OSX for now
    fn create_db_if_not_exists(config: &Config) -> Result<()> {
        let username = whoami::username();
        let db_name = config.pg.db.as_ref().unwrap_or(&username);

        if !String::from_utf8_lossy(&Command::new("psql").arg("-l").output()?.stdout)
            .contains(&format!("\n {}", &db_name))
        {
            warn!("Database {} doesn't exists, creating...", &db_name);
            Command::new("createdb")
                .arg(&db_name)
                .stdout(Stdio::null())
                .status()?;

            info!("Created databse {}", &db_name);
        }

        Ok(())
    }

    /// New orm set with connection
    pub fn new(config: &Config) -> Result<Self> {
        Self::create_db_if_not_exists(config)?;
        Ok(Self(Pool::builder().build(ConnectionManager::<
            PgConnection,
        >::new(
            config.pg.url()
        ))?))
    }

    /// Create tables
    pub fn create_tables(self, tables: Vec<(&'static str, Vec<&'static str>)>) -> Result<Self> {
        for t in tables {
            diesel::sql_query(CREATE_TABLE.replace("${TABLE_NAME}", t.0).replace(
                "${TABLE_CTX}",
                &format!("\n{}\n\n", t.1.join(",\n").trim_end_matches(",\n")),
            ))
            .execute(&self.0.get()?)?;
        }
        Ok(self)
    }

    /// Give out the pool
    pub fn conn(&self) -> Result<Conn> {
        Ok(self.0.get()?)
    }
}

impl Context for Pg {}
