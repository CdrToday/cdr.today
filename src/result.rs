//! Cdr.today Result

use actix_web::Error as ActixWeb;
use std::result::Result as StdResult;
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as Io,
};

macro_rules! error {
    ($($e:ident),*) => {
        /// ST Errors
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            $($e(String),)+
        }

        impl Display for Error {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $(Error::$e(e) => e.fmt(f),)+
                }
            }
        }

        impl ErrorTrait for Error {}

        $(
            impl From<$e> for Error {
                fn from(e: $e) -> Error {
                    Error::$e(format!("{}", e))
                }
            }
        )+
    };
}

error!(Io);

/// Cdr.Today Result
pub type Result<T> = StdResult<T, Error>;

/// ActixWeb Result
pub type HttpResult<T> = StdResult<T, ActixWeb>;
