//! Auth Token
use super::{error, header};
use actix_web::{dev::ServiceRequest, Error};

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
pub fn token(req: &ServiceRequest, _address: String) -> Result<(), Error> {
    if let Some(_token) = req.headers().get(header::TOKEN) {
        Ok(())
    } else {
        Err(error::AuthError::TokenNotFound.into())
    }
}
