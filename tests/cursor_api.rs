
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::method::ErrorCode;
use arangodb_client::api::query::Query;
use arangodb_client::api::types::JsonValue;
use arangodb_client::collection::CreateCollection;
use arangodb_client::cursor::*;

#[test]
fn query_returns_cursor_with_no_results() {
    arango_user_db_test("test_cursor_user1", "test_cursor_db11", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let query = Query::new("FOR c IN customers RETURN c");

        let method = CreateCursor::<JsonValue>::from_query(query);
        let work = conn.execute(method);
        let cursor = core.run(work).unwrap();

        assert_eq!(None, cursor.id());
        assert_eq!(false, cursor.has_more());
        assert!(cursor.result().is_empty());
    });
}

#[test]
fn insert_documents_and_return_their_names() {
    arango_user_db_test("test_cursor_user2", "test_cursor_db21", |conn, ref mut core| {

        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let query = Query::new(
            "FOR i IN 1..10 \
              INSERT { \
                name: CONCAT('No.', i), \
                age: i + 21 \
              } IN customers \
              RETURN NEW.name"
        );

        let method = CreateCursor::<String>::from_query(query);
        let work = conn.execute(method);
        let cursor = core.run(work).unwrap();

        assert_eq!(None, cursor.id());
        assert_eq!(false, cursor.has_more());
        assert_eq!(10, cursor.result().len());
        assert!(cursor.result().contains(&"No.1".to_owned()));
        assert!(cursor.result().contains(&"No.2".to_owned()));
        assert!(cursor.result().contains(&"No.3".to_owned()));
        assert!(cursor.result().contains(&"No.4".to_owned()));
        assert!(cursor.result().contains(&"No.5".to_owned()));
        assert!(cursor.result().contains(&"No.6".to_owned()));
        assert!(cursor.result().contains(&"No.7".to_owned()));
        assert!(cursor.result().contains(&"No.8".to_owned()));
        assert!(cursor.result().contains(&"No.9".to_owned()));
        assert!(cursor.result().contains(&"No.10".to_owned()));
        assert_eq!(10, cursor.extra().unwrap().stats().writes_executed());
    });
}
