
#[macro_use] extern crate galvanic_assert;

extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_client;
extern crate rincon_connector;
extern crate rincon_session;
extern crate rincon_test_helper;

use galvanic_assert::matchers::*;

use tokio_core::reactor::Core;

use rincon_core::api::types::Empty;
use rincon_core::api::connector::Execute;
use rincon_connector::http::JsonHttpConnector;
use rincon_client::database::methods::DropDatabase;
use rincon_session::*;
use rincon_session::client::*;

use rincon_test_helper::*;


#[test]
fn create_database() {
    arango_session_test(|connector, core| {

        let arango = ArangoSession::new(connector, core);

        let database = arango.create_database::<_, Empty>("the_social_network",
            vec![NewUser::with_name("an_user", "a_pass")]).unwrap();

        assert_that!(&database.name(), eq("the_social_network"));
    },
    |conn, ref mut core| {
        let _ = core.run(conn.execute(DropDatabase::with_name("the_social_network")));
    });
}

#[test]
fn use_database() {
    let core = Core::new().unwrap();

    let datasource = system_datasource();
    let connector = JsonHttpConnector::new(datasource, &core.handle()).unwrap();

    let arango = ArangoSession::new(connector, core);

    let database = arango.use_database_with_name("the_social_network");

    assert_that!(&database.name(), eq("the_social_network"));
}
