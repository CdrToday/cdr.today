//! cdr.today poc
//!
//! Quick version
mod cli;
mod cmd;
pub mod config;
mod net;
pub mod orm;
mod result;
mod scheme;

pub use self::{
    cli::Opt,
    config::Config,
    result::{Error, Result},
};
