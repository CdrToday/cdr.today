//! Acitx App
use crate::{middleware, service::graphql, share::Shared, Config, Result};
use actix_cors::Cors;
use actix_web::{get, http::header, web, App, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Serve actix App
pub async fn serve(config: Config) -> Result<()> {
    let http_url = config.http.url();
    let data = Arc::new(Mutex::new(Shared::new(config.clone())?));
    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(middleware::Auth)
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .send_wildcard()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphgl")
                    .route(web::post().to(graphql::graphql))
                    .route(web::get().to(graphql::graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(graphql::playground)))
            .service(web::resource("/graphiql").route(web::get().to(graphql::graphiql)))
            .service(index)
    })
    .bind(&http_url)?
    .run()
    .await?;

    Ok(())
}
