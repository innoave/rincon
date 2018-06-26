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
//! * [rincon_core] : Defines the common API for the driver and is used by the
//!   other crates.
//! * [rincon_connector] : Implements the communication layer of the driver.
//! * [rincon_client] : Implements the methods of the REST API provided by
//!   [ArangoDB].
//! * [rincon_session] : Provides a synchronous higher level API on top of
//!   [rincon_client].
//! * [rincon_test_helper] : Provides utilities used in integration tests with
//!   an [ArangoDB] server.
//!
//! This rincon crate is a meta crate that provides documentation on how to
//! use the multiple crates provided by the Rincon ArangoDB driver. It does not
//! provide any functionality itself. In the following we will look at examples
//! to show which crates to use for different scenarios.
//!
//! *...while I'm working on writing mentioned scenarios please see the
//! documentation of the [rincon_client] crate and the [rincon_session] crate
//! for how to use the Rincon ArangoDB driver.*
//!
//!
//! [ArangoDB]: https://www.arangodb.com
//! [AQL]: https://docs.arangodb.com/3.2/AQL/index.html
//! [rincon_core]: https://docs.rs/rincon_core
//! [rincon_connector]: https://docs.rs/rincon_connector
//! [rincon_client]: https://docs.rs/rincon_client
//! [rincon_session]: https://docs.rs/rincon_session
//! [rincon_session_async]: https://docs.rs/rincon_session_async
//! [rincon_aql]: https://docs.rs/rincon_aql
//! [rincon_test_helper]: https://docs.rs/rincon_test_helper

#![doc(html_root_url = "https://docs.rs/rincon/0.1.0")]
#![warn(
    missing_copy_implementations, missing_debug_implementations, missing_docs, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications
)]
