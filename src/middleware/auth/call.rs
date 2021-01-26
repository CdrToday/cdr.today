//! Auth call
use super::{header::TOKEN, token::token};
use actix_web::{dev::ServiceRequest, Error};

/// Auth calling service request
pub fn call(mut req: ServiceRequest) -> Result<ServiceRequest, Error> {
    // Check token
    token(&mut req)?;
    println!("{:?}", req.headers().get(TOKEN));
    Ok(req)
}
