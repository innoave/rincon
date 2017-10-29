//!
//! Some attributes are only meaningful in a certain server configuration, like
//! using RocksDB instead of MMFiles or the server is setup in a cluster. Those
//! attributes are only available in the API of this crate if the crate is
//! compiled with the related feature enabled. The crate feature `mmfiles`
//! enables MMFiles related attributes, the crate feature `rocksdb` enables
//! RocksDB related attributes and the crate feature `cluster` enables cluster
//! related attributes.

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
)]

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
pub mod aql;
pub mod authentication;
pub mod collection;
pub mod cursor;
pub mod database;
pub mod document;
pub mod index;
pub mod user;

mod consts;
