//! Middleware util
use crate::share::{block, Share, Shared};
use actix_web::{dev::ServiceRequest, Error};
use std::sync::MutexGuard;

/// Get app data from `ServiceRequest`
pub fn data(req: &ServiceRequest) -> Result<MutexGuard<'_, Shared>, Error> {
    if let Some(data) = req.app_data::<Share>() {
        Ok(block(data))
    } else {
        Err(actix_web::error::ErrorInternalServerError(
            "Could not find shared data",
        ))
    }
}
