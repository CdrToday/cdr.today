//! Auth call
use super::{address::address, token::token};
use actix_web::{dev::ServiceRequest, Error};

/// Auth calling service request
pub fn call(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    // if !req.path().contains("/graph") {
    token(&req, &address(&req)?)?;
    // }
    Ok(req)
}
