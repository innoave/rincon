
extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_connector;
extern crate rincon_client;
extern crate rincon_test_helper;

use tokio_core::reactor::Core;

use rincon_core::api::auth::Credentials;
use rincon_core::api::ErrorCode;
use rincon_core::api::connector::{Connector, Error, Execute};
use rincon_connector::http::BasicConnector;
use rincon_client::auth::methods::*;

use rincon_test_helper::*;


#[test]
fn authenticate_root_user() {
    let system_ds = system_datasource();

    let (username, password) = root_user();

    let mut core = Core::new().unwrap();
    let connector = BasicConnector::new(&MyUserAgent, system_ds, &core.handle()).unwrap();
    let conn = connector.system_connection();

    let method = Authenticate::with_user(username, password);
    let result = core.run(conn.execute(method)).unwrap();

    assert!(!result.jwt().is_empty())
}

#[test]
fn authenticate_with_invalid_credentials() {
    let system_ds = system_datasource();

    let credentials = Credentials::new("not existing", "user");

    let mut core = Core::new().unwrap();
    let connector = BasicConnector::new(&MyUserAgent, system_ds, &core.handle()).unwrap();
    let conn = connector.system_connection();

    let method = Authenticate::with_credentials(&credentials);
    let result = core.run(conn.execute(method));

    match result {
        Err(Error::Method(error)) => {
            assert_eq!(401, error.status_code());
            assert_eq!(ErrorCode::HttpUnauthorized, error.error_code());
            assert_eq!("Wrong credentials", error.message());
        },
        _ => panic!("Expected error but got {:?}", result),
    }
}
