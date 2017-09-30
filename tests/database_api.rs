
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::Empty;
use arangodb_client::connection::Connection;
use arangodb_client::database::*;
use arangodb_client::user::{CreateUser, NewUser, RemoveUser};

#[test]
fn create_database_for_default_user() {
    arango_system_db_test(|conn, ref mut core| {

        let method = CreateDatabase::<Empty>::with_name("test_database_dd1");
        let work = conn.execute(method);
        let created = core.run(work).unwrap();

        assert!(created);

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_dd1"))).unwrap();
    });
}

#[test]
fn create_database_for_one_new_user() {
    arango_system_db_test(|conn, ref mut core| {

        let new_user = NewUser::<Empty>::with_name("test_user_d1", "");

        let method = CreateDatabase::with_name_for_users("test_database_d11",
            vec![new_user]);
        let work = conn.execute(method);
        let created = core.run(work).unwrap();

        assert!(created);

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d11"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d1"))).unwrap();
    });
}

#[test]
fn create_database_for_two_new_users() {
    arango_system_db_test(|conn, ref mut core| {

        let new_user1 = NewUser::<Empty>::with_name("test_user_d2", "");
        let new_user2 = NewUser::<Empty>::with_name("test_user_d3", "");

        let method = CreateDatabase::with_name_for_users("test_database_d22",
            vec![new_user1, new_user2]);
        let work = conn.execute(method);
        let created = core.run(work).unwrap();

        assert!(created);

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d22"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d2"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d3"))).unwrap();
    });
}

#[test]
fn create_database_for_one_existing_user() {
    arango_system_db_test(|conn, ref mut core| {

        let new_user = NewUser::<Empty>::with_name("test_user_d4", "");
        let user = core.run(conn.execute(CreateUser::new(new_user.clone()))).unwrap();

        assert_eq!("test_user_d4", user.name());

        let method = CreateDatabase::with_name_for_users("test_database_d31",
            vec![new_user]);
        let work = conn.execute(method);
        let created = core.run(work).unwrap();

        assert!(created);

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d31"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d4"))).unwrap();
    });
}

#[test]
fn create_database_for_two_existing_users_and_one_new_user() {
    arango_system_db_test(|conn, ref mut core| {

        let new_user1 = NewUser::<Empty>::with_name("test_user_d5", "");
        let user1 = core.run(conn.execute(CreateUser::new(new_user1.clone()))).unwrap();
        assert_eq!("test_user_d5", user1.name());

        let new_user2 = NewUser::<Empty>::with_name("test_user_d6", "");
        let user2 = core.run(conn.execute(CreateUser::new(new_user2.clone()))).unwrap();
        assert_eq!("test_user_d6", user2.name());

        let new_user3 = NewUser::<Empty>::with_name("test_user_d7", "");

        let method = CreateDatabase::with_name_for_users("test_database_d43",
            vec![new_user1, new_user2, new_user3]);
        let work = conn.execute(method);
        let created = core.run(work).unwrap();

        assert!(created);

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d43"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d5"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d6"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d7"))).unwrap();
    });
}

#[test]
fn get_current_database_default_for_root_user() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetCurrentDatabase::new();
        let work = conn.execute(method);
        let database = core.run(work).unwrap();

        assert_eq!("_system", database.name());

    }, |_, _| {
    });
}

#[test]
fn get_current_database_specific_for_root_user() {
    arango_system_db_test(|conn, ref mut core| {

        let _ = core.run(conn.execute(CreateDatabase::<Empty>::with_name("test_database_d05"))).unwrap();

        let user_ds = conn.datasource().clone().use_database("test_database_d05");
        let user_conn = Connection::establish(user_ds, &core.handle()).unwrap();

        let method = GetCurrentDatabase::new();
        let work = user_conn.execute(method);
        let database = core.run(work).unwrap();

        assert_eq!("test_database_d05", database.name());

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d05"))).unwrap();
    });
}

#[test]
fn get_current_database_specific_for_user() {
    arango_system_db_test(|conn, ref mut core| {

        let user1 = NewUser::<Empty>::with_name("test_user_d8", "");

        let _ = core.run(conn.execute(CreateDatabase::with_name_for_users(
            "test_database_d81", vec![user1]))).unwrap();

        let user_ds = conn.datasource().clone()
            .with_basic_authentication("test_user_d8", "")
            .use_database("test_database_d81");
        let user_conn = Connection::establish(user_ds, &core.handle()).unwrap();

        let method = GetCurrentDatabase::new();
        let work = user_conn.execute(method);
        let database = core.run(work).unwrap();

        assert_eq!("test_database_d81", database.name());

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d81"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d8"))).unwrap();
    });
}

#[test]
fn list_databases() {
    arango_system_db_test(|conn, ref mut core| {

        let _ = core.run(conn.execute(CreateDatabase::<Empty>::with_name("test_database_d09"))).unwrap();

        let method = ListDatabases::new();
        let work = conn.execute(method);
        let databases = core.run(work).unwrap();

        assert!(databases.contains(&"_system".into()));
        assert!(databases.contains(&"test_database_d09".into()));

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d09"))).unwrap();
    });
}

#[test]
fn list_accessible_databases_for_root_user() {
    arango_system_db_test(|conn, ref mut core| {

        let _ = core.run(conn.execute(CreateDatabase::<Empty>::with_name("test_database_d010"))).unwrap();

        let method = ListAccessibleDatabases::new();
        let work = conn.execute(method);
        let databases = core.run(work).unwrap();

        assert!(databases.contains(&"_system".into()));
        assert!(databases.contains(&"test_database_d010".into()));

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d010"))).unwrap();
    });
}

#[test]
fn list_accessible_databases_for_test_user() {
    arango_system_db_test(|conn, ref mut core| {

        let user = NewUser::<Empty>::with_name("test_user_d9", "");

        let _ = core.run(conn.execute(CreateDatabase::with_name_for_users(
            "test_database_d91", vec![user.clone()]))).unwrap();
        let _ = core.run(conn.execute(CreateDatabase::with_name_for_users(
            "test_database_d92", vec![user]))).unwrap();

        let user_ds = conn.datasource().clone()
            .with_basic_authentication("test_user_d9", "")
            .use_database("test_database_d91");
        let user_conn = Connection::establish(user_ds, &core.handle()).unwrap();

        let method = ListAccessibleDatabases::new();
        let work = user_conn.execute(method);
        let databases = core.run(work).unwrap();

        assert!(databases.contains(&"test_database_d91".into()));
        assert!(databases.contains(&"test_database_d92".into()));
        assert_eq!(2, databases.len());

    }, |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d91"))).unwrap();
        let _ = core.run(conn.execute(DropDatabase::with_name("test_database_d92"))).unwrap();
        let _ = core.run(conn.execute(RemoveUser::with_name("test_user_d9"))).unwrap();
    });
}
