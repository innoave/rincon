
extern crate dotenv;
extern crate hyper;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_client;

use std::io;
use std::time::Duration;

use tokio_core::reactor::Core;

use arangodb_client::admin::GetServerVersion;
use arangodb_client::connection::{self, Connection};
use arangodb_client::datasource::DataSource;


#[test]
fn establish_connection_timeout() {
    dotenv::dotenv().ok();
    let mut core = Core::new().unwrap();
    // 10.255.255.1 is a not a routable IP address
    let datasource = DataSource::from_url("http://10.255.255.1:8529").unwrap()
        .with_timeout(Duration::from_millis(500));
    let conn = Connection::establish(datasource, &core.handle()).unwrap();

    let method = GetServerVersion::new();
    let work = conn.execute(method);

    match core.run(work) {
        Err(connection::Error::CommunicationFailed(hyper::Error::Io(e))) => {
            assert_eq!(e.kind(), io::ErrorKind::TimedOut);
        }
        e => panic!("Expected timeout error, got {:?}", e),
    }
}
