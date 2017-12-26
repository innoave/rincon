
#![doc(html_root_url = "https://docs.rs/rincon_connector/0.1.0")]

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

extern crate failure;
extern crate futures;
extern crate hyper;
extern crate hyper_timeout;
extern crate hyper_tls;
#[macro_use] extern crate log;
extern crate native_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

extern crate rincon_core;

pub mod datasource;
pub mod connection;
