
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

use rincon_core::api::types::Empty;
use rincon_core::api::user_agent::{UserAgent, Version};
use rincon_connector::connection::Connection;
use rincon_connector::datasource::DataSource;
use rincon_client::collection::{CreateCollection, DropCollection};
use rincon_client::database::{CreateDatabase, DropDatabase, ListAccessibleDatabases, NewDatabase};
use rincon_client::user::{NewUser, RemoveUser};

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
pub fn init_logging() {
    log4rs::init_file("tests/log4rs.yml", Default::default()).unwrap();
}

#[allow(dead_code)]
pub fn arango_system_db_test<Test, CleanUp>(test: Test, clean_up: CleanUp) -> ()
    where
        Test: FnOnce(Connection, &mut Core) -> () + panic::UnwindSafe,
        CleanUp: FnOnce(Connection, &mut Core) -> (),
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();

    let system_ds = DataSource::from_url(&db_url).unwrap();

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let conn = Connection::establish(&MyUserAgent, system_ds.clone(), &core.handle()).unwrap();
        test(conn, &mut core);
    });

    let mut core = Core::new().unwrap();
    let sys_conn = Connection::establish(&MyUserAgent, system_ds, &core.handle()).unwrap();
    clean_up(sys_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_user_db_test<Test, CleanUp>(test: Test, clean_up: CleanUp) -> ()
    where
        Test: FnOnce(Connection, &mut Core) -> () + panic::UnwindSafe,
        CleanUp: FnOnce(Connection, &mut Core) -> (),
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let database = env::var(ENV_ARANGO_TEST_DATABASE).unwrap();
    let username = env::var(ENV_ARANGO_TEST_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_TEST_PASSWORD).unwrap();

    let mut core = Core::new().unwrap();

    setup_database_if_not_existing(&username, &password, &database, &db_url, &mut core);

    let user_ds = DataSource::from_url(&db_url).unwrap()
        .with_basic_authentication(&username, &password)
        .use_database(database);

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let user_conn = Connection::establish(&MyUserAgent, user_ds.clone(), &core.handle()).unwrap();
        test(user_conn, &mut core);
    });

    let user_conn = Connection::establish(&MyUserAgent, user_ds, &core.handle()).unwrap();
    clean_up(user_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_test_with_user_db<Test>(user: &str, database: &str, test: Test) -> ()
    where
        Test: FnOnce(Connection, &mut Core) -> () + panic::UnwindSafe,
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();

    let mut core = Core::new().unwrap();

    let system_ds = DataSource::from_url(&db_url).unwrap();
    let sys_conn = Connection::establish(&MyUserAgent, system_ds, &core.handle()).unwrap();

    setup_database(user, "", database, &sys_conn, &mut core);

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let user_ds = DataSource::from_url(&db_url).unwrap()
            .with_basic_authentication(user, "")
            .use_database(database);
        let conn = Connection::establish(&MyUserAgent, user_ds, &core.handle()).unwrap();
        test(conn, &mut core);
    });

    teardown_database(user, database, &sys_conn, &mut core);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn arango_test_with_document_collection<Test>(collection: &str, test: Test) -> ()
    where
        Test: FnOnce(Connection, &mut Core) -> () + panic::UnwindSafe,
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let database = env::var(ENV_ARANGO_TEST_DATABASE).unwrap();
    let username = env::var(ENV_ARANGO_TEST_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_TEST_PASSWORD).unwrap();

    let mut core = Core::new().unwrap();

    setup_database_if_not_existing(&username, &password, &database, &db_url, &mut core);

    let user_ds = DataSource::from_url(&db_url).unwrap()
        .with_basic_authentication(&username, &password)
        .use_database(database);
    let user_conn = Connection::establish(&MyUserAgent, user_ds.clone(), &core.handle()).unwrap();

    core.run(user_conn.execute(CreateCollection::documents_with_name(collection)))
        .expect(&format!("Error on creating document collection: {}", collection));

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let conn = Connection::establish(&MyUserAgent, user_ds, &core.handle()).unwrap();
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
        Test: FnOnce(Connection, &mut Core) -> () + panic::UnwindSafe,
{
    dotenv().ok();
    let db_url = env::var(ENV_ARANGO_DB_URL).unwrap();
    let database = env::var(ENV_ARANGO_TEST_DATABASE).unwrap();
    let username = env::var(ENV_ARANGO_TEST_USERNAME).unwrap();
    let password = env::var(ENV_ARANGO_TEST_PASSWORD).unwrap();

    let mut core = Core::new().unwrap();

    setup_database_if_not_existing(&username, &password, &database, &db_url, &mut core);

    let user_ds = DataSource::from_url(&db_url).unwrap()
        .with_basic_authentication(&username, &password)
        .use_database(database);
    let user_conn = Connection::establish(&MyUserAgent, user_ds.clone(), &core.handle()).unwrap();

    core.run(user_conn.execute(CreateCollection::edges_with_name(collection)))
        .expect(&format!("Error on creating edge collection: {}", collection));

    let result = panic::catch_unwind(|| {
        let mut core = Core::new().unwrap();
        let conn = Connection::establish(&MyUserAgent, user_ds, &core.handle()).unwrap();
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
fn is_database_existing(database: &str, conn: &Connection, core: &mut Core) -> bool {
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
    let system_ds = DataSource::from_url(&db_url).unwrap();
    let sys_conn = Connection::establish(&MyUserAgent, system_ds, &core.handle()).unwrap();

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
    sys_conn: &Connection,
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
    sys_conn: &Connection,
    core: &mut Core
)
    where
        User: Into<String>,
        Db: Into<String>,
{
    let _ = core.run(sys_conn.execute(DropDatabase::with_name(database))).unwrap();
    let _ = core.run(sys_conn.execute(RemoveUser::with_name(user))).unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MyUserAgent;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MyVersion;

impl UserAgent for MyUserAgent {
    fn name(&self) -> &str {
        "rincon"
    }

    fn version(&self) -> &Version {
        &MyVersion
    }

    fn homepage(&self) -> &str {
        "https://github.com/innoave/rincon"
    }
}

impl Version for MyVersion {
    fn major(&self) -> &str {
        "2"
    }

    fn minor(&self) -> &str {
        "5"
    }

    fn patch(&self) -> &str {
        "9"
    }

    fn pre(&self) -> &str {
        ""
    }
}
