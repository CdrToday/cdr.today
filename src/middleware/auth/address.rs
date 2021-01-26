//! Auth address
use super::{error, header};
use actix_web::{dev::ServiceRequest, Error};

pub fn address(req: &ServiceRequest) -> Result<String, Error> {
    if let Some(address) = req.headers().get(header::ADDRESS) {
        if address.len() != 44 {
            return Err(error::AuthError::AddressInvalid.into());
        }
        Ok("".to_string())
    } else {
        Err(error::AuthError::AddressNotFound.into())
    }
}
