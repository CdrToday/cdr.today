//! ST API
use crate::{middleware::Auth, Error};
use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

/// Serve the http server
pub async fn serve(port: u16) -> Result<(), Error> {
    HttpServer::new(|| App::new().service(index).wrap(Logger::default()).wrap(Auth))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await?;

    Ok(())
}
