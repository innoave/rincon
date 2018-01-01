
use std::iter::{FromIterator, IntoIterator};

use user::types::{NewUser, UserExtra};

/// This struct holds the properties of a database.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    /// the id of the database
    id: String,
    /// the name of the database
    name: String,
    /// the filesystem path of the database
    path: String,
    /// whether or not the database is the `_system` database
    is_system: bool,
}

impl Database {
    /// Returns the id of the database.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the database.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the filesystem path of the database.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns whether or not the database is the `_system` database.
    ///
    /// Returns `true` if the database is the `_system` database,
    /// `false` otherwise.
    pub fn is_system(&self) -> bool {
        self.is_system
    }
}

/// This struct specifies the properties of a database that is going to be
/// created.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDatabase<E>
    where E: UserExtra
{
    /// Has to contain a valid database name.
    name: String,
    /// Has to be an array of user objects to initially create for the new
    /// database.
    ///
    /// User information will not be changed for users that already exist. If
    /// users is not specified or does not contain any users, a default user
    /// root will be created with an empty string password. This ensures that
    /// the new database will be accessible after it is created.
    users: Vec<NewUser<E>>,
}

impl<E> NewDatabase<E>
    where E: UserExtra
{
    /// Constructs a new instance of `NewDatabase` with all attributes
    /// set explicitly.
    pub fn new<N, U>(name: N, users: U) -> Self
        where N: Into<String>, U: IntoIterator<Item=NewUser<E>>
    {
        NewDatabase {
            name: name.into(),
            users: Vec::from_iter(users.into_iter()),
        }
    }

    /// Constructs a new instance of `NewDatabase` with the specified
    /// database name.
    ///
    /// The method will be called with an empty user array.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewDatabase {
            name: name.into(),
            users: Vec::new(),
        }
    }

    /// Sets the users for this `NewDatabase` that should be assigned to the
    /// newly created database.
    pub fn set_users<U>(&mut self, users: U)
        where U: IntoIterator<Item=NewUser<E>>
    {
        self.users = Vec::from_iter(users.into_iter());
    }

    /// Returns the name of the database to be created.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a slice of users that will be assigned to the newly created
    /// database.
    pub fn users(&self) -> &[NewUser<E>] {
        &self.users
    }
}
