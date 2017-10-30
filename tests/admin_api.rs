
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::admin::*;

#[test]
fn get_target_version() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetTargetVersion::new();
        let work = conn.execute(method);
        let target_version = core.run(work).unwrap();

        assert_eq!("30206", target_version.version());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_without_details() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("arango", server_version.server());
        assert_eq!("community", server_version.license());
        assert_eq!("3.2.6", server_version.version());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_major_part() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("3", server_version.major());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_minor_part() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("2", server_version.minor());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_sub_part() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("6", server_version.sub());

    }, |_, _| {
    });
}
