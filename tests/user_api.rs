
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::user::*;

#[test]
fn list_available_users_should_return_1_user() {
    let (mut core, conn) = init_db_test();

    let method = ListAvailableUsers::<EmptyUserInfo>::new();
    let work = conn.execute(method);
    let available_users = core.run(work).unwrap();

    assert_eq!(1, available_users.len());
}

#[test]
fn list_available_users_should_return_the_root_user() {
    let (mut core, conn) = init_db_test();

    let method = ListAvailableUsers::<EmptyUserInfo>::new();
    let work = conn.execute(method);
    let available_users = core.run(work).unwrap();

    let user1 = &available_users[0];
    assert_eq!("root", user1.name());
    assert!(user1.is_active());
    assert_eq!(&EmptyUserInfo{}, user1.extra())
}
