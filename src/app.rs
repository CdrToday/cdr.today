//! Acitx App
use crate::{service::graphql, share::Shared, Config, Result};
use actix_cors::Cors;
use actix_web::{
    dev::ServiceRequest, get, http::header, middleware, web, App, Error, HttpResponse, HttpServer,
};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use std::sync::{Arc, Mutex};

async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> core::result::Result<ServiceRequest, Error> {
    eprintln!("{:?}", credentials);
    Ok(req)
}

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
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(HttpAuthentication::bearer(ok_validator))
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
