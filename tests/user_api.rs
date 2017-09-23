
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::{Empty, EMPTY};
use arangodb_client::user::*;

#[test]
fn list_available_users_should_return_1_user() {
    let (mut core, conn) = init_db_test();

    let method = ListAvailableUsers::<Empty>::new();
    let work = conn.execute(method);
    let available_users = core.run(work).unwrap();

    assert_eq!(1, available_users.len());
}

#[test]
fn list_available_users_should_return_the_root_user() {
    let (mut core, conn) = init_db_test();

    let method = ListAvailableUsers::new();
    let work = conn.execute(method);
    let available_users = core.run(work).unwrap();

    let user1 = &available_users[0];
    assert_eq!("root", user1.name());
    assert!(user1.is_active());
    assert_eq!(&Empty{}, user1.extra())
}

#[test]
fn get_user_should_return_active_root_user() {
    let (mut core, conn) = init_db_test();

    let method = GetUser::with_name("root");
    let work = conn.execute(method);
    let user = core.run(work).unwrap();

    assert_eq!("root", user.name());;
    assert!(user.is_active());
    assert_eq!(&EMPTY, user.extra())
}

#[test]
fn create_user_with_name_should_return_newly_created_user_as_active() {
    let (mut core, conn) = init_db_test();

    let new_user = NewUser::with_name("testuser1", "testpw1");
    let method = CreateUser::new(new_user);
    let work = conn.execute(method);
    let user = core.run(work).unwrap();

    assert_eq!("testuser1", user.name());
    assert!(user.is_active());
    assert_eq!(&EMPTY, user.extra());

    let method = RemoveUser::with_name("testuser1");
    let work = conn.execute(method);
    core.run(work).unwrap();
}
