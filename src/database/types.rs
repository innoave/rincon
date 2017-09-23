
use user::{NewUser, UserExtra};

/// `DatabaseInfo` contains information about a database.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInfo {
    /// the id of the database
    id: String,
    /// the name of the database
    name: String,
    /// the filesystem path of the database
    path: String,
    /// whether or not the database is the `_system` database
    is_system: bool,
}

impl DatabaseInfo {
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

/// The `NewDatabase` struct specifies the attributes used when creating
/// a new database.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDatabase<'a, T>
    where T: 'a + UserExtra
{
    /// Has to contain a valid database name.
    name: &'a str,
    /// Has to be an array of user objects to initially create for the new
    /// database.
    ///
    /// User information will not be changed for users that already exist. If
    /// users is not specified or does not contain any users, a default user
    /// root will be created with an empty string password. This ensures that
    /// the new database will be accessible after it is created.
    users: &'a [NewUser<'a, T>],
}

impl<'a, T> NewDatabase<'a, T>
    where T: 'a + UserExtra
{
    /// Constructs a new instance of `NewDatabase` with the specified
    /// database name.
    pub fn with_name(name: &'a str) -> Self {
        NewDatabase {
            name,
            users: &[],
        }
    }

    /// Returns a new instance of `NewDatabase` with the name of `self`
    /// and the given `users`.
    pub fn for_users(&self, users: &'a [NewUser<T>]) -> Self {
        NewDatabase {
            name: self.name,
            users,
        }
    }

    /// Returns the name of the database to be created.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns a slice of users that will be assigned to the newly created
    /// database.
    pub fn users(&self) -> &[NewUser<T>] {
        self.users
    }
}
