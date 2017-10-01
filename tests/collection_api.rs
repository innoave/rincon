
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api;
use arangodb_client::collection::*;
use arangodb_client::connection::Error;

#[test]
fn create_collection_with_default_properties() {
    arango_user_db_test("test_coll_user1", "test_coll_db11", |conn, ref mut core| {

        let method = CreateCollection::with_name("test_collection1");
        let work = conn.execute(method);
        let collection = core.run(work).unwrap();

        assert_eq!("test_collection1", collection.name());
        assert_eq!(CollectionType::Documents, collection.kind());
        assert_eq!(CollectionStatus::Loaded, collection.status());
        assert!(!collection.is_system());
        assert!(!collection.is_wait_for_sync());

        #[cfg(feature = "mmfiles")]
        assert!(!collection.is_volatile());
    });
}

#[test]
fn create_edge_collection_with_wait_for_sync() {
    arango_user_db_test("test_coll_user2", "test_coll_db21", |conn, ref mut core| {

        let mut new_collection = NewCollection::edges_with_name("test_collection1");
        new_collection.set_wait_for_sync(Some(true));

        let method = CreateCollection::new(new_collection);
        let work = conn.execute(method);
        let collection = core.run(work).unwrap();

        assert_eq!("test_collection1", collection.name());
        assert_eq!(CollectionType::Edges, collection.kind());
        assert_eq!(CollectionStatus::Loaded, collection.status());
        assert!(!collection.is_system());
        assert!(collection.is_wait_for_sync());

        #[cfg(feature = "mmfiles")]
        assert!(!collection.is_volatile());
    });
}

#[test]
fn drop_collection_should_return_the_id_of_the_dropped_collection() {
    arango_user_db_test("test_coll_user3", "test_coll_db31", |conn, ref mut core| {

        let collection1 = core.run(conn.execute(CreateCollection::with_name("test_collection1"))).unwrap();
        let _ = core.run(conn.execute(CreateCollection::with_name("test_collection2"))).unwrap();

        let method = DropCollection::with_name("test_collection1");
        let work = conn.execute(method);
        let coll1_id = core.run(work).unwrap();

        assert_eq!(collection1.id(), coll1_id);
    });
}

#[test]
fn list_collections_should_return_two_collections() {
    arango_user_db_test("test_coll_user4", "test_coll_db41", |conn, ref mut core| {

        let _ = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();
        let _ = core.run(conn.execute(CreateCollection::edges_with_name("test_collection2"))).unwrap();

        let method = ListCollections::new();
        let work = conn.execute(method);
        let collections = core.run(work).unwrap();

        let collection1 = collections.iter().find(|coll|
            coll.name() == "test_collection1").unwrap();

        assert_eq!("test_collection1", collection1.name());
        assert_eq!(CollectionType::Documents, collection1.kind());
        assert_eq!(CollectionStatus::Loaded, collection1.status());
        assert!(!collection1.is_system());

        let collection2 = collections.iter().find(|coll|
            coll.name() == "test_collection2").unwrap();

        assert_eq!("test_collection2", collection2.name());
        assert_eq!(CollectionType::Edges, collection2.kind());
        assert_eq!(CollectionStatus::Loaded, collection2.status());
        assert!(!collection2.is_system());
    });
}

#[test]
fn list_collections_should_return_empty_list() {
    arango_user_db_test("test_coll_user5", "test_coll_db51", |conn, ref mut core| {

        let method = ListCollections::new();
        let work = conn.execute(method);
        let collections = core.run(work).unwrap();

        assert!(collections.is_empty());
    });
}

#[test]
fn get_collection_should_return_collection_info() {
    arango_user_db_test("test_coll_user6", "test_coll_db61", |conn, ref mut core| {

        let _ = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();
        let _ = core.run(conn.execute(CreateCollection::edges_with_name("test_collection2"))).unwrap();

        let method = GetCollection::with_name("test_collection2");
        let work = conn.execute(method);
        let collection = core.run(work).unwrap();

        assert_eq!("test_collection2", collection.name());
        assert_eq!(CollectionType::Edges, collection.kind());
        assert_eq!(CollectionStatus::Loaded, collection.status());
        assert!(!collection.is_system());
    });
}

#[test]
fn get_collection_should_return_an_error_if_collection_not_found() {
    arango_user_db_test("test_coll_user7", "test_coll_db71", |conn, ref mut core| {

        let _ = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();
        let _ = core.run(conn.execute(CreateCollection::edges_with_name("test_collection2"))).unwrap();

        let method = GetCollection::with_name("test_collection_not_existing");
        let work = conn.execute(method);
        let result = core.run(work);

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(api::ErrorCode::ArangoCollectionNotFound, error.error_code());
                assert_eq!("unknown collection 'test_collection_not_existing'", error.message());
            },
            _ => panic!("Error::ApiError expected but got {:?}", result),
        };
    });
}

#[test]
fn get_collection_properties_should_return_collection_properties() {
    arango_user_db_test("test_coll_user8", "test_coll_db81", |conn, ref mut core| {

        let _ = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();
        let _ = core.run(conn.execute(CreateCollection::edges_with_name("test_collection2"))).unwrap();

        let method = GetCollectionProperties::with_name("test_collection1");
        let work = conn.execute(method);
        let collection = core.run(work).unwrap();

        assert_eq!("test_collection1", collection.name());
        assert_eq!(CollectionType::Documents, collection.kind());
        assert_eq!(CollectionStatus::Loaded, collection.status());
        assert!(!collection.is_system());
        assert!(collection.key_options().is_allow_user_keys());
        assert_eq!(KeyGeneratorType::Traditional, collection.key_options().kind());
        assert_eq!(0, collection.key_options().last_value());
        assert!(!collection.is_wait_for_sync());

        #[cfg(feature = "mmfiles")]
        assert!(!collection.is_volatile());
        #[cfg(feature = "mmfiles")]
        assert!(collection.is_do_compact());
        #[cfg(feature = "mmfiles")]
        assert_eq!(8, collection.index_buckets());
    });
}

#[test]
fn get_collection_properties_should_return_an_error_if_collection_not_found() {
    arango_user_db_test("test_coll_user9", "test_coll_db91", |conn, ref mut core| {

        let _ = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();
        let _ = core.run(conn.execute(CreateCollection::edges_with_name("test_collection2"))).unwrap();

        let method = GetCollectionProperties::with_name("test_collection_not_existing");
        let work = conn.execute(method);
        let result = core.run(work);

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(api::ErrorCode::ArangoCollectionNotFound, error.error_code());
                assert_eq!("unknown collection 'test_collection_not_existing'", error.message());
            },
            _ => panic!("Error::ApiError expected but got {:?}", result),
        };
    });
}

#[test]
fn change_collection_properties_wait_for_sync() {
    arango_user_db_test("test_coll_user10", "test_coll_db101", |conn, ref mut core| {

        let _ = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();
        let original = core.run(conn.execute(GetCollectionProperties::with_name("test_collection1"))).unwrap();

        assert_eq!("test_collection1", original.name());
        assert!(!original.is_wait_for_sync());
        #[cfg(feature = "mmfiles")]
        assert_eq!(32 * 1024 * 1024, original.journal_size());

        let mut updates = CollectionPropertiesUpdate::new();
        updates.set_wait_for_sync(Some(true));
        let method = ChangeCollectionProperties::new("test_collection1".into(), updates);
        let work = conn.execute(method);
        let updated = core.run(work).unwrap();

        assert_eq!("test_collection1", updated.name());
        assert!(updated.is_wait_for_sync());
        #[cfg(feature = "mmfiles")]
        assert_eq!(32 * 1024 * 1024, updated.journal_size());
    });
}

#[cfg(feature = "mmfiles")]
#[test]
fn change_collection_properties_journal_size() {
    arango_user_db_test("test_coll_user11", "test_coll_db111", |conn, ref mut core| {

        let _ = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();
        let original = core.run(conn.execute(GetCollectionProperties::with_name("test_collection1"))).unwrap();

        assert_eq!("test_collection1", original.name());
        assert!(!original.is_wait_for_sync());
        assert_eq!(32 * 1024 * 1024, original.journal_size());

        let mut updates = CollectionPropertiesUpdate::new();
        updates.set_journal_size(Some(128 * 1024 * 1024));
        let method = ChangeCollectionProperties::new("test_collection1".into(), updates);
        let work = conn.execute(method);
        let updated = core.run(work).unwrap();

        assert_eq!("test_collection1", updated.name());
        assert!(!updated.is_wait_for_sync());
        assert_eq!(128 * 1024 * 1024, updated.journal_size());
    });
}

#[test]
fn rename_collection_to_new_name() {
    arango_user_db_test("test_coll_user12", "test_coll_db121", |conn, ref mut core| {

        let original = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();

        assert_eq!("test_collection1", original.name());

        let method = RenameCollection::with_name("test_collection1")
            .to_name("test_collection_renamed");
        let work = conn.execute(method);
        let updated = core.run(work).unwrap();

        assert_eq!("test_collection_renamed", updated.name());
    });
}

#[test]
fn rename_collection_to_empty_name() {
    arango_user_db_test("test_coll_user13", "test_coll_db131", |conn, ref mut core| {

        let original = core.run(conn.execute(CreateCollection::documents_with_name("test_collection1"))).unwrap();

        assert_eq!("test_collection1", original.name());

        let method = RenameCollection::with_name("test_collection1")
            .to_name("");
        let work = conn.execute(method);
        let result = core.run(work);

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(403, error.status_code());
                assert_eq!(api::ErrorCode::Forbidden, error.error_code());
                assert_eq!("forbidden", error.message());
            },
            _ => panic!("Error::ApiError expected but got {:?}", result),
        }
    });
}
