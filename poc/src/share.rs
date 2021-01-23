//! Shared data
use crate::{
    orm::{ConnPool, Orm},
    schema::{Account, Schema},
    Config, Result,
};

/// Shared data
pub struct Shared {
    pub pool: ConnPool,
    pub config: Config,
}

impl Shared {
    fn gather_tables() -> Vec<(&'static str, Vec<&'static str>)> {
        vec![<Account as Schema>::table()]
    }

    /// New shared data
    pub fn new(config: Config) -> Result<Shared> {
        Ok(Shared {
            pool: Orm::new(&config)?
                .create_tables(Self::gather_tables())?
                .pool(),
            config: config,
        })
    }
}
