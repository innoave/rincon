
extern crate futures;

extern crate arangodb_client;

use futures::Future;

use arangodb_client::admin::*;
use arangodb_client::connection::{self, Connection};
use arangodb_client::datasource::DataSource;

#[test]
fn get_target_version_of_arangodb_server() {
    let datasource = DataSource::default();
    let conn = Connection::establish(datasource).unwrap();

    let method = GetTargetVersion::new();
    let result: Result<TargetVersion, connection::Error> =
        conn.execute(method).wait();
    let target_version = result.unwrap();

    assert_eq!("30202", target_version.version());
    assert_eq!(false, target_version.error());
    assert_eq!(200, target_version.code());
}
