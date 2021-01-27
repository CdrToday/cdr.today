//! GraphQL service
use crate::{schema::Account, share::Shared};
use actix_web::{error::InternalError, http::StatusCode, web, Error, HttpResponse};
use juniper::graphql_object;
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use std::sync::{Arc, Mutex};

pub async fn graphiql() -> core::result::Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}

pub async fn playground() -> core::result::Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}

pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    shared: web::Data<Arc<Mutex<Shared>>>,
) -> Result<HttpResponse, Error> {
    if let Ok(share) = shared.try_lock() {
        graphql_handler(&share.root_node, &share, req, payload).await
    } else {
        Err(InternalError::new("lock shared data failed", StatusCode::SERVICE_UNAVAILABLE).into())
    }
}

/// GraphQL function set
pub struct Query;

#[graphql_object(context = Shared)]
impl Query {
    fn version() -> String {
        "1.0".to_string()
    }

    #[graphql(arguments(addr(description = "address of the user")))]
    fn account(shared: &Shared, addr: String) -> Option<Account> {
        Account::first(&shared.pg.conn().ok()?, addr).ok()
    }
}
