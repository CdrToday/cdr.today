//! cdr.today poc
//!
//! Quick version
mod app;
mod cli;
pub mod config;
pub mod crypto;
mod db;
mod middleware;
mod result;
pub mod schema;
mod service;
mod share;

#[macro_use]
extern crate diesel;

pub use self::{
    cli::Opt,
    config::Config,
    result::{Error, Result},
};
