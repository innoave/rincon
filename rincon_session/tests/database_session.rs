
#[macro_use] extern crate hamcrest;

extern crate tokio_core;

extern crate rincon_client;
extern crate rincon_connector;
extern crate rincon_session;
extern crate rincon_test_helper;

use hamcrest::prelude::*;

use rincon_session::*;

use rincon_test_helper::*;


#[test]
fn create_graph() {
    arango_session_test_with_user_db("socius10", "the_social_network10", |connector, core| {

        let arango = ArangoSession::new(connector, core);
        let database = arango.use_database_with_name("the_social_network10");

        let graph_session = database.create_graph(NewGraph::with_name("social")
            .with_edge_definitions(vec![
                EdgeDefinition::new("person", vec!["male".to_owned()], vec!["female".to_owned()]),
                EdgeDefinition::new("friend", vec!["male".to_owned()], vec!["female".to_owned()]),
            ])).unwrap();

        assert_that!(graph_session.name(), is(equal_to("social")));
    });
}
