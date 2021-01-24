//! GraphQL
use crate::{orm::Orm, schema::Account, share::Shared};
use actix_web::{web, Error, HttpResponse};
use juniper::graphql_object;
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

pub async fn graphiql_route() -> core::result::Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}

pub async fn playground_route() -> core::result::Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    shared: web::Data<Shared>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&shared.root_node, &shared.orm, req, payload).await
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
