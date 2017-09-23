
extern crate dotenv;
extern crate futures;
extern crate log4rs;
#[macro_use] extern crate serde_derive;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::{Empty, EMPTY};
use arangodb_client::database::*;
use arangodb_client::user::*;

#[test]
fn list_available_users_should_return_the_root_user() {
    let (mut core, conn) = init_db_test();

    let method: ListAvailableUsers<Empty> = ListAvailableUsers::new();
    let work = conn.execute(method);
    let available_users = core.run(work).unwrap();

    assert!(available_users.iter().any(|user| user.name() == "root"));
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

    let user_name = String::from("testuser1");

    let new_user = NewUser::with_name(user_name.as_ref(), "testpass1");
    let method = CreateUser::new(new_user);
    let work = conn.execute(method);
    let user = core.run(work).unwrap();

    assert_eq!(&user_name, user.name());
    assert!(user.is_active());
    assert_eq!(&EMPTY, user.extra());

    let method = RemoveUser::with_name(user_name);
    let work = conn.execute(method);
    core.run(work).unwrap();
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CustomExtra {
    email: String,
    age: u16,
}

impl UserExtra for CustomExtra {}

#[test]
fn create_user_with_extra_should_return_newly_created_user_with_extra() {
    let (mut core, conn) = init_db_test();

    let user_extra = CustomExtra {
        email: "testuser2@mail.rs".to_owned(),
        age: 27,
    };
    let mut new_user = NewUser::with_name("testuser2", "testpw2");
    new_user.set_extra(Some(user_extra));
    let method = CreateUser::new(new_user);
    let work = conn.execute(method);
    let user = core.run(work).unwrap();

    assert_eq!("testuser2", user.name());
    assert!(user.is_active());
    assert_eq!(&CustomExtra { email: "testuser2@mail.rs".to_owned(), age: 27 }, user.extra());

    let method = RemoveUser::with_name("testuser2");
    let work = conn.execute(method);
    core.run(work).unwrap();
}

#[test]
fn list_databases_for_user_testuser3() {
    let (mut core, conn) = init_db_test();

    let new_user: NewUser<Empty> = NewUser::with_name("testuser3", "");

    let new_database1 = NewDatabase::new("testbase31".to_owned(), vec![new_user.clone()]);
    let _ = core.run(conn.execute(CreateDatabase::new(new_database1))).unwrap();
    let new_database2 = NewDatabase::new("testbase32".to_owned(), vec![new_user.clone()]);
    let _ = core.run(conn.execute(CreateDatabase::new(new_database2))).unwrap();

    let method = ListDatabasesForUser::for_user("testuser3");
    let work = conn.execute(method);
    let databases = core.run(work).unwrap();

    assert!(databases.contains_key("testbase31"));
    assert_eq!(&Permission::ReadWrite, databases.get("testbase31").unwrap());
    assert!(databases.contains_key("testbase32"));
    assert_eq!(&Permission::ReadWrite, databases.get("testbase32").unwrap());

    let _ = core.run(conn.execute(DropDatabase::with_name("testbase32"))).unwrap();
    let _ = core.run(conn.execute(DropDatabase::with_name("testbase31"))).unwrap();
}
