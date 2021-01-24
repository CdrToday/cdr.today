//! Acitx App
use crate::{graphql, share::Shared, Config, Result};
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use std::sync::{Arc, Mutex};

/// Serve actix App
pub async fn serve(config: Config) -> Result<()> {
    let http_url = config.http.url();
    let data = Arc::new(Mutex::new(Shared::new(config.clone())?));
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .data(data.clone())
            .service(
                web::resource("/graphgl")
                    .route(web::post().to(graphql::graphql_route))
                    .route(web::get().to(graphql::graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(graphql::playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphql::graphiql_route)))
    })
    .bind(&http_url)?
    .run()
    .await?;

    Ok(())
}
