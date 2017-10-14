
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::method::ErrorCode;
use arangodb_client::collection::CreateCollection;
use arangodb_client::connection::Error;
use arangodb_client::index::*;

#[test]
fn get_index_list_from_collection() {
    arango_user_db_test("test_index_user1", "test_index_db11", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = GetIndexList::of_collection("customers");
        let result = core.run(conn.execute(method)).unwrap();

        let indexes = result.indexes();
        let identifiers = result.identifiers();
        {
            let index1 = indexes.get(0).unwrap();
            assert_eq!("customers/0", index1.id());
            assert_eq!(&vec!["_key".to_owned()][..], index1.fields());
            assert_eq!(false, index1.is_newly_created());
            if let &Index::Primary(ref primary_index) = index1 {
                assert_eq!(1, primary_index.selectivity_estimate());
                assert_eq!(true, primary_index.is_unique());
            } else {
                panic!("PrimaryIndex expected, but got {:?}", index1);
            }
        }
        {
            let identifier1 = identifiers.get("customers/0").unwrap();
            assert_eq!("customers/0", identifier1.id());
            assert_eq!(&vec!["_key".to_owned()][..], identifier1.fields());
            assert_eq!(false, identifier1.is_newly_created());
            if let &Index::Primary(ref primary_index) = identifier1 {
                assert_eq!(1, primary_index.selectivity_estimate());
                assert_eq!(true, primary_index.is_unique());
            } else {
                panic!("PrimaryIndex expected, but got {:?}", identifier1);
            }
        }

        assert_eq!(1, indexes.len());
        assert_eq!(1, identifiers.len());
    });
}

#[test]
fn get_index_from_collection() {
    arango_user_db_test("test_index_user2", "test_index_db21", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = GetIndex::new("customers", "0");
        let index = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers/0", index.id());
        assert_eq!(&vec!["_key".to_owned()][..], index.fields());
        assert_eq!(false, index.is_newly_created());
        if let Index::Primary(ref primary_index) = index {
            assert_eq!(1, primary_index.selectivity_estimate());
            assert_eq!(true, primary_index.is_unique());
        } else {
            panic!("PrimaryIndex expected, but got {:?}", index);
        }
    });
}
