//! ST API
use crate::{middleware::Auth, result::Result};
use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};

mod graphql;

/// Serve the http server
pub async fn serve(port: u16) -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(graphql::schema())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Auth::default())
            .wrap(
                Cors::default()
                    .allowed_origin("send_wildcard")
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/")
                    .route(web::post().to(graphql::graphql))
                    .route(web::get().to(graphql::graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(graphql::playground_handler)))
            .service(web::resource("/graphiql").route(web::get().to(graphql::graphiql_handler)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
