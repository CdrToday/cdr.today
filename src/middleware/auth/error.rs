//! Auth Error
use super::header;
use actix_web::{dev::HttpResponseBuilder, error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

static UUID_NOT_FOUND: &str =
    "Uuid not found in header CDR-TODAY-UUID, sign the returned UUID with ED25519 secret key.";
static UUID_INVALID: &str =
    "Uuid invalid in header CDR-TODAY-UUID, check your token in CDR-TODAY-TOKEN";
static TOKEN_NOT_FOUND: &str =
    "Token not found in header CDR-TODAY-TOKEN, sign the returned UUID with ED25519 secret key.";
static TOKEN_INVALID: &str =
    "Token invalid in header CDR-TODAY-TOKEN, check your address in CDR-TODAY-ADDRESS.";
static ADDRESS_NOT_FOUND: &str =
    "Address not found in header CDR-TODAY-ADDRESS, should be a ED25519 public key in base58.";
static ADDRESS_INVALID: &str =
    "Address in header CDR-TODAY-ADDRESS is invalid, should be a ED25519 public key in base58.";

/// Auth Error
#[derive(Debug, Display, Error)]
pub enum AuthError {
    /// 401, uuid not found, return a uuid for signing
    #[display(fmt = "{}", UUID_NOT_FOUND)]
    UuidNotFound { uuid: String },
    /// 401, uuid invalid, return a uuid for signing
    #[display(fmt = "{}", UUID_INVALID)]
    UuidInvalid { uuid: String },
    /// 401, token not found, return a uuid for signing
    #[display(fmt = "{}", TOKEN_NOT_FOUND)]
    TokenNotFound { uuid: String },
    /// 401, token invalid, return a uuid for signing
    #[display(fmt = "{}", TOKEN_INVALID)]
    TokenInvalid { uuid: String },
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
            Self::TokenNotFound { uuid } => builder
                .set_header(header::UUID, uuid.to_string())
                .body(&self.to_string()),
            _ => builder.body(&self.to_string()),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
