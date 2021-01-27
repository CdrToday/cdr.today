//! orm, postgresql here
use crate::{Config, Result};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    RunQueryDsl,
};
use log::{info, warn};
use std::process::{Command, Stdio};

/// Pooled connection
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

/// CREATE TABLE Tempalte
static CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS ${TABLE_NAME} (${TABLE_CTX})";

/// Default database name
static DEFAULT_NAME: &str = "cdr_today";

/// Orm operation set
pub struct Orm(Pool<ConnectionManager<PgConnection>>);

impl Orm {
    // only support OSX for now
    fn create_db_if_not_exists(config: &Config) -> Result<()> {
        let default_name = DEFAULT_NAME.to_string();
        let db_name = config.pg.db.as_ref().unwrap_or(&default_name);

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
    pub fn conn(&self) -> Result<PooledConn> {
        Ok(self.0.get()?)
    }
}
