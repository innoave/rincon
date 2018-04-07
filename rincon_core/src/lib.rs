
#![doc(html_root_url = "https://docs.rs/rincon_core/0.1.0")]

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

#[macro_use] extern crate failure;
extern crate futures;
extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate url;

pub mod api;
pub mod arango;

const LIB_NAME: &str = "rincon";
const LIB_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const LIB_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const LIB_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
const LIB_VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");
const LIB_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
