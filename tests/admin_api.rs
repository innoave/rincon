
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
    let (mut core, conn) = init_db_test();

    let method = GetTargetVersion::new();
    let work = conn.execute(method);
    let target_version = core.run(work).unwrap();

    assert_eq!("30204", target_version.version());
}

#[test]
fn get_server_version_without_details() {
    let (mut core, conn) = init_db_test();

    let method = GetServerVersion::new();
    let work = conn.execute(method);
    let server_version = core.run(work).unwrap();

    assert_eq!("arango", server_version.server());
    assert_eq!("community", server_version.license());
    assert_eq!("3.2.4", server_version.version());
}

#[test]
fn get_server_version_major_part() {
    let (mut core, conn) = init_db_test();

    let method = GetServerVersion::new();
    let work = conn.execute(method);
    let server_version = core.run(work).unwrap();

    assert_eq!("3", server_version.major());
}

#[test]
fn get_server_version_minor_part() {
    let (mut core, conn) = init_db_test();

    let method = GetServerVersion::new();
    let work = conn.execute(method);
    let server_version = core.run(work).unwrap();

    assert_eq!("2", server_version.minor());
}

#[test]
fn get_server_version_sub_part() {
    let (mut core, conn) = init_db_test();

    let method = GetServerVersion::new();
    let work = conn.execute(method);
    let server_version = core.run(work).unwrap();

    assert_eq!("4", server_version.sub());
}
