//! CDR-TODAY-UUID
use crate::{
    middleware::auth::{error, header, util::set_uuid},
    share::Shared,
};
use actix_web::{http::header::HeaderMap, Error};
use std::sync::MutexGuard;
use uuid::Uuid;

/// # No UUID
///
/// Could not find UUID while verifying token
///
/// # UUID Invalid
///
/// UUID and the token not paired.
pub fn uuid<'u>(
    data: &MutexGuard<'u, Shared>,
    headers: &HeaderMap,
    address: &str,
) -> Result<String, Error> {
    if let Some(uuid) = headers.get(header::UUID) {
        let new_uuid = Uuid::new_v4().to_string();
        let uuid = uuid.to_str().map_err(|_| error::AuthError::UuidInvalid {
            uuid: new_uuid.clone(),
        })?;

        let stored_uuid: String = data.redis.get(address).map_err(|_| {
            actix_web::error::ErrorInternalServerError("Get address from redis failed")
        })?;

        if uuid != stored_uuid {
            set_uuid(&data, address, &new_uuid)?;
            Err(error::AuthError::UuidInvalid { uuid: new_uuid }.into())
        } else {
            Ok(uuid.to_string())
        }
    } else {
        let uuid = Uuid::new_v4().to_string();
        set_uuid(&data, address, &uuid)?;
        Err(error::AuthError::UuidNotFound { uuid }.into())
    }
}
