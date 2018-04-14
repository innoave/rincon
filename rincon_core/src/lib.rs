//! Core types of the rincon ArangoDB driver
//!
//! `rincon_core` defines an types, traits and constants that are common to the
//! other crates of the rincon driver project. This `rincon_core` driver API
//! enables the modular design of the driver.
//!
//! The main parts of the API are:
//!
//! * `datasource` : the `DataSource` struct holds the parameters needed by `Connector`s
//! * `connector` : a `Connector` defines how the driver communicates with an ArangoDB server
//! * `auth` : types used to define the authentication method and credentials
//! * `method` : defines the traits `Method`, `Prepare` and `Execute` that need
//!   to be implemented by all methods for the ArangoDB REST API in order that
//!   they can be executed by a `Connection` of a `Connector`.
//! * `query` : the `Query` struct holds AQL-queries with query-parameters
//! * `types` : defines common types, such as Url, Value, JsonValue and JsonString
//! * `arango` : defines constants of values used by the ArangoDB REST API
//!
//! By defining this driver internal API the driver can be easily extended with
//! new methods that may be coming in newer version of ArangoDB and use
//! different implementations of `Connector`s while the `Method`s of the REST
//! API are implemented only once.

#![doc(html_root_url = "https://docs.rs/rincon_core/0.1.0")]

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
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
