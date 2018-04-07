
#![doc(html_root_url = "https://docs.rs/rincon_test_helper/0.1.0")]

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
)]

extern crate dotenv;
extern crate log4rs;
extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_connector;
extern crate rincon_client;

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::panic;
use std::time::{Duration, Instant};

use dotenv::dotenv;
use tokio_core::reactor::Core;

use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::datasource::DataSource;
use rincon_core::api::types::Empty;
use rincon_connector::http::{BasicConnection, BasicConnector};
use rincon_client::collection::methods::{CreateCollection, DropCollection};
use rincon_client::database::methods::{CreateDatabase, DropDatabase, ListAccessibleDatabases};
use rincon_client::database::types::NewDatabase;
use rincon_client::user::methods::DeleteUser;
use rincon_client::user::types::NewUser;

pub const ENV_ARANGO_DB_URL: &str = "ARANGO_DB_URL";
#[allow(dead_code)]
pub const ENV_ARANGO_ROOT_USERNAME: &str = "ARANGO_ROOT_USERNAME";
#[allow(dead_code)]
pub const ENV_ARANGO_ROOT_PASSWORD: &str = "ARANGO_ROOT_PASSWORD";
#[allow(dead_code)]
pub const ENV_ARANGO_TEST_DATABASE: &str = "ARANGO_TEST_DATABASE";
#[allow(dead_code)]
pub const ENV_ARANGO_TEST_USERNAME: &str = "ARANGO_TEST_USERNAME";
#[allow(dead_code)]
pub const ENV_ARANGO_TEST_PASSWORD: &str = "ARANGO_TEST_PASSWORD";

const LOCK_FILE: &str = "db_test.lock";

#[allow(dead_code)]
pub fn root_user() -> (String, String) {
    dotenv().ok();
    let username = env::var(ENV_ARANGO_ROOT_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_ROOT_PASSWORD).unwrap();
    (username, password)
}

#[allow(dead_code)]
pub fn init_logging() {
    log4rs::init_file("tests/log4rs.yml", Default::default()).unwrap();
}

#[allow(dead_code)]
pub fn system_datasource() -> DataSource {
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();

    DataSource::from_url(&db_url).unwrap()
}

#[allow(dead_code)]
pub fn test_datasource() -> (DataSource, String) {
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let database = env::var(ENV_ARANGO_TEST_DATABASE).unwrap();
    let username = env::var(ENV_ARANGO_TEST_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_TEST_PASSWORD).unwrap();

    (DataSource::from_url(&db_url).unwrap()
        .with_basic_authentication(&username, &password),
    database)
}

#[allow(dead_code)]
pub fn arango_session_test<Test, CleanUp>(test: Test, clean_up: CleanUp) -> ()
    where
        Test: FnOnce(BasicConnector, Core) -> () + panic::UnwindSafe,
        CleanUp: FnOnce(BasicConnection, &mut Core) -> (),
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();

    let system_ds = DataSource::from_url(&db_url).unwrap();

    let result = panic::catch_unwind(|| {
        let core = Core::new().unwrap();
        let connector = BasicConnector::new(system_ds.clone(), &core.handle()).unwrap();
        test(connector, core);
    });

    let mut core = Core::new().unwrap();
    let connector = BasicConnector::new(system_ds.clone(), &core.handle()).unwrap();
    let sys_conn = connector.system_connection();
    clean_up(sys_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_session_test_with_user_db<Test>(user: &str, database: &str, test: Test) -> ()
    where
        Test: FnOnce(BasicConnector, Core) -> () + panic::UnwindSafe,
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();

    let mut core = Core::new().unwrap();

    let system_ds = DataSource::from_url(&db_url).unwrap();
    let connector = BasicConnector::new(system_ds.clone(), &core.handle()).unwrap();
    let sys_conn = connector.system_connection();

    setup_database(user, "", database, &sys_conn, &mut core);

    let result = panic::catch_unwind(|| {
        let core = Core::new().unwrap();
        let user_ds = DataSource::from_url(&db_url).unwrap()
            .with_basic_authentication(user, "");
        let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
        test(connector, core);
    });

    teardown_database(user, database, &sys_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_system_db_test<Test, CleanUp>(test: Test, clean_up: CleanUp) -> ()
    where
        Test: FnOnce(BasicConnection, &mut Core) -> () + panic::UnwindSafe,
        CleanUp: FnOnce(BasicConnection, &mut Core) -> (),
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();

    let system_ds = DataSource::from_url(&db_url).unwrap();

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let connector = BasicConnector::new(system_ds.clone(), &core.handle()).unwrap();
        let conn = connector.system_connection();
        test(conn, &mut core);
    });

    let mut core = Core::new().unwrap();
    let connector = BasicConnector::new(system_ds.clone(), &core.handle()).unwrap();
    let sys_conn = connector.system_connection();
    clean_up(sys_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_user_db_test<Test, CleanUp>(test: Test, clean_up: CleanUp) -> ()
    where
        Test: FnOnce(BasicConnection, &mut Core) -> () + panic::UnwindSafe,
        CleanUp: FnOnce(BasicConnection, &mut Core) -> (),
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let database = env::var(ENV_ARANGO_TEST_DATABASE).unwrap();
    let username = env::var(ENV_ARANGO_TEST_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_TEST_PASSWORD).unwrap();

    let mut core = Core::new().unwrap();

    setup_database_if_not_existing(&username, &password, &database, &db_url, &mut core);

    let user_ds = DataSource::from_url(&db_url).unwrap()
        .with_basic_authentication(&username, &password);

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
        let user_conn = connector.connection(&database);
        test(user_conn, &mut core);
    });

    let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
    let user_conn = connector.connection(&database);
    clean_up(user_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_test_with_user_db<Test>(user: &str, database: &str, test: Test) -> ()
    where
        Test: FnOnce(BasicConnection, &mut Core) -> () + panic::UnwindSafe,
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();

    let mut core = Core::new().unwrap();

    let system_ds = DataSource::from_url(&db_url).unwrap();
    let connector = BasicConnector::new(system_ds.clone(), &core.handle()).unwrap();
    let sys_conn = connector.system_connection();

    setup_database(user, "", database, &sys_conn, &mut core);

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let user_ds = DataSource::from_url(&db_url).unwrap()
            .with_basic_authentication(user, "");
        let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
        let conn = connector.connection(database);
        test(conn, &mut core);
    });

    teardown_database(user, database, &sys_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_test_with_document_collection<Test>(collection: &str, test: Test) -> ()
    where
        Test: FnOnce(BasicConnection, &mut Core) -> () + panic::UnwindSafe,
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let database = env::var(ENV_ARANGO_TEST_DATABASE).unwrap();
    let username = env::var(ENV_ARANGO_TEST_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_TEST_PASSWORD).unwrap();

    let mut core = Core::new().unwrap();

    setup_database_if_not_existing(&username, &password, &database, &db_url, &mut core);

    let user_ds = DataSource::from_url(&db_url).unwrap()
        .with_basic_authentication(&username, &password);
    let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
    let user_conn = connector.connection(&database);

    core.run(user_conn.execute(CreateCollection::documents_with_name(collection)))
        .expect(&format!("Error on creating document collection: {}", collection));

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
        let conn = connector.connection(&database);
        test(conn, &mut core);
    });

    let dropped = core.run(user_conn.execute(DropCollection::with_name(collection)));
    if let Err(ref error) = dropped {
        panic!("Error on dropping collection {}: {:?}", collection, error);
    }
    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_test_with_edge_collection<Test>(collection: &str, test: Test) -> ()
    where
        Test: FnOnce(BasicConnection, &mut Core) -> () + panic::UnwindSafe,
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let database = env::var(ENV_ARANGO_TEST_DATABASE).unwrap();
    let username = env::var(ENV_ARANGO_TEST_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_TEST_PASSWORD).unwrap();

    let mut core = Core::new().unwrap();

    setup_database_if_not_existing(&username, &password, &database, &db_url, &mut core);

    let user_ds = DataSource::from_url(&db_url).unwrap()
        .with_basic_authentication(&username, &password);
    let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
    let user_conn = connector.connection(&database);

    core.run(user_conn.execute(CreateCollection::edges_with_name(collection)))
        .expect(&format!("Error on creating edge collection: {}", collection));

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let connector = BasicConnector::new(user_ds.clone(), &core.handle()).unwrap();
        let conn = connector.connection(&database);
        test(conn, &mut core);
    });

    let dropped = core.run(user_conn.execute(DropCollection::with_name(collection)));
    if let Err(ref error) = dropped {
        panic!("Error on dropping collection {}: {:?}", collection, error);
    }
    assert!(result.is_ok())
}

#[allow(dead_code)]
fn obtain_file_lock() -> Result<File, io::Error> {
    let mut lock_file = env::current_dir().unwrap();
    lock_file.push(LOCK_FILE);
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(lock_file)
}

#[allow(dead_code)]
fn release_file_lock() {
    let mut lock_file = env::current_dir().unwrap();
    lock_file.push(LOCK_FILE);
    fs::remove_file(&lock_file)
        .expect(&format!("Error deleting lock file: {:?}", &lock_file));
}

#[allow(dead_code)]
fn is_database_existing(database: &str, conn: &BasicConnection, core: &mut Core) -> bool {
    let db_list = core.run(conn.execute(ListAccessibleDatabases::new()))
        .expect(&format!("Could not get list of accessible databases for connection: {:?}", conn));
    db_list.contains(&database.to_owned())
}

#[allow(dead_code)]
fn setup_database_if_not_existing(
    user: &str,
    pass: &str,
    database: &str,
    db_url: &str,
    core: &mut Core,
) {
    let system_ds = DataSource::from_url(db_url).unwrap();
    let connector = BasicConnector::new(system_ds.clone(), &core.handle()).unwrap();
    let sys_conn = connector.system_connection();

    let timeout = Duration::from_secs(15);
    let time = Instant::now();

    loop {
        let lock = obtain_file_lock();
        if lock.is_ok() {
            if !is_database_existing(database, &sys_conn, core) {
                setup_database(user, pass, database, &sys_conn, core);
            }
            release_file_lock();
            break;
        }
        if Instant::now().duration_since(time) > timeout {
            release_file_lock();
            panic!("Could not create shared database {}", database);
        }
        let wait = Instant::now();
        while Instant::now().duration_since(wait) < Duration::from_millis(100) {}
    }
}

#[allow(dead_code)]
fn setup_database<User, Pass, Db>(
    user: User,
    pass: Pass,
    database: Db,
    sys_conn: &BasicConnection,
    core: &mut Core,
)
    where
        User: Into<String>,
        Pass: Into<String>,
        Db: Into<String>,
{
    let new_user = NewUser::<Empty>::with_name(user, pass);

    let new_database = NewDatabase::new(database, vec![new_user]);
    let created = core.run(sys_conn.execute(CreateDatabase::new(new_database))).unwrap();

    assert!(created, "Error on setting up test database");
}

#[allow(dead_code)]
fn teardown_database<User, Db>(
    user: User,
    database: Db,
    sys_conn: &BasicConnection,
    core: &mut Core
)
    where
        User: Into<String>,
        Db: Into<String>,
{
    let _ = core.run(sys_conn.execute(DropDatabase::with_name(database))).unwrap();
    let _ = core.run(sys_conn.execute(DeleteUser::with_name(user))).unwrap();
}
