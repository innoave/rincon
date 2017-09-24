
extern crate dotenv;
extern crate futures;
extern crate log4rs;
#[macro_use] extern crate serde_derive;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::{Empty, EMPTY};
use arangodb_client::collection::*;
use arangodb_client::database::{CreateDatabase, DropDatabase, NewDatabase};
use arangodb_client::user::{NewUser};
