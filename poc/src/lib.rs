//! cdr.today poc
//!
//! Quick version
mod cli;
mod cmd;
pub mod config;
mod net;
mod orm;
pub mod result;
mod scheme;

pub use self::cli::Opt;
