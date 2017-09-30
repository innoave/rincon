
use std::env;
use std::panic;

use dotenv::dotenv;
use log4rs;
use tokio_core::reactor::Core;

use arangodb_client::api::Empty;
use arangodb_client::connection::Connection;
use arangodb_client::database::{CreateDatabase, DropDatabase, NewDatabase};
use arangodb_client::datasource::DataSource;
use arangodb_client::user::{NewUser, RemoveUser};

const ENV_ARANGO_DB_URL: &str = "ARANGO_DB_URL";

#[allow(dead_code)]
pub fn init_logging() {
    log4rs::init_file("tests/log4rs.yml", Default::default()).unwrap();
}

#[allow(dead_code)]
pub fn arango_system_db_test<T>(test: T) -> ()
    where T: FnOnce(Connection, &mut Core) -> () + panic::UnwindSafe
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let system_ds = DataSource::from_url(&db_url).unwrap();

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let conn = Connection::establish(system_ds, &core.handle()).unwrap();
        test(conn, &mut core);
    });

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_user_db_test<T>(user: &str, database: &str, test: T) -> ()
    where T: FnOnce(Connection, &mut Core) -> () + panic::UnwindSafe
{
    dotenv().ok();
    let mut core = Core::new().unwrap();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let system_ds = DataSource::from_url(&db_url).unwrap();
    let conn = Connection::establish(system_ds.clone(), &core.handle()).unwrap();
    setup_database(user, database, &conn, &mut core);

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let user_ds = DataSource::from_url(&db_url).unwrap()
            .with_basic_authentication(user, "")
            .use_database(database);
        let conn = Connection::establish(user_ds, &core.handle()).unwrap();
        test(conn, &mut core);
    });

    teardown_database(user, database, &conn, &mut core);
    assert!(result.is_ok())
}

#[allow(dead_code)]
fn setup_database<U, D>(user: U, database: D, conn: &Connection, core: &mut Core)
    where U: Into<String>, D: Into<String>
{
    let new_user: NewUser<Empty> = NewUser::with_name(user, "");

    let new_database = NewDatabase::new(database, vec![new_user.clone()]);
    let created = core.run(conn.execute(CreateDatabase::new(new_database))).unwrap();

    assert!(created, "Error on setting up test database");
}

#[allow(dead_code)]
fn teardown_database<U, D>(user: U, database: D, conn: &Connection, core: &mut Core)
    where U: Into<String>, D: Into<String>
{
    let _ = core.run(conn.execute(DropDatabase::with_name(database))).unwrap();
    let _ = core.run(conn.execute(RemoveUser::with_name(user))).unwrap();
}
