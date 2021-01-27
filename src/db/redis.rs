//! Redis
use crate::{Config, Result};
use r2d2::{Pool, PooledConnection};
use redis::Client;

/// Redis Pooled Connection
pub type Conn = PooledConnection<Client>;

/// r2d2 Redis
pub struct Redis(Pool<Client>);

impl Redis {
    /// New redis
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Self(
            Pool::builder().build(Client::open(&*config.redis.url())?)?,
        ))
    }

    /// Give out the pool
    pub fn conn(&self) -> Result<Conn> {
        Ok(self.0.get()?)
    }
}
