
#![doc(html_root_url = "https://docs.rs/rincon_session/0.1.0")]

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
extern crate serde;
extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_client;

mod arango_session;
mod collection_session;
mod cursor_session;
mod database_session;
mod graph_session;

pub use self::arango_session::*;
pub use self::collection_session::*;
pub use self::cursor_session::*;
pub use self::database_session::*;
pub use self::graph_session::*;

pub use rincon_client::admin::types::{ServerVersion, TargetVersion};
pub use rincon_client::aql::types::{ExplainedQuery, ExplainOptions, ParsedQuery};
pub use rincon_client::collection::types::{Collection, CollectionProperties,
    CollectionPropertiesUpdate, CollectionRevision, NewCollection, RenameTo};
pub use rincon_client::document::types::{Document, DocumentHeader, DocumentId,
    DocumentKey, DocumentModifyOptions, DocumentReplaceOptions, DocumentUpdate,
    NewDocument, UpdatedDocument};
pub use rincon_client::cursor::types::{Cursor, CursorStatistics, NewCursor,
    Warning};
pub use rincon_client::database::types::{Database, NewDatabase};
pub use rincon_client::graph::types::{EdgeDefinition, Graph, NewGraph};
pub use rincon_client::user::types::{NewUser, Permission, User, UserExtra,
    UserUpdate};
pub use rincon_core::api::connector::Error;
pub use rincon_core::api::method::ResultList;
pub use rincon_core::api::query::Query;
pub use rincon_core::api::types::{Empty, EMPTY, Entity};

pub type Result<T> = ::std::result::Result<T, Error>;
