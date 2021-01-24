//! Shared data
use crate::{
    graphql::Query,
    orm::Orm,
    schema::{Account, Schema},
    Config, Result,
};
use juniper::{Context, EmptyMutation, EmptySubscription, RootNode};

/// Shared data
pub struct Shared {
    pub orm: Orm,
    pub config: Config,
    pub root_node: RootNode<'static, Query, EmptyMutation<Orm>, EmptySubscription<Orm>>,
}

impl Context for Shared {}

impl Shared {
    fn gather_tables() -> Vec<(&'static str, Vec<&'static str>)> {
        vec![<Account as Schema>::table()]
    }

    /// New shared data
    pub fn new(config: Config) -> Result<Shared> {
        Ok(Shared {
            orm: Orm::new(&config)?.create_tables(Self::gather_tables())?,
            config,
            root_node: RootNode::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        })
    }
}
