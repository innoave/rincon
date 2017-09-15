
use dotenv::dotenv;
use log4rs;
use tokio_core::reactor::Core;

use arangodb_client::connection::Connection;
use arangodb_client::datasource::DataSource;


pub fn init_logging() {
    log4rs::init_file("tests/log4rs.yml", Default::default()).unwrap();
}

pub fn init_db_test() -> (Core, Connection) {
    dotenv().ok();
    let core = Core::new().unwrap();
    let datasource = DataSource::from_url("http://localhost:8529").unwrap();
    let conn = Connection::establish(datasource, &core.handle()).unwrap();
    (core, conn)
}
