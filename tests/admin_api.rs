
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

use tokio_core::reactor::Core;

use arangodb_client::admin::*;
use arangodb_client::connection::Connection;
use arangodb_client::datasource::DataSource;

fn init_logging() {
    log4rs::init_file("tests/log4rs.yml", Default::default()).unwrap();
}

#[test]
fn get_target_version_of_arangodb_server() {
    init_logging();
    let mut core = Core::new().unwrap();
    let datasource = DataSource::default();
    let conn = Connection::establish(datasource, &core.handle()).unwrap();

    let method = GetTargetVersion::new();
    let work = conn.execute(method);

    let target_version = core.run(work).unwrap();

    assert_eq!("30202", target_version.version());
    assert_eq!(false, target_version.error());
    assert_eq!(200, target_version.code());
}
