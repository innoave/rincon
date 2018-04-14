
#[macro_use] extern crate galvanic_assert;

extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_client;
extern crate rincon_connector;
extern crate rincon_session;
extern crate rincon_test_helper;

use galvanic_assert::matchers::*;

use rincon_core::api::types::Entity;
use rincon_session::*;
use rincon_session::client::*;

use rincon_test_helper::*;

#[test]
fn create_collection() {
    arango_session_test_with_user_db("socius10", "the_social_network10", |connector, core| {

        let arango = ArangoSession::new(connector, core);
        let database = arango.use_database_with_name("the_social_network10");

        let collection_session = database.create_collection("people").unwrap();

        if let Entity::Object(collection) = collection_session.unwrap() {

            expect_that!(&collection.name(), eq("people"));
            expect_that!(&collection.kind(), eq(CollectionType::Documents));
            expect_that!(&collection.is_system(), eq(false));
            expect_that!(&collection.status(), eq(CollectionStatus::Loaded));

        } else {
            panic!("DatabaseSession::create_collection did not return Entity::Object");
        }
    });
}

// this test also tests whether the propagation of crate features is working
#[test]
fn get_collection_properties() {
    arango_session_test_with_user_db("socius11", "the_social_network11", |connector, core| {

        let arango = ArangoSession::new(connector, core);
        let database = arango.use_database_with_name("the_social_network11");

        let collection_session = database.create_collection("people").unwrap();
        let properties = collection_session.get_properties().unwrap();

        expect_that!(&properties.name(), eq("people"));
        expect_that!(&properties.kind(), eq(CollectionType::Documents));
        expect_that!(&properties.is_system(), eq(false));
        expect_that!(&properties.status(), eq(CollectionStatus::Loaded));
        expect_that!(&properties.is_wait_for_sync(), eq(false));

        #[cfg(feature = "mmfiles")] {
            expect_that!(&properties.is_volatile(), eq(false));
            expect_that!(&properties.is_do_compact(), eq(true));
            expect_that!(&properties.index_buckets(), gt(0));
            expect_that!(&properties.journal_size(), gt(0));
        }
    });
}

#[test]
fn create_graph() {
    arango_session_test_with_user_db("socius20", "the_social_network20", |connector, core| {

        let arango = ArangoSession::new(connector, core);
        let database = arango.use_database_with_name("the_social_network20");

        let graph_session = database.create_graph(NewGraph::with_name("social")
            .with_edge_definitions(vec![
                EdgeDefinition::new("person", vec!["male".to_owned()], vec!["female".to_owned()]),
                EdgeDefinition::new("friend", vec!["male".to_owned()], vec!["female".to_owned()]),
            ])).unwrap();

        assert_that!(&graph_session.name(), eq("social"));
    });
}
