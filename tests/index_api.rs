
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
fn get_index_list_from_document_collection() {
    arango_user_db_test("test_index_user1", "test_index_db11", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = GetIndexList::of_collection("customers");
        let result = core.run(conn.execute(method)).unwrap();

        let indexes = result.indexes();
        let identifiers = result.identifiers();
        {
            let index1 = indexes.get(0).unwrap();
            let index_id = match *index1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("customers/0", &index_id.to_string());
            assert_eq!(&vec!["_key".to_owned()][..], index1.fields());
            assert_eq!(false, index1.is_newly_created());
            if let &Index::Primary(ref primary_index) = index1 {
                assert_eq!(true, primary_index.is_unique());
                assert_eq!(1, primary_index.selectivity_estimate());
            } else {
                panic!("PrimaryIndex expected, but got {:?}", index1);
            }
        }
        {
            let identifier1 = identifiers.get("customers/0").unwrap();
            let index_id = match *identifier1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("customers/0", index_id.to_string());
            assert_eq!(&vec!["_key".to_owned()][..], identifier1.fields());
            assert_eq!(false, identifier1.is_newly_created());
            if let &Index::Primary(ref primary_index) = identifier1 {
                assert_eq!(true, primary_index.is_unique());
                assert_eq!(1, primary_index.selectivity_estimate());
            } else {
                panic!("PrimaryIndex expected, but got {:?}", identifier1);
            }
        }

        assert_eq!(1, indexes.len());
        assert_eq!(1, identifiers.len());
    });
}

#[test]
fn get_index_list_from_edge_collection() {
    arango_user_db_test("test_index_user10", "test_index_db101", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::edges_with_name("customers"))).unwrap();

        let method = GetIndexList::of_collection("customers");
        let result = core.run(conn.execute(method)).unwrap();

        let indexes = result.indexes();
        let identifiers = result.identifiers();
        {
            let index1 = indexes.get(0).unwrap();
            let index_id = match *index1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("customers/0", &index_id.to_string());
            assert_eq!(&vec!["_key".to_owned()][..], index1.fields());
            assert_eq!(false, index1.is_newly_created());
            if let &Index::Primary(ref primary_index) = index1 {
                assert_eq!(true, primary_index.is_unique());
                assert_eq!(false, primary_index.is_sparse());
                assert_eq!(1, primary_index.selectivity_estimate());
            } else {
                panic!("PrimaryIndex expected, but got {:?}", index1);
            }
        }
        {
            let index2 = indexes.get(1).unwrap();
            let index_id = match *index2.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("customers/1", &index_id.to_string());
            assert_eq!(&vec!["_from".to_owned(), "_to".to_owned()][..], index2.fields());
            assert_eq!(false, index2.is_newly_created());
            if let &Index::Edge(ref edge_index) = index2 {
                assert_eq!(false, edge_index.is_unique());
                assert_eq!(false, edge_index.is_sparse());
            } else {
                panic!("EdgeIndex expected, but got {:?}", index2);
            }
        }
        {
            let identifier1 = identifiers.get("customers/0").unwrap();
            let index_id = match *identifier1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("customers/0", index_id.to_string());
            assert_eq!(&vec!["_key".to_owned()][..], identifier1.fields());
            assert_eq!(false, identifier1.is_newly_created());
            if let &Index::Primary(ref primary_index) = identifier1 {
                assert_eq!(true, primary_index.is_unique());
                assert_eq!(false, primary_index.is_sparse());
                assert_eq!(1, primary_index.selectivity_estimate());
            } else {
                panic!("PrimaryIndex expected, but got {:?}", identifier1);
            }
        }
        {
            let identifier2 = identifiers.get("customers/1").unwrap();
            let index_id = match *identifier2.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("customers/1", index_id.to_string());
            assert_eq!(&vec!["_from".to_owned(), "_to".to_owned()][..], identifier2.fields());
            assert_eq!(false, identifier2.is_newly_created());
            if let &Index::Edge(ref edge_index) = identifier2 {
                assert_eq!(false, edge_index.is_unique());
                assert_eq!(false, edge_index.is_sparse());
            } else {
                panic!("EdgeIndex expected, but got {:?}", identifier2);
            }
        }

        assert_eq!(2, indexes.len());
        assert_eq!(2, identifiers.len());
    });
}

#[test]
fn get_index_from_collection() {
    arango_user_db_test("test_index_user2", "test_index_db21", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = GetIndex::new(IndexId::new("customers", "0"));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers/0", index_id.to_string());
        assert_eq!(&vec!["_key".to_owned()][..], index.fields());
        assert_eq!(false, index.is_newly_created());
        if let Index::Primary(ref primary_index) = index {
            assert_eq!(true, primary_index.is_unique());
            assert_eq!(1, primary_index.selectivity_estimate());
        } else {
            panic!("PrimaryIndex expected, but got {:?}", index);
        }
    });
}

#[test]
fn create_index_of_type_hash_for_collection() {
    arango_user_db_test("test_index_user3", "test_index_db31", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = CreateIndex::new("customers", NewHashIndex::new(
            vec!["name".to_owned()], true, false, true));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers", index_id.collection_name());
        assert!(!index_id.index_key().is_empty());
        assert_eq!(&vec!["name".to_owned()][..], index.fields());
        assert_eq!(true, index.is_newly_created());
        if let Index::Hash(ref hash_index) = index {
            assert_eq!(true, hash_index.is_unique());
            assert_eq!(false, hash_index.is_sparse());
            assert_eq!(true, hash_index.is_deduplicate());
            assert_eq!(1, hash_index.selectivity_estimate());
        } else {
            panic!("HashIndex expected, but got {:?}", index);
        }
    });
}

#[test]
fn create_index_of_type_skip_list_for_collection() {
    arango_user_db_test("test_index_user4", "test_index_db41", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = CreateIndex::new("customers", NewSkipListIndex::new(
            vec!["age".to_owned(), "gender".to_owned()], false, true, false));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers", index_id.collection_name());
        assert!(!index_id.index_key().is_empty());
        assert_eq!(&vec!["age".to_owned(), "gender".to_owned()][..], index.fields());
        assert_eq!(true, index.is_newly_created());
        if let Index::SkipList(ref skip_list_index) = index {
            assert_eq!(false, skip_list_index.is_unique());
            assert_eq!(true, skip_list_index.is_sparse());
            assert_eq!(false, skip_list_index.is_deduplicate());
        } else {
            panic!("SkipListIndex expected, but got {:?}", index);
        }
    });
}

#[test]
fn create_index_of_type_persistent_for_collection() {
    arango_user_db_test("test_index_user5", "test_index_db51", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = CreateIndex::new("customers", NewPersistentIndex::new(
            vec!["age".to_owned(), "gender".to_owned()], false, true));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers", index_id.collection_name());
        assert!(!index_id.index_key().is_empty());
        assert_eq!(&vec!["age".to_owned(), "gender".to_owned()][..], index.fields());
        assert_eq!(true, index.is_newly_created());
        if let Index::Persistent(ref persistent_index) = index {
            assert_eq!(false, persistent_index.is_unique());
            assert_eq!(true, persistent_index.is_sparse());
            assert_eq!(true, persistent_index.is_deduplicate());
        } else {
            panic!("PersistentIndex expected, but got {:?}", index);
        }
    });
}

#[test]
fn create_index_of_type_geo1_for_collection() {
    arango_user_db_test("test_index_user6", "test_index_db61", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = CreateIndex::new("customers",
            NewGeoIndex::with_location_field("location", true));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers", index_id.collection_name());
        assert!(!index_id.index_key().is_empty());
        assert_eq!(&vec!["location".to_owned()][..], index.fields());
        assert_eq!(true, index.is_newly_created());
        if let Index::Geo1(ref geo_index) = index {
            assert_eq!(false, geo_index.is_unique());
            assert_eq!(true, geo_index.is_sparse());
            assert_eq!(true, geo_index.is_geo_json());
        } else {
            panic!("Geo1Index expected, but got {:?}", index);
        }
    });
}

#[test]
fn create_index_of_type_geo2_for_collection() {
    arango_user_db_test("test_index_user7", "test_index_db71", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = CreateIndex::new("customers",
            NewGeoIndex::with_lat_lng_fields("latitude", "longitude"));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers", index_id.collection_name());
        assert!(!index_id.index_key().is_empty());
        assert_eq!(&vec!["latitude".to_owned(), "longitude".to_owned()][..], index.fields());
        assert_eq!(true, index.is_newly_created());
        if let Index::Geo2(ref geo_index) = index {
            assert_eq!(false, geo_index.is_unique());
            assert_eq!(true, geo_index.is_sparse());
        } else {
            panic!("Geo2Index expected, but got {:?}", index);
        }
    });
}

#[test]
fn create_index_of_type_fulltext_for_collection() {
    arango_user_db_test("test_index_user8", "test_index_db81", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = CreateIndex::new("customers", NewFulltextIndex::new(
            "description", 4));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers", index_id.collection_name());
        assert!(!index_id.index_key().is_empty());
        assert_eq!(&vec!["description".to_owned()][..], index.fields());
        assert_eq!(true, index.is_newly_created());
        if let Index::Fulltext(ref fulltext_index) = index {
            assert_eq!(false, fulltext_index.is_unique());
            assert_eq!(true, fulltext_index.is_sparse());
            assert_eq!(4, fulltext_index.min_length());
        } else {
            panic!("FulltextIndex expected, but got {:?}", index);
        }
    });
}

#[test]
fn create_index_that_is_already_existing() {
    arango_user_db_test("test_index_user9", "test_index_db91", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let method = CreateIndex::new("customers", NewFulltextIndex::new(
            "description", 4));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("customers", index_id.collection_name());
        assert!(!index_id.index_key().is_empty());
        assert_eq!(&vec!["description".to_owned()][..], index.fields());
        assert_eq!(true, index.is_newly_created());
        if let Index::Fulltext(ref fulltext_index) = index {
            assert_eq!(false, fulltext_index.is_unique());
            assert_eq!(true, fulltext_index.is_sparse());
            assert_eq!(4, fulltext_index.min_length());
        } else {
            panic!("FulltextIndex expected, but got {:?}", index);
        }

        let method = CreateIndex::new("customers", NewFulltextIndex::new(
            "description", 4));
        let index2 = core.run(conn.execute(method)).unwrap();

        assert_eq!(index.id(), index2.id());
    });
}

#[test]
fn delete_index_of_type_hash_for_collection() {
    arango_user_db_test("test_index_user11", "test_index_db111", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let create = CreateIndex::new("customers", NewHashIndex::new(
            vec!["name".to_owned()], true, false, true));
        let index = core.run(conn.execute(create)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        let delete = DeleteIndex::new(index_id.clone());
        let index_id = core.run(conn.execute(delete)).unwrap();

        assert_eq!(index.id(), &index_id);
    });
}

#[test]
fn delete_not_existing_index_in_existing_collection() {
    arango_user_db_test("test_index_user12", "test_index_db121", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let create = CreateIndex::new("customers", NewHashIndex::new(
            vec!["name".to_owned()], true, false, true));
        core.run(conn.execute(create)).unwrap();

        let delete = DeleteIndex::new(IndexId::new("customers", "9999999"));
        let result = core.run(conn.execute(delete));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoIndexNotFound, error.error_code());
                assert_eq!("index not found", error.message());
            },
            _ => panic!("Expected error but got {:?}", result),
        }
    });
}
