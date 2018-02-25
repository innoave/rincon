
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::hash::BuildHasher;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use rincon_core::api::types::{Empty, JsonValue};
use super::DEFAULT_ROOT_PASSWORD;

const PERMISSION_READ_WRITE: &str = "rw";
const PERMISSION_READ_ONLY: &str = "ro";
const PERMISSION_NONE: &str = "none";

/// This struct contains the properties of a user.
///
/// The type parameter `E` specifies the type of the extra data about the
/// user. If users are created without any extra data one can use the
/// provided `Empty` type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User<E>
    where E: UserExtra
{
    /// The name of the user as a string.
    user: String,
    /// A flag that specifies whether the user is active.
    active: bool,
    /// An object with arbitrary extra data about the user.
    extra: E,
}

impl<E> User<E>
    where E: UserExtra
{
    /// Returns the name of the user.
    pub fn name(&self) -> &str {
        &self.user
    }

    /// Returns whether the user is active or not.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns the extra data assigned to this user.
    pub fn extra(&self) -> &E {
        &self.extra
    }
}

/// The `UserExtra` trait marks a type for being used as extra information
/// in the `User`, `NewUser` and `UserUpdate` structs.
pub trait UserExtra {}

impl UserExtra for Empty {}

impl<K, V, S: BuildHasher> UserExtra for HashMap<K, V, S> {}

impl UserExtra for JsonValue {}

/// This struct specifies the properties for a new user that is going to be
/// created.
///
/// The type parameter `E` defines the type of the extra data about the user.
/// If users are created without any extra data one can use the provided
/// `Empty` type.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser<E>
    where E: UserExtra
{
    /// The name of the user as a string. This is mandatory.
    user: String,

    /// The user password as a string. If no password is specified, the empty string will be used.
    ///
    /// If you pass the special value ARANGODB_DEFAULT_ROOT_PASSWORD, then the password will be set
    /// the value stored in the environment variable ARANGODB_DEFAULT_ROOT_PASSWORD. This can be
    /// used to pass an instance variable into ArangoDB. For example, the instance identifier from
    /// Amazon.
    #[serde(skip_serializing_if = "Option::is_none")]
    passwd: Option<String>,

    /// Specifies whether the user is active. If not specified, this will
    /// default to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// An optional object with arbitrary extra data about the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<E>,
}

impl<E> NewUser<E>
    where E: UserExtra
{
    /// Constructs an new instance of `NewUser` with all attributes explicitly
    /// set.
    pub fn new<N, P, A, O>(name: N, password: P, active: A, extra: O) -> Self
        where N: Into<String>, P: Into<Option<String>>, A: Into<Option<bool>>, O: Into<Option<E>>
    {
        NewUser {
            user: name.into(),
            passwd: password.into(),
            active: active.into(),
            extra: extra.into(),
        }
    }

    /// Constructs a new instance of `NewUser` with given name and password.
    ///
    /// The user will be active by default and will not have any extra data
    /// assigned.
    pub fn with_name<N, P>(name: N, password: P) -> Self
        where N: Into<String>, P: Into<String>
    {
        NewUser {
            user: name.into(),
            passwd: Some(password.into()),
            active: None,
            extra: None,
        }
    }

    /// Constructs a new instance of `NewUser` with given name and the default
    /// root password configured for the ArangoDB-Server.
    ///
    /// The user will be active by default and will not have any extra data
    /// assigned.
    pub fn with_default_root_password<N>(name: N) -> Self
        where N: Into<String>
    {
        NewUser {
            user: name.into(),
            passwd: Some(DEFAULT_ROOT_PASSWORD.to_owned()),
            active: None,
            extra: None,
        }
    }

    /// Sets the extra data for this `NewUser`.
    pub fn set_extra<O>(&mut self, extra: O)
        where O: Into<Option<E>>
    {
        self.extra = extra.into();
    }

    /// Sets the active flag for this `NewUser`.
    pub fn set_active<A>(&mut self, active: A)
        where A: Into<Option<bool>>
    {
        self.active = active.into();
    }

    /// Returns the name of the user to be created.
    pub fn name(&self) -> &str {
        &self.user
    }

    /// Returns the password of the user to be created.
    pub fn password(&self) -> Option<&String> {
        self.passwd.as_ref()
    }

    /// Returns whether the user will be created as active or inactive.
    pub fn is_active(&self) -> Option<bool> {
        self.active
    }

    /// Returns the extra data that will be stored with the user to be
    /// created.
    pub fn extra(&self) -> Option<&E> {
        self.extra.as_ref()
    }
}

/// This struct holds properties of a user that shall be changed. It is used
/// for partially modifying an existing user.
///
/// The type parameter `E` defines the type of the extra data about the user.
/// If users are created without any extra data one can use the provided
/// `Empty` type.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdate<E>
    where E: UserExtra
{
    /// An optional password as a string if the password shall be changed.
    ///
    /// If no password is specified the password remains unchanged. The empty
    /// string is a valid password.
    passwd: Option<String>,

    /// Specifies whether the active flag of this user shall be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// An optional object with arbitrary extra data that shall replace the
    /// existing extra data of this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<E>,
}

impl<E> UserUpdate<E>
    where E: UserExtra
{
    /// Constructs an new instance of `UserUpdate` with all attributes
    /// explicitly set.
    pub fn new<P, A, O>(password: P, active: A, extra: O) -> Self
        where P: Into<Option<String>>, A: Into<Option<bool>>, O: Into<Option<E>>
    {
        UserUpdate {
            passwd: password.into(),
            active: active.into(),
            extra: extra.into(),
        }
    }

    /// Constructs a new empty instance of `UserUpdate`.
    pub fn empty() -> Self {
        UserUpdate {
            passwd: None,
            active: None,
            extra: None,
        }
    }

    /// Sets the password for this `UserUpdate`.
    pub fn set_password<P>(&mut self, password: P)
        where P: Into<Option<String>>
    {
        self.passwd = password.into();
    }

    /// Sets the extra data for this `UserUpdate`.
    pub fn set_extra<O>(&mut self, extra: O)
        where O: Into<Option<E>>
    {
        self.extra = extra.into();
    }

    /// Sets the active flag for this `UserUpdate`.
    pub fn set_active<A>(&mut self, active: A)
        where A: Into<Option<bool>>
    {
        self.active = active.into();
    }

    /// Returns the password of the user to be created.
    pub fn password(&self) -> Option<&String> {
        self.passwd.as_ref()
    }

    /// Returns whether the user will be created as active or inactive.
    pub fn is_active(&self) -> Option<bool> {
        self.active
    }

    /// Returns the extra data that will be stored with the user to be
    /// created.
    pub fn extra(&self) -> Option<&E> {
        self.extra.as_ref()
    }
}

impl<E> From<User<E>> for UserUpdate<E>
    where E: UserExtra + Clone
{
    fn from(user: User<E>) -> Self {
        UserUpdate {
            passwd: None,
            active: Some(user.active),
            extra: Some(user.extra.clone()),
        }
    }
}

/// This struct specifies an access level to be granted.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAccessLevel {
    grant: Permission,
}

impl NewAccessLevel {
    /// Constructs a new instance of `NewAccessLevel`.
    pub fn new(grant: Permission) -> Self {
        NewAccessLevel {
            grant,
        }
    }

    /// Returns the access level to be granted.
    pub fn grant(&self) -> &Permission {
        &self.grant
    }
}

/// This enum defines the possible access levels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Permission {
    /// The 'Administrate' access level.
    ReadWrite,
    /// The 'Access' access level.
    ReadOnly,
    /// The 'No access' access level.
    None,
}

impl Permission {
    pub fn as_str(&self) -> &str {
        use self::Permission::*;
        match *self {
            ReadWrite => PERMISSION_READ_WRITE,
            ReadOnly => PERMISSION_READ_ONLY,
            None => PERMISSION_NONE,
        }
    }
}

impl FromStr for Permission {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, String> {
        use self::Permission::*;
        match value {
            PERMISSION_READ_WRITE => Ok(ReadWrite),
            PERMISSION_READ_ONLY => Ok(ReadOnly),
            PERMISSION_NONE => Ok(None),
            _ => Err(format!("Not a valid permission string: {:?}", value)),
        }
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Permission::*;
        f.write_str(match *self {
            ReadWrite => "Read/Write",
            ReadOnly  => "Read Only",
            None      => "No access",
        })
    }
}

impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        Permission::from_str(&value).map_err(D::Error::custom)
    }
}
