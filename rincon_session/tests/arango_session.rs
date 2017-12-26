
#[macro_use] extern crate hamcrest;

extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_client;
extern crate rincon_connector;
extern crate rincon_session;
extern crate rincon_test_helper;

use hamcrest::prelude::*;

use tokio_core::reactor::Core;

use rincon_test_helper::{MyUserAgent, system_datasource};
use rincon_core::api::types::Empty;
use rincon_client::database::NewDatabase;
use rincon_client::user::NewUser;
use rincon_connector::connection::Connection;
use rincon_session::async::ArangoSession;


#[test]
fn create_database() {
    let mut core = Core::new().unwrap();

    let datasource = system_datasource();
    let connection = Connection::establish(&MyUserAgent, datasource, &core.handle()).unwrap();

    let arango = ArangoSession::new(connection);

    let database = core.run(arango.create_database::<Empty>(NewDatabase::new("the_social_network",
        vec![NewUser::with_name("an_user", "a_pass")]))).unwrap();

    assert_that!(database.name(), is(equal_to("the_social_network")));
}

#[test]
fn use_database() {
    let core = Core::new().unwrap();

    let datasource = system_datasource();
    let connection = Connection::establish(&MyUserAgent, datasource, &core.handle()).unwrap();

    let arango = ArangoSession::new(connection);

    let database = arango.use_database("the_social_network");

    assert_that!(database.name(), is(equal_to("the_social_network")));
}
