// palantir
// HTTP REST API reverse proxy
// Copyright: 2018, Maani Beigy <manibeygi@gmail.com>, 
//                  AASAAM Software Group <info@aasaam.com>
// License: MIT/Apache-2.0
//! # palantir
//!
//! `palantir` is a HTTP REST API reverse proxy. It performs load balance,
//! caching, and health check; and also prevents DDOS and reports metrics 
//! concerning health status of backend servers.
//! 
// ----------------------------- bring Modules --------------------------------
pub mod proxy;
// mod pool;
pub mod config;
pub mod connection;
// mod health;
// mod cache;
// mod header;
// mod metrics;
// ------------------ bring external libraries/crates -------------------------
extern crate actix_web;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate toml;
// ------------------ bring external functions/traits -------------------------
pub use std::ops::Deref;
pub use std::str::FromStr;
pub use log::LevelFilter;
// ------------------ bring internal functions/traits -------------------------
pub use crate::config::logger::ConfigLogger;
pub use crate::connection::connection::connect_upstream;
pub use crate::connection::appargs;
pub use crate::proxy::proxy::PalantirProxy;
//use pool::pool::ThreadPool;
// ---------------------- main functions of palantir --------------------------
/// This function ensures all statics are valid (a `deref` is enough to lazily 
/// initialize them)
pub fn ensure_states() {
    let (_, _) = (appargs::APP_ARGS.deref(), appargs::APP_CONF.deref());
}
