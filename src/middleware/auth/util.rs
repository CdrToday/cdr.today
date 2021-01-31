//! Auth util
use crate::share::Shared;
use actix_web::Error;
use std::sync::MutexGuard;

pub fn set_uuid<'s>(data: &MutexGuard<'s, Shared>, address: &str, uuid: &str) -> Result<(), Error> {
    data.redis
        .set(address, uuid)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Set uuid into redis failed"))
}
