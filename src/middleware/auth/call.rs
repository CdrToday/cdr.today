//! Auth call
use crate::middleware::{
    auth::{address::address, token::token},
    util,
};
use actix_web::{dev::ServiceRequest, Error};

/// Auth calling service request
pub fn call(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    let data = util::data(&req)?;
    let headers = req.headers();
    token(&data, &headers, &address(&headers)?)?;
    drop(data);
    Ok(req)
}
