//! Redis
use crate::{Config, Result};
use r2d2::{Pool, PooledConnection};
use redis::{Client, Commands};

/// Redis Pooled Connection
type Conn = PooledConnection<Client>;

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

    /// Set (string,string)
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        Ok(self.conn()?.set(key, value)?)
    }

    /// Get (string,string)
    pub fn get(&self, key: &str) -> Result<String> {
        Ok(self.conn()?.get(key)?)
    }
}
