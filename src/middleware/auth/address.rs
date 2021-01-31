//! Auth address
use super::{error, header};
use actix_web::{http::header::HeaderMap, Error};

/// Check address
pub fn address(headers: &HeaderMap) -> Result<String, Error> {
    if let Some(address) = headers.get(header::ADDRESS) {
        if address.len() != 44 {
            return Err(error::AuthError::AddressInvalid.into());
        }

        Ok(address
            .to_str()
            .map_err(|_| error::AuthError::AddressInvalid)?
            .to_string())
    } else {
        Err(error::AuthError::AddressNotFound.into())
    }
}
