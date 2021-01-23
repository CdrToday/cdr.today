//! cdr.today poc
//!
//! Quick version
mod actix;
mod cli;
pub mod config;
pub mod orm;
mod result;
pub mod schema;
mod share;

#[macro_use]
extern crate diesel;

pub use self::{
    cli::Opt,
    config::Config,
    result::{Error, Result},
};
