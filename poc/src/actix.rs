//! Acitx App
use crate::{share::Shared, Config, Result};
use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

/// Serve actix App
pub async fn serve(config: Config) -> Result<()> {
    let http_url = config.http.url();
    HttpServer::new(move || {
        App::new()
            .data(Shared::new(config.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(index)
    })
    .bind(&http_url)?
    .run()
    .await?;

    Ok(())
}
