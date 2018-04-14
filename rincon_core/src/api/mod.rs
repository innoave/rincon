//! The common API of the rincon driver.

pub mod auth;
pub mod connector;
pub mod datasource;
pub mod method;
pub mod query;
//pub mod statement;
pub mod types;
pub mod user_agent;

mod error;

pub use self::error::*;
