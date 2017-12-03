
extern crate dotenv;
extern crate hyper;
extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_connector;
extern crate rincon_test_helper;

use std::io;
use std::time::Duration;

use tokio_core::reactor::Core;

use rincon_core::api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use rincon_core::api::types::JsonValue;
use rincon_core::arango::protocol::{PARAM_DETAILS, PATH_API_VERSION};
use rincon_connector::connection::{self, Connection};
use rincon_connector::datasource::DataSource;

use rincon_test_helper::*;

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
struct GetServerVersion {
    details: bool,
}

impl Method for GetServerVersion {
    type Result = JsonValue;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: None,
    };
}

impl Prepare for GetServerVersion {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_VERSION)
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if self.details {
            params.insert(PARAM_DETAILS, true);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[ignore]
#[test]
fn establish_connection_timeout() {
    dotenv::dotenv().ok();
    let mut core = Core::new().unwrap();
    // 10.255.255.1 is a not a routable IP address
    let datasource = DataSource::from_url("http://10.255.255.1:8529").unwrap()
        .with_timeout(Duration::from_millis(500));
    let conn = Connection::establish(&MyUserAgent, datasource, &core.handle()).unwrap();

    let method = GetServerVersion { details: true };
    let work = conn.execute(method);

    match core.run(work) {
        Err(connection::Error::CommunicationFailed(hyper::Error::Io(e))) => {
            assert_eq!(e.kind(), io::ErrorKind::TimedOut);
        }
        e => panic!("Expected timeout error, got {:?}", e),
    }
}
