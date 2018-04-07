//! *Rincon* is an [ArangoDB] driver to operate with an [ArangoDB] server via
//! its REST API from within Rust applications.
//!
//! The *Rincon* [ArangoDB] driver is split up into multiple crates to pick
//! and choose the level of abstraction an applications wants to use.
//! Additionally the split into multiple crates enable us to flexible extend the
//! driver.
//!
//! The provided crates are:
//!
//! * [rincon_core] : Defines the common API for the driver and is used by the other crates.
//! * [rincon_client] : Implements the methods of the REST API provided by [ArangoDB].
//! * [rincon_connector] : Implements the communication layer of the driver.
//! * [rincon_session] : Provides a synchronous higher level API on top of [rincon_client].
//! * [rincon_test_helper] : Provides utilities used in integration tests with an [ArangoDB] server.
//!
//! This `rincon` crate is a meta crate that provides documentation on how to
//! use the multiple crates provided by this project. It does not provide any
//! functionality itself. In the following examples are given which crates to
//! use for different scenarios.
//!
//! # Synchronous Session API
//!
//! The synchronous session API is the most convenient way to use this driver.
//! It is provided by the `rincon_session` crate.
//!
//! Here is some example code to showcase the basic usage of the session API:
//!
//! ```rust,dont_run
//! # extern crate rincon_core;
//! # extern crate rincon_connector;
//! # extern crate rincon_session;
//! # extern crate rincon_test_helper;
//! # #[macro_use] extern crate serde_derive;
//! # extern crate tokio_core;
//! # use rincon_core::api::connector::Connector;
//! # use rincon_core::api::datasource::DataSource;
//! # use rincon_connector::http::BasicConnector;
//! # use rincon_session::{ArangoSession, Document, Result};
//! # use tokio_core::reactor::Core;
//! #
//! # fn main() {
//! #    let datasource = DataSource::from_url("http://localhost:8529").unwrap()
//! #        .with_basic_authentication("root", "s3cur3");
//! #
//! #    let mut core = Core::new().unwrap();
//! #
//! #    let connector = BasicConnector::new(datasource, &core.handle()).unwrap();
//! #
//! #    let session = ArangoSession::new(connector, core);
//! #
//! #    fn create_doc<C>(session: ArangoSession<C>) -> Result<()>
//! #        where C: 'static + Connector
//! #    {
//! #
//! #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u16,
//! }
//!
//! let person = Person { name: "herbert".to_string(), age: 42 };
//!
//! let friendsbook = session.use_database_with_name("friendsbook");
//! let people = friendsbook.create_collection("people")?;
//!
//! let doc_header = people.insert_document(person)?;
//!
//! let (_, doc_key, _) = doc_header.deconstruct();
//! let document: Document<Person> = people.get_document(doc_key)?;
//! #
//! #        Ok(())
//! #     }
//! # }
//! ```
//!
//! If you have a closer look at this example you may ask what this `session` is
//! that we are accessing in the second `let` statement. This is an
//! `ArangoSession` instance from the `rincon_session` API. Lets create such
//! an `ArangoSession` instance step by step.
//!
//! To use the `rincon_session` API add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rincon_core = "0.1"
//! rincon_session = "0.1"
//! rincon_connector = "0.1"
//! tokio-core = "0.1"
//! ```
//!
//! And in your crate root add this:
//!
//! ```rust
//! extern crate rincon_core;
//! extern crate rincon_connector;
//! extern crate rincon_session;
//! extern crate tokio_core;
//! ```
//!
//! The `tokio_core` crate is needed for instantiating a `Connector` as we will
//! see in a moment.
//!
//! First we configure a `DataSource` for our [ArangoDB] server. A `DataSource`
//! is a struct that holds the parameters needed to connect to a concrete
//! installation of [ArangoDB].
//!
//! ```rust
//! # extern crate rincon_core;
//! use rincon_core::api::datasource::DataSource;
//!
//! fn main() {
//!     let datasource = DataSource::from_url("http://localhost:8529").unwrap()
//!         .with_basic_authentication("root", "s3cur3");
//! }
//! ```
//!
//! Next we create a `Connector` which will be used to communicate with the
//! [ArangoDB] server. The `Connector` defines which transport protocol is used
//! and how the payload is serialized. We choose the provided `BasicConnector`
//! which support HTTP and HTTPS and serializes the payload as JSON.
//!
//! ```rust
//! # extern crate rincon_core;
//! # extern crate rincon_connector;
//! # extern crate rincon_test_helper;
//! # extern crate tokio_core;
//! use rincon_core::api::datasource::DataSource;
//! use rincon_connector::http::BasicConnector;
//! use tokio_core::reactor::Core;
//!
//! fn main() {
//!     let datasource = DataSource::from_url("http://localhost:8529").unwrap()
//!         .with_basic_authentication("root", "s3cur3");
//!
//!     let mut core = Core::new().unwrap();
//!
//!     let connector = BasicConnector::new(datasource, &core.handle()).unwrap();
//! }
//! ```
//!
//! The `new()` method of the `BasicConnector` takes 2 arguments. The first
//! argument is the datasource we have just created before. The second argument
//! is a handle to `reactor::Core` from the `tokio_core` crate.
//!
//! And finally we create an `ArangoSession`:
//!
//! ```rust
//! # extern crate rincon_core;
//! # extern crate rincon_connector;
//! # extern crate rincon_session;
//! # extern crate rincon_test_helper;
//! # extern crate tokio_core;
//! use rincon_core::api::datasource::DataSource;
//! use rincon_connector::http::BasicConnector;
//! use rincon_session::ArangoSession;
//! use tokio_core::reactor::Core;
//!
//! fn main() {
//!     let datasource = DataSource::from_url("http://localhost:8529").unwrap()
//!         .with_basic_authentication("root", "s3cur3");
//!
//!     let mut core = Core::new().unwrap();
//!
//!     let connector = BasicConnector::new(datasource, &core.handle()).unwrap();
//!
//!     let session = ArangoSession::new(connector, core);
//! }
//! ```
//!
//! Now we are ready to conveniently interact with the [ArangoDB] server as
//! shown in the example at the beginning of this chapter.
//!
//!
//! [ArangoDB]: https://www.arangodb.org
//! [AQL]: https://docs.arangodb.com/3.2/AQL/index.html
//! [rincon_core]: https://docs.rs/rincon_core
//! [rincon_client]: https://docs.rs/rincon_client
//! [rincon_connector]: https://docs.rs/rincon_connector
//! [rincon_session]: https://docs.rs/rincon_session
//! [rincon_session_async]: https://docs.rs/rincon_session_async
//! [rincon_aql]: https://docs.rs/rincon_aql
//! [rincon_test_helper]: https://docs.rs/rincon_test_helper

#![doc(html_root_url = "https://docs.rs/rincon/0.1.0")]

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
