//! GraphQL
use crate::{orm::Orm, schema::Account, share::Shared};
use actix_web::{error::InternalError, http::StatusCode, web, Error, HttpResponse};
use juniper::graphql_object;
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use std::sync::{Arc, Mutex};

pub async fn graphiql_route() -> core::result::Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}

pub async fn playground_route() -> core::result::Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    shared: web::Data<Arc<Mutex<Shared>>>,
) -> Result<HttpResponse, Error> {
    if let Ok(share) = shared.try_lock() {
        graphql_handler(&share.root_node, &share.orm, req, payload).await
    } else {
        Err(InternalError::new("lock shared data failed", StatusCode::SERVICE_UNAVAILABLE).into())
    }
}

/// GraphQL function set
pub struct Query;

#[graphql_object(context = Orm)]
impl Query {
    fn version() -> String {
        "1.0".to_string()
    }

    #[graphql(arguments(addr(description = "address of the user")))]
    fn account(orm: &Orm, addr: String) -> Option<Account> {
        Account::first(&orm.conn().ok()?, addr).ok()
    }
}
