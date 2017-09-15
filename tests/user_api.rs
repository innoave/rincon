
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::user::*;

#[test]
fn list_available_users() {
    let (mut core, conn) = init_db_test();

    let method = ListAvailableUsers::<EmptyUserInfo>::new();
    let work = conn.execute(method);
    let result = core.run(work).unwrap();

    assert_eq!(false, result.is_error());
    assert_eq!(200, result.code());

    let available_users = result.result();

    assert_eq!(1, available_users.len());
}
