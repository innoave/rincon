
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

use dotenv::dotenv;
use tokio_core::reactor::Core;

use arangodb_client::admin::*;
use arangodb_client::connection::Connection;
use arangodb_client::datasource::DataSource;

fn init_logging() {
    log4rs::init_file("tests/log4rs.yml", Default::default()).unwrap();
}

fn init_db_test() -> (Core, Connection) {
    dotenv().ok();
    let core = Core::new().unwrap();
    let datasource = DataSource::from_url("http://localhost:8529").unwrap();
    let conn = Connection::establish(datasource, &core.handle()).unwrap();
    (core, conn)
}

#[test]
fn get_target_version() {
    let (mut core, conn) = init_db_test();

    let method = GetTargetVersion::new();
    let work = conn.execute(method);
    let target_version = core.run(work).unwrap();

    assert_eq!("30202", target_version.version());
    assert_eq!(false, target_version.error());
    assert_eq!(200, target_version.code());
}

#[test]
fn get_server_version_without_details() {
    let (mut core, conn) = init_db_test();

    let method = GetServerVersion::new();
    let work = conn.execute(method);
    let server_version = core.run(work).unwrap();

    assert_eq!("arango", server_version.server());
    assert_eq!("community", server_version.license());
    assert_eq!("3.2.2", server_version.version());
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

    assert_eq!("2", server_version.sub());
}
