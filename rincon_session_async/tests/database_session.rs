
#[macro_use] extern crate hamcrest;

extern crate tokio_core;

extern crate rincon_client;
extern crate rincon_connector;
extern crate rincon_session_async;
extern crate rincon_test_helper;

use hamcrest::prelude::*;

use tokio_core::reactor::Core;

use rincon_connector::http::Connection;
use rincon_session_async::*;

use rincon_test_helper::*;


#[test]
fn create_graph() {
    let mut core = Core::new().unwrap();

    let datasource = system_datasource();
    let connection = Connection::establish(&MyUserAgent, datasource, &mut core.handle()).unwrap();

    let arango = ArangoSession::new(connection);
    let database = arango.use_database("the_social_network");

    let graph_session = core.run(database.create_graph(NewGraph::with_name("social")
        .with_edge_definitions(vec![
            EdgeDefinition::new("person", vec!["male".to_owned()], vec!["female".to_owned()]),
            EdgeDefinition::new("friend", vec!["male".to_owned()], vec!["female".to_owned()]),
        ]))).unwrap();

    assert_that!(graph_session.graph().name(), is(equal_to("social")));
}
