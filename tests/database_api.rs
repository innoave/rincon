
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::Empty;
use arangodb_client::database::*;

#[test]
fn create_database_for_default_user() {
    arango_system_db_test(|conn, ref mut core| {

        let method = CreateDatabase::<Empty>::with_name("testdatabase1");
        let work = conn.execute(method);
        let created = core.run(work).unwrap();

        assert!(created);

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("testdatabase1"))).unwrap();
    });
}
