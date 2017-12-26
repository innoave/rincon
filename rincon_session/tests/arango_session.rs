
#[macro_use] extern crate hamcrest;

extern crate tokio_core;

extern crate rincon_client;
extern crate rincon_connector;
extern crate rincon_session;
extern crate rincon_test_helper;

use hamcrest::prelude::*;

use tokio_core::reactor::Core;

use rincon_test_helper::{MyUserAgent, system_datasource};
use rincon_connector::connection::Connection;
use rincon_session::{ArangoSession};


#[test]
fn create_database() {
    let mut core = Core::new().unwrap();

    let datasource = system_datasource();
    let connection = Connection::establish(&MyUserAgent, datasource, &core.handle()).unwrap();

    let arango = ArangoSession::new(connection);

    let database = arango.create_database("the_social_network");

    assert_that!(database.name(), is(equal_to("the_social_network")));
}

#[test]
fn use_database() {
    let mut core = Core::new().unwrap();

    let datasource = system_datasource();
    let connection = Connection::establish(&MyUserAgent, datasource, &core.handle()).unwrap();

    let arango = ArangoSession::new(connection);

    let database = arango.use_database("the_social_network");

    assert_that!(database.name(), is(equal_to("the_social_network")));
}
