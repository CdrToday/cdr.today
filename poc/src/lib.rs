//! cdr.today poc
//!
//! Quick version
mod cli;
mod cmd;
pub mod config;
mod net;
pub mod orm;
mod result;
pub mod schema;

#[macro_use]
extern crate diesel;

pub use self::{
    cli::Opt,
    config::Config,
    result::{Error, Result},
};
