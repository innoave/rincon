
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

use std::env;

use dotenv::dotenv;
use tokio_core::reactor::Core;

use arangodb_client::admin::*;
use arangodb_client::connection::Connection;
use arangodb_client::datasource::DataSource;

fn config_test() {
    init_logging();
    init_env();
}

fn init_logging() {
    log4rs::init_file("tests/log4rs.yml", Default::default()).unwrap();
}

fn init_env() {
    dotenv().ok();
    println!("{}", env::var("ARANGO_ROOT_PASSWORD").unwrap());
}

fn init_datasource() -> DataSource {
    DataSource::from_url("http://localhost:8529").unwrap()
}

#[test]
fn get_target_version_of_arangodb_server() {
    config_test();
    let mut core = Core::new().unwrap();
    let datasource = init_datasource();
    let conn = Connection::establish(datasource, &core.handle()).unwrap();

    let method = GetTargetVersion::new();
    let work = conn.execute(method);

    let target_version = core.run(work).unwrap();

    assert_eq!("30202", target_version.version());
    assert_eq!(false, target_version.error());
    assert_eq!(200, target_version.code());
}
