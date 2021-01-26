//! Auth Token
use super::{error, header};
use actix_web::{
    dev::{HttpResponseBuilder, ServiceRequest},
    http::{
        header::{HeaderName, HeaderValue},
        StatusCode,
    },
    Error,
};
use uuid::Uuid;

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
pub fn token(req: &mut ServiceRequest) -> Result<(), Error> {
    if let Some(token) = req.headers().get(header::TOKEN) {
    } else {
        req.headers_mut().append(
            HeaderName::from_static(header::TOKEN),
            HeaderValue::from_str(&format!("{}", Uuid::new_v4()))?,
        );

        // Return 401
        return Err(error::AuthError::TokenNotFound.into());
    }

    // The token is valid
    Ok(())
}
