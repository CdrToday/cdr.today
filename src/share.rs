//! Shared data
use crate::{
    db::{Pg, Redis},
    schema::{Account, Schema},
    service::graphql::Query,
    Config, Result,
};
use actix_web::web::Data;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
    time::Duration,
};

/// Shared data
pub struct Shared {
    pub pg: Pg,
    pub redis: Redis,
    pub config: Config,
    pub root_node: RootNode<'static, Query, EmptyMutation<Pg>, EmptySubscription<Pg>>,
}

impl Shared {
    /// Init tables
    fn gather_tables() -> Vec<(&'static str, Vec<&'static str>)> {
        vec![<Account as Schema>::table()]
    }

    /// New shared data
    pub fn new(config: Config) -> Result<Shared> {
        Ok(Shared {
            pg: Pg::new(&config)?.create_tables(Self::gather_tables())?,
            redis: Redis::new(&config)?,
            config,
            root_node: RootNode::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        })
    }
}

/// Shared data in actix_web
pub type Share = Data<Arc<Mutex<Shared>>>;

/// Block Mutex
pub fn block(share: &Share) -> MutexGuard<'_, Shared> {
    loop {
        if let Ok(share) = share.try_lock() {
            return share;
        } else {
            thread::sleep(Duration::from_secs(1));
            continue;
        }
    }
}
