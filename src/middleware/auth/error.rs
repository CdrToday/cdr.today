//! Auth Error
use super::header;
use actix_web::{dev::HttpResponseBuilder, error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use uuid::Uuid;

static TOKEN_NOT_FOUND: &str =
    "Token not found in header CDR-TODAY-TOKEN, sign the returned UUID with ED25519 secret key.";
static ADDRESS_NOT_FOUND: &str =
    "Address not found in header CDR-TODAY-ADDRESS, should be a ED25519 public key in base58.";
static ADDRESS_INVALID: &str =
    "Address in header CDR-TODAY-ADDRESS is invalid, should be a ED25519 public key in base58.";

/// Auth Error
#[derive(Debug, Display, Error)]
pub enum AuthError {
    /// 401, token not found, return a uuid for signing
    #[display(fmt = "{}", TOKEN_NOT_FOUND)]
    TokenNotFound,
    /// 401, address not found
    #[display(fmt = "{}", ADDRESS_NOT_FOUND)]
    AddressNotFound,
    /// 401, address invalid
    #[display(fmt = "{}", ADDRESS_INVALID)]
    AddressInvalid,
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponseBuilder::new(self.status_code());
        match self {
            Self::TokenNotFound => builder
                .set_header(header::UUID, Uuid::new_v4().to_string())
                .body(&self.to_string()),
            _ => builder.body(&self.to_string()),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
