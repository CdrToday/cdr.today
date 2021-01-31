//! CDR-TODAY-UUID
use super::{error, header};
use crate::share::Shared;
use actix_web::{dev::ServiceRequest, web::Data, Error};
use redis::Commands;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// # No UUID
///
/// Could not find UUID while verifying token
///
/// # UUID Invalid
///
/// UUID and the token not paired.
pub fn uuid(req: &ServiceRequest, address: &String) -> Result<String, Error> {
    if let Some(uuid) = req.headers().get(header::UUID) {
        let new_uuid = Uuid::new_v4().to_string();
        let uuid = uuid.to_str().map_err(|_| error::AuthError::UuidInvalid {
            uuid: new_uuid.clone(),
        })?;

        if let Some(data) = req.app_data::<Data<Arc<Mutex<Shared>>>>() {
            let stored_uuid: String = data
                .lock()
                .unwrap()
                .redis
                .conn()
                .map_err(|e| {
                    actix_web::error::ErrorInternalServerError("Get redis connection failed")
                })?
                .get(address)
                .map_err(|e| {
                    actix_web::error::ErrorInternalServerError("Get address from redis failed")
                })?;

            if uuid != stored_uuid {
                Err(error::AuthError::UuidInvalid { uuid: new_uuid }.into())
            } else {
                Ok(uuid.to_string())
            }
        } else {
            Err(actix_web::error::ErrorInternalServerError(
                "Could not find shared data",
            ))
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
            Err(error::AuthError::UuidNotFound { uuid }.into())
        } else {
            Err(actix_web::error::ErrorInternalServerError(
                "Could not find shared data",
            ))
        }
    }
}
