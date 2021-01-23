//! orm, postgresql here
use crate::{Config, Result};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::{info, warn};
use std::process::{Command, Stdio};

/// Orm operation set
pub struct Orm {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Orm {
    // only support OSX for now
    fn create_db_if_not_exists(config: &Config) -> Result<()> {
        if !String::from_utf8_lossy(&Command::new("psql").arg("-l").output()?.stdout)
            .contains(&format!("\n {}", &config.pg.name))
        {
            warn!("Database {} doesn't exists, creating...", &config.pg.name);
            Command::new("createdb")
                .arg(config.pg.name)
                .stdout(Stdio::null())
                .status()?;

            info!("Created databse {}", &config.pg.name);
        }

        Ok(())
    }

    /// New orm set with connection
    pub fn new(config: &Config) -> Result<Self> {
        Self::create_db_if_not_exists(config)?;
        Ok(Self {
            pool: Pool::builder().build(ConnectionManager::<PgConnection>::new(config.pg.url()))?,
        })
    }
}
