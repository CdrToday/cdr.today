use actix_web::{web, Error, HttpResponse};
use juniper::{
    tests::fixtures::starwars::schema::{Database, Query},
    EmptyMutation, EmptySubscription, RootNode,
};
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};

/// GraphQL schema
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}

/// Mock the GraphQL schema
pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

/// GraphQL APIs
pub async fn graphiql_handler() -> Result<HttpResponse, Error> {
    gqli_handler("/", None).await
}

/// GraphQL playground
pub async fn playground_handler() -> Result<HttpResponse, Error> {
    play_handler("/", None).await
}

/// GraphQL Service
pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}
