//! Auth Error
use super::header;
use actix_web::{dev::HttpResponseBuilder, error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use uuid::Uuid;

/// Auth Error
#[derive(Debug, Display, Error)]
pub enum AuthError {
    /// 401, Return a uuid for signing
    #[display(fmt = "Token not found in header CDR-TODAY-TOKEN, please sign the returned UUID.")]
    TokenNotFound,
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponseBuilder::new(self.status_code());
        match self {
            AuthError::TokenNotFound => builder
                .set_header(header::UUID, Uuid::new_v4().to_string())
                .body(&self.to_string()),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::TokenNotFound => StatusCode::UNAUTHORIZED,
        }
    }
}
