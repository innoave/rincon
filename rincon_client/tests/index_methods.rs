
#![cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]

extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_connector;
extern crate rincon_client;
extern crate rincon_test_helper;

use rincon_core::api::ErrorCode;
use rincon_core::api::connector::{Error, Execute};
use rincon_client::index::methods::*;
use rincon_client::index::types::*;

use rincon_test_helper::*;


#[test]
fn get_index_list_from_document_collection() {
    arango_test_with_document_collection("index_customers01", |conn, ref mut core| {

        let method = GetIndexList::of_collection("index_customers01");
        let result = core.run(conn.execute(method)).unwrap();

        let indexes = result.indexes();
        let identifiers = result.identifiers();
        {
            let index1 = &indexes[0];
            let index_id = match *index1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("index_customers01/0", &index_id.to_string());
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
            let identifier1 = identifiers.get("index_customers01/0").unwrap();
            let index_id = match *identifier1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("index_customers01/0", index_id.to_string());
            assert_eq!(&vec!["_key".to_owned()][..], identifier1.fields());
            assert_eq!(false, identifier1.is_newly_created());
            if let Index::Primary(ref primary_index) = *identifier1 {
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
    arango_test_with_edge_collection("index_customers02", |conn, ref mut core| {

        let method = GetIndexList::of_collection("index_customers02");
        let result = core.run(conn.execute(method)).unwrap();

        let indexes = result.indexes();
        let identifiers = result.identifiers();
        {
            let index1 = &indexes[0];
            let index_id = match *index1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("index_customers02/0", &index_id.to_string());
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
            let index2 = &indexes[1];
            let index_id = match *index2.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("index_customers02/1", &index_id.to_string());
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
            let identifier1 = identifiers.get("index_customers02/0").unwrap();
            let index_id = match *identifier1.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("index_customers02/0", index_id.to_string());
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
            let identifier2 = identifiers.get("index_customers02/1").unwrap();
            let index_id = match *identifier2.id() {
                IndexIdOption::Qualified(ref index_id) => index_id,
                _ => panic!("Qualified index id expected!"),
            };
            assert_eq!("index_customers02/1", index_id.to_string());
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
    arango_test_with_document_collection("index_customers03", |conn, ref mut core| {

        let method = GetIndex::new(IndexId::new("index_customers03", "0"));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers03/0", index_id.to_string());
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
    arango_test_with_document_collection("index_customers04", |conn, ref mut core| {

        let method = CreateIndex::new("index_customers04", NewHashIndex::new(
            vec!["name".to_owned()], true, false, true));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers04", index_id.collection_name());
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
    arango_test_with_document_collection("index_customers05", |conn, ref mut core| {

        let method = CreateIndex::new("index_customers05", NewSkipListIndex::new(
            vec!["age".to_owned(), "gender".to_owned()], false, true, false));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers05", index_id.collection_name());
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
    arango_test_with_document_collection("index_customers06", |conn, ref mut core| {

        let method = CreateIndex::new("index_customers06", NewPersistentIndex::new(
            vec!["age".to_owned(), "gender".to_owned()], false, true));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers06", index_id.collection_name());
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
    arango_test_with_document_collection("index_customers07", |conn, ref mut core| {

        let method = CreateIndex::new("index_customers07",
            NewGeoIndex::with_location_field("location", true));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers07", index_id.collection_name());
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
    arango_test_with_document_collection("index_customers08", |conn, ref mut core| {

        let method = CreateIndex::new("index_customers08",
            NewGeoIndex::with_lat_lng_fields("latitude", "longitude"));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers08", index_id.collection_name());
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
    arango_test_with_document_collection("index_customers09", |conn, ref mut core| {

        let method = CreateIndex::new("index_customers09", NewFulltextIndex::new(
            "description", 4));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers09", index_id.collection_name());
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
    arango_test_with_document_collection("index_customers10", |conn, ref mut core| {

        let method = CreateIndex::new("index_customers10", NewFulltextIndex::new(
            "description", 4));
        let index = core.run(conn.execute(method)).unwrap();
        let index_id = match *index.id() {
            IndexIdOption::Qualified(ref index_id) => index_id,
            _ => panic!("Qualified index id expected!"),
        };

        assert_eq!("index_customers10", index_id.collection_name());
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

        let method = CreateIndex::new("index_customers10", NewFulltextIndex::new(
            "description", 4));
        let index2 = core.run(conn.execute(method)).unwrap();

        assert_eq!(index.id(), index2.id());
    });
}

#[test]
fn delete_index_of_type_hash_for_collection() {
    arango_test_with_document_collection("index_customers11", |conn, ref mut core| {

        let create = CreateIndex::new("index_customers11", NewHashIndex::new(
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
    arango_test_with_document_collection("index_customers12", |conn, ref mut core| {

        let create = CreateIndex::new("index_customers12", NewHashIndex::new(
            vec!["name".to_owned()], true, false, true));
        core.run(conn.execute(create)).unwrap();

        let delete = DeleteIndex::new(IndexId::new("index_customers12", "9999999"));
        let result = core.run(conn.execute(delete));

        match result {
            Err(Error::Method(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoIndexNotFound, error.error_code());
                assert_eq!("index not found", error.message());
            },
            _ => panic!("Expected error but got {:?}", result),
        }
    });
}
