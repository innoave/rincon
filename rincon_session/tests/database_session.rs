
#[macro_use] extern crate hamcrest;

extern crate tokio_core;

extern crate rincon_client;
extern crate rincon_connector;
extern crate rincon_session;
extern crate rincon_test_helper;

use hamcrest::prelude::*;

use tokio_core::reactor::Core;

use rincon_connector::connection::Connection;
use rincon_session::ArangoSession;

use rincon_test_helper::*;

#[test]
fn create_graph() {
    let mut core = Core::new().unwrap();

    let datasource = system_datasource();
    let connection = Connection::establish(&MyUserAgent, datasource, &mut core.handle());

    let arango = ArangoSession::new(connection);
    let database = arango.use_database("the_social_network");

    let graph = database.create_graph("social",
        vec!["male".to_owned(), "female".to_owned()],
        vec!["male".to_owned(), "female".to_owned()]
    );

    assert_that!(graph.name(), is(equal_to("social")));
}
