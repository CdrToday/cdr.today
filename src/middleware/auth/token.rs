//! Auth Token
use super::{error, header};
use crate::{crypto::Address, share::Shared};
use actix_web::{dev::ServiceRequest, web::Data, Error};
use redis::Commands;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// # No Token
///
/// Account doesn't have token when they login cdr.today for
/// the first time, as result, we'll return 401 and set a uuid
/// to the client.
///
/// # Has token
///
/// If have token in header, check the database to find if the
/// token is paired.
pub fn token(req: &ServiceRequest, address: &String) -> Result<(), Error> {
    if let Some(token) = req.headers().get(header::TOKEN) {
        let verifier = Address::from_str(&address).map_err(|_| error::AuthError::AddressInvalid)?;
        let uuid = super::uuid::uuid(req, address)?;
        if !verifier
            .verify(
                uuid.as_bytes(),
                &hex::decode(token)
                    .map_err(|_| error::AuthError::TokenInvalid { uuid: uuid.clone() })?,
            )
            .map_err(|e| {
                println!("{}", e);
                error::AuthError::AddressInvalid
            })?
        {
            Err(error::AuthError::TokenInvalid {
                uuid: Uuid::new_v4().to_string(),
            }
            .into())
        } else {
            Ok(())
        }
    } else {
        let uuid = Uuid::new_v4().to_string();
        if let Some(data) = req.app_data::<Data<Arc<Mutex<Shared>>>>() {
            let _: () = data
                .lock()
                .unwrap()
                .redis
                .conn()
                .map_err(|_| {
                    actix_web::error::ErrorInternalServerError("Get redis connection failed")
                })?
                .set(address, &uuid)
                .map_err(|_| {
                    actix_web::error::ErrorInternalServerError("Set uuid into redis failed")
                })?;
        }

        Err(error::AuthError::TokenNotFound { uuid }.into())
    }
}
