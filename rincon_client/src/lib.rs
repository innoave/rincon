//! Type safe interaction with the ArangoDB REST API.
//!
//! This module defines types and functions to interact with the REST API of the
//! [ArangoDB] server. Lets define the concept and terms used throughout the
//! Rincon ArangoDB driver crates.
//!
//! Each operation on the REST API is represented by a struct in this client
//! lib. These structs are called methods within the Rincon ArangoDB driver. A
//! method is instantiated with the desired parameters and data to get a method
//! call. The method call is executed against an ArangoDB server on a connection
//! that is provided by a connector.
//!
//! Here is some example code to showcase the basic usage of the client API:
//!
//! ```rust,dont_run
//! # extern crate rincon_core;
//! # extern crate rincon_connector;
//! # extern crate rincon_client;
//! # #[macro_use] extern crate serde_derive;
//! # extern crate tokio_core;
//! # use rincon_core::api::connector::{Connector, Error, Execute};
//! # use rincon_core::api::datasource::DataSource;
//! # use rincon_connector::http::JsonHttpConnector;
//! # use rincon_client::collection::methods::CreateCollection;
//! # use rincon_client::document::methods::{GetDocument, InsertDocument};
//! # use rincon_client::document::types::{Document, NewDocument};
//! # use tokio_core::reactor::Core;
//! # use std::str::FromStr;
//! #
//! # fn main() {
//! #    let datasource = DataSource::from_str("http://localhost:8529")
//! #        .expect("invalid URL for datasource")
//! #        .with_basic_authentication("root", "s3cur3");
//! #
//! #    let mut core = Core::new().unwrap();
//! #
//! #    let connector = JsonHttpConnector::new(datasource, &core.handle()).unwrap();
//! #
//! #    fn create_doc<C>(connector: C, core: &mut Core) -> Result<(), Error>
//! #        where C: 'static + Connector
//! #    {
//! #
//! #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u16,
//! }
//!
//! // create an instance of our custom struct
//! let person = Person { name: "herbert".to_string(), age: 42 };
//!
//! // create a new document for with our custom struct as content
//! let new_document = NewDocument::from_content(person);
//!
//! // create a method call for inserting the new document
//! let insert_document = InsertDocument::new("people", new_document);
//!
//! // create a second method call for creating a new collection
//! let create_collection = CreateCollection::with_name("people");
//!
//! // now execute all the method calls
//! let db_connection = connector.connection("friendsbook");
//! let people = core.run(db_connection.execute(create_collection))?;
//! let doc_header = core.run(db_connection.execute(insert_document))?;
//!
//! // lets fetch the whole document from the db
//! let (doc_id, _, _) = doc_header.deconstruct();
//! let get_document = GetDocument::with_id(doc_id);
//! let document: Document<Person> = core.run(db_connection.execute(get_document))?;
//! #
//! #        Ok(())
//! #     }
//! # }
//! ```
//!
//! This example code uses two variables `connector` and `core` where you may
//! wonder what they are. To learn how to instantiate the `connector` and the
//! `core` variable see the documentation of the [`rincon_connector`] crate.
//!
//! The `execute()` function of a connection returns a `Future`. Thanks to the
//! power of `Future` we can also chain the executions of the method calls by
//! composing the `Future`s like so:
//!
//! ```rust,dont_run
//! # extern crate rincon_core;
//! # extern crate rincon_connector;
//! # extern crate rincon_client;
//! # extern crate futures;
//! # #[macro_use] extern crate serde_derive;
//! # extern crate tokio_core;
//! # use rincon_core::api::connector::{Connector, Error, Execute};
//! # use rincon_core::api::datasource::DataSource;
//! # use rincon_connector::http::JsonHttpConnector;
//! # use rincon_client::collection::methods::CreateCollection;
//! # use rincon_client::document::methods::{GetDocument, InsertDocument};
//! # use rincon_client::document::types::{Document, NewDocument};
//! # use futures::future::Future;
//! # use tokio_core::reactor::Core;
//! # use std::str::FromStr;
//! #
//! # fn main() {
//! #    let datasource = DataSource::from_str("http://localhost:8529")
//! #        .expect("invalid URL for datasource")
//! #        .with_basic_authentication("root", "s3cur3");
//! #
//! #    let mut core = Core::new().unwrap();
//! #
//! #    let connector = JsonHttpConnector::new(datasource, &core.handle()).unwrap();
//! #
//! #    fn create_doc<C>(connector: C, core: &mut Core) -> Result<(), Error>
//! #        where C: 'static + Connector
//! #    {
//! #
//! # #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//! # struct Person {
//! #     name: String,
//! #     age: u16,
//! # }
//! #
//! # // create an instance of our custom struct
//! # let person = Person { name: "herbert".to_string(), age: 42 };
//! #
//! # // create a new document for with our custom struct as content
//! # let new_document = NewDocument::from_content(person);
//! #
//! let db_conn = connector.connection("friendsbook");
//!
//! // chaining the method calls
//! let document: Document<Person> = core.run(
//!     db_conn.execute(CreateCollection::with_name("people"))
//!         .and_then(|_people|
//!             db_conn.execute(InsertDocument::new("people", new_document))
//!         )
//!         .and_then(|doc_header| {
//!             let (doc_id, _, _) = doc_header.deconstruct();
//!             db_conn.execute(GetDocument::with_id(doc_id))
//!         })
//! )?;
//! #
//! #        Ok(())
//! #     }
//! # }
//! ```
//!
//! # Advanced attributes and optional crate features
//!
//! Some attributes of methods or their results are only meaningful in a certain
//! server configuration, like using RocksDB instead of MMFiles or the server is
//! setup in a cluster. Those attributes are only available in the API of this
//! crate if the crate is compiled with the related feature enabled. The
//! optional crate features are:
//!
//! * `mmfiles` : enables MMFiles storage engine related attributes,
//! * `rocksdb` : enables RocksDB storage engine related attributes,
//! * `cluster` : enables cluster configuration related attributes,
//! * `enterprise` : enables attributes supported only by the enterprise version of ArangoDB,
//!
//! It is not necessary to activate any of the optional crate features if an
//! application does not need to access the feature related attributes.
//!
//! [ArangoDB]: https://www.arangodb.com
//! [`rincon_connector`]: https://docs.rs/rincon_connector

#![doc(html_root_url = "https://docs.rs/rincon_client/0.1.1")]

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

extern crate serde;
#[macro_use] extern crate serde_derive;
#[cfg(not(test))] extern crate serde_json;
#[cfg(test)] #[macro_use] extern crate serde_json;

extern crate rincon_core;

#[allow(missing_docs)] pub mod admin;
#[allow(missing_docs)] pub mod aql;
pub mod auth;
pub mod collection;
pub mod cursor;
pub mod database;
#[allow(missing_docs)] pub mod document;
#[allow(missing_docs)] pub mod graph;
#[allow(missing_docs)] pub mod index;
pub mod user;

pub mod client {
    //! Re-export of all public types of the client API.
    //!
    //! If your application needs a lot of types from the client API you may
    //! import them from this client module instead of digging through the various
    //! sub-modules.

    pub use super::admin::methods::*;
    pub use super::admin::types::*;
    pub use super::aql::methods::*;
    pub use super::aql::types::*;
    pub use super::auth::methods::*;
    pub use super::auth::types::*;
    pub use super::collection::methods::*;
    pub use super::collection::types::*;
    pub use super::cursor::methods::*;
    pub use super::cursor::types::*;
    pub use super::database::methods::*;
    pub use super::database::types::*;
    pub use super::document::methods::*;
    pub use super::document::types::*;
    pub use super::graph::methods::*;
    pub use super::graph::types::*;
    pub use super::index::methods::*;
    pub use super::index::types::*;
    pub use super::user::methods::*;
    pub use super::user::types::*;
}
