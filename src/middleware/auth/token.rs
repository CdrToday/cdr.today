//! Auth Token
use crate::{
    crypto::Address,
    middleware::{
        auth::{error, header, util::set_uuid},
        util,
    },
};
use actix_web::{dev::ServiceRequest, Error};
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
pub fn token(req: &ServiceRequest, address: &String) -> Result<(), Error> {
    let data = util::data(req)?;
    if let Some(token) = req.headers().get(header::TOKEN) {
        let verifier = Address::from_str(&address).map_err(|_| error::AuthError::AddressInvalid)?;
        let uuid = super::uuid::uuid(req, address)?;

        // map token bytes
        let token_bytes = match hex::decode(token) {
            Err(_) => {
                set_uuid(&data, &address, &uuid).unwrap_or_default();
                return Err(error::AuthError::TokenInvalid { uuid: uuid.clone() }.into());
            }
            Ok(b) => b,
        };

        // Verify the signature
        if match verifier.verify(uuid.as_bytes(), &token_bytes) {
            Err(_) => {
                set_uuid(&data, &address, &uuid).unwrap_or_default();
                return Err(error::AuthError::UuidInvalid { uuid: uuid.clone() }.into());
            }
            Ok(r) => r,
        } {
            Ok(())
        } else {
            set_uuid(&data, &address, &uuid)?;
            Err(error::AuthError::TokenInvalid {
                uuid: Uuid::new_v4().to_string(),
            }
            .into())
        }
    } else {
        let uuid = Uuid::new_v4().to_string();
        set_uuid(&data, &address, &uuid)?;
        Err(error::AuthError::TokenNotFound { uuid }.into())
    }
}
