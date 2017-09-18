
extern crate futures;
extern crate hyper;
extern crate hyper_timeout;
extern crate hyper_tls;
#[macro_use] extern crate log;
extern crate native_tls;
extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

pub mod api;
pub mod connection;
pub mod datasource;

pub mod admin;
pub mod collection;
pub mod database;
pub mod user;
