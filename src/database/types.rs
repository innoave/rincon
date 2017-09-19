
use user::{NewUser, UserInfo};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInfo {
    id: String,
    name: String,
    path: String,
    is_system: bool,
}

impl DatabaseInfo {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn is_system(&self) -> bool {
        self.is_system
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDatabase<'a, T>
    where T: 'a + UserInfo
{
    /// Has to contain a valid database name.
    name: &'a str,
    /// Has to be an array of user objects to initially create for the new database.
    ///
    /// User information will not be changed for users that already exist. If users is not specified
    /// or does not contain any users, a default user root will be created with an empty string
    /// password. This ensures that the new database will be accessible after it is created.
    users: &'a [NewUser<'a, T>],
}

impl<'a, T> NewDatabase<'a, T>
    where T: 'a + UserInfo
{
    pub fn with_name(name: &'a str) -> Self {
        NewDatabase {
            name,
            users: &[],
        }
    }

    pub fn for_users(mut self, users: &'a [NewUser<T>]) -> Self {
        self.users = users;
        self
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn users(&self) -> &[NewUser<T>] {
        self.users
    }
}
