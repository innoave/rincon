
extern crate dotenv;
extern crate futures;
extern crate log4rs;
extern crate tokio_core;

extern crate arangodb_core;
extern crate arangodb_connector;
extern crate arangodb_test_helper;

use std::env;

use dotenv::dotenv;
use tokio_core::reactor::Core;

use arangodb_test_helper::*;
use arangodb_core::api::auth::Credentials;
use arangodb_core::api::ErrorCode;
use arangodb_connector::connection::{Connection, Error};
use arangodb_connector::datasource::DataSource;
use arangodb_connector::authentication::*;

#[test]
fn authenticate_root_user() {
    dotenv().ok();

    let username = env::var(ENV_ARANGO_ROOT_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_ROOT_PASSWORD).unwrap();

    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let system_ds = DataSource::from_url(&db_url).unwrap();

    let mut core = Core::new().unwrap();
    let conn = Connection::establish(system_ds, &core.handle()).unwrap();

    let method = Authenticate::with_user(username, password);
    let result = core.run(conn.execute(method)).unwrap();

    assert!(!result.jwt().is_empty())
}

#[test]
fn authenticate_with_invalid_credentials() {
    dotenv().ok();

    let credentials = Credentials::new("not existing", "user");

    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let system_ds = DataSource::from_url(&db_url).unwrap();

    let mut core = Core::new().unwrap();
    let conn = Connection::establish(system_ds, &core.handle()).unwrap();

    let method = Authenticate::with_credentials(credentials);
    let result = core.run(conn.execute(method));

    match result {
        Err(Error::ApiError(error)) => {
            assert_eq!(401, error.status_code());
            assert_eq!(ErrorCode::HttpUnauthorized, error.error_code());
            assert_eq!("Wrong credentials", error.message());
        },
        _ => panic!("Expected error but got {:?}", result),
    }
}
