//!
//! Some attributes are only meaningful in a certain server configuration, like
//! using RocksDB instead of MMFiles or the server is setup in a cluster. Those
//! attributes are only available in the API of this crate if the crate is
//! compiled with the related feature enabled. The crate feature `mmfiles`
//! enables MMFiles related attributes, the crate feature `rocksdb` enables
//! RocksDB related attributes and the crate feature `cluster` enables cluster
//! related attributes.

#![doc(html_root_url = "https://docs.rs/arangodb_client/0.1.0")]

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

extern crate arangodb_core;
#[cfg(test)] extern crate arangodb_connector;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[cfg(not(test))] extern crate serde_json;
#[cfg(test)] #[macro_use] extern crate serde_json;

pub mod admin;
pub mod collection;
pub mod database;
pub mod document;
pub mod graph;
pub mod index;
pub mod user;
