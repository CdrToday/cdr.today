//! Auth Token
use crate::{
    crypto::Address,
    middleware::auth::{error, header, util::set_uuid},
    share::Shared,
};
use actix_web::{http::header::HeaderMap, Error};
use std::sync::MutexGuard;
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
pub fn token<'t>(
    data: &MutexGuard<'t, Shared>,
    headers: &HeaderMap,
    address: &String,
) -> Result<(), Error> {
    if let Some(token) = headers.get(header::TOKEN) {
        let verifier = Address::from_str(&address).map_err(|_| error::AuthError::AddressInvalid)?;
        let uuid = super::uuid::uuid(&data, headers, address)?;

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
