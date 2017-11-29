
#![doc(html_root_url = "https://docs.rs/rincon_aql/0.1.0")]

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

extern crate rincon_core;
extern crate rincon_client;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate serde_json;

pub mod aql;
pub mod cursor;
