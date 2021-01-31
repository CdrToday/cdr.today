//! Shared data
use crate::{
    db::{Pg, Redis},
    schema::{Account, Schema},
    service::graphql::Query,
    Config, Result,
};
use juniper::{EmptyMutation, EmptySubscription, RootNode};

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
