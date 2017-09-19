
use api::{Method, Operation, Parameters, Prepare, RpcErrorType};
use user::UserInfo;
use super::types::*;

/// Retrieves information about the current database.
#[derive(Debug, PartialEq, Eq)]
pub struct GetCurrentDatabase {}

impl GetCurrentDatabase {
    pub fn new() -> Self {
        GetCurrentDatabase {}
    }
}

impl Method for GetCurrentDatabase {
    type Result = DatabaseInfo;
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for GetCurrentDatabase {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/database/current")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }
}

/// Retrieves the list of all existing databases.
///
/// *Note*: retrieving the list of databases is only possible from within the
/// `_system` database.
/// *Note*: You should use the `user::ListAccessibleDatabases` to fetch the
/// list of the available databases now.
#[derive(Debug, PartialEq, Eq)]
pub struct ListOfDatabases {}

impl ListOfDatabases {
    pub fn new() -> Self {
        ListOfDatabases {}
    }
}

impl Method for ListOfDatabases {
    type Result = Vec<String>;
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for ListOfDatabases {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/database")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }
}

/// Retrieves the list of all databases the current user can access without
/// specifying a different username or password.
#[derive(Debug, PartialEq, Eq)]
pub struct ListAccessibleDatabases {}

impl ListAccessibleDatabases {
    pub fn new() -> Self {
        ListAccessibleDatabases {}
    }
}

impl Method for ListAccessibleDatabases {
    type Result = Vec<String>;
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for ListAccessibleDatabases {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/database/user")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }
}

/// Creates a new database.
///
/// *Note*: creating a new database is only possible from within the `_system`
/// database.
#[derive(Debug, PartialEq, Eq)]
pub struct CreateDatabase<'a, T>
    where T: 'a + UserInfo
{
    database: NewDatabase<'a, T>,
}

impl<'a, T> CreateDatabase<'a, T>
    where T: 'a + UserInfo
{
    pub fn new(database: NewDatabase<'a, T>) -> Self {
        CreateDatabase {
            database,
        }
    }

    pub fn database(&self) -> &NewDatabase<'a, T> {
        &self.database
    }
}

impl<'a, T> Method for CreateDatabase<'a, T>
    where T: 'a + UserInfo
{
    type Result = bool;
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl<'a, T> Prepare for CreateDatabase<'a, T>
    where T: 'a + UserInfo
{
    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from("/_api/database")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }
}

/// Drops the database along with all data stored in it.
///
/// *Note*: dropping a database is only possible from within the `_system`
/// database. The `_system` database itself cannot be dropped.
#[derive(Debug, PartialEq, Eq)]
pub struct DropDatabase {
    database_name: String,
}

impl DropDatabase {
    pub fn new<N>(database_name: N) -> Self
        where N: Into<String>
    {
        DropDatabase {
            database_name: database_name.into(),
        }
    }

    pub fn database_name(&self) -> &str {
        &self.database_name
    }
}

impl Method for DropDatabase {
    type Result = bool;
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for DropDatabase {
    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from("/_api/database/") + &self.database_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }
}
