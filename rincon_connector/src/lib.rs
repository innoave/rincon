//! Connecting to an [ArangoDB] server
//!
//! A connector implements the communication layer of the driver. It knows how
//! to talk to an [ArangoDB] server, by using a transport protocol like HTTP or
//! HTTPS and a serialization format like [JSON] or [VelocyPack].
//!
//! The `Connector` trait is defined in the [`rincon_core`] API. This crate
//! provides some default implementations to be used out of the box.
//!
//! Currently there is only one `Connector` implementation provided:
//!
//! * `BasicConnector` : using [JSON] over HTTP/HTTPS
//!
//! but more are planned to be added in the future.
//!
//! # Example
//!
//! Using a `Connector` is straight forward. Here is an example of how to use
//! the `BasicConnector`. Other `Connector`s are used in a similar way.
//!
//! First we create an instance by calling the `new()` function. The `new()`
//! function takes a `DataSource` and a handle of a `reactor::Core` as
//! parameters.
//!
//! The `DataSource` is a data struct that holds the parameters needed to
//! connect to an [ArangoDB] server. It is defined by the [`rincon_core`] API.
//! The `reactor::Core` is taken from the [`tokio-core`] crate.
//!
//! ```rust
//! # extern crate rincon_core;
//! # extern crate rincon_connector;
//! # extern crate tokio_core;
//! use rincon_core::api::connector::Error;
//! use rincon_core::api::datasource::DataSource;
//! use rincon_connector::http::BasicConnector;
//! use tokio_core::reactor::Core;
//!
//! # fn main() {
//! #     let connector = create_connector().unwrap();
//! # }
//!
//! fn create_connector() -> Result<BasicConnector, Error> {
//!
//!     let datasource = DataSource::from_url("http://localhost:8529")
//!         .expect("invalid URL for datasource")
//!         .with_basic_authentication("root", "s3cur3");
//!
//!     let mut core = Core::new()?;
//!
//!     let connector = BasicConnector::new(datasource, &core.handle());
//!     connector
//! }
//! ```
//!
//! The connector we just created is used to execute method calls of the
//! [`rincon_client`] API or is passed to the [`rincon_session`] API dependant
//! on what API you want to use. For more details see the documentation of
//! those crates.
//!
//! [ArangoDB]: https://www.arangodb.org
//! [JSON]: https://json.org
//! [`rincon_core`]: https://docs.rs/rincon_core
//! [`rincon_client`]: https://docs.rs/rincon_client
//! [`rincon_session`]: https://docs.rs/rincon_session
//! [`tokio_core`]: https://docs.rs/tokio-core
//! [VelocyPack]: https://github.com/arangodb/velocypack

#![doc(html_root_url = "https://docs.rs/rincon_connector/0.1.0")]

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

pub mod http;
