
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::collection::*;

#[test]
fn create_collection_with_default_attributes() {
    arango_user_db_test("testcolluser1", "testcolldb11", |conn, ref mut core| {

        let method = CreateCollection::with_name("testcollection111");
        let work = conn.execute(method);
        let collection = core.run(work).unwrap();

        assert_eq!("testcollection111", collection.name());
        assert_eq!(&CollectionType::Documents, collection.kind());
        assert_eq!(&CollectionStatus::Loaded, collection.status());
        assert!(!collection.is_system());
        assert!(!collection.is_volatile());
        assert!(!collection.is_wait_for_sync());
    });
}
