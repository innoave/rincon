
use std::collections::HashMap;
use std::fmt::{self, Display};

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde_json::Value;

use api::types::Empty;
use super::DEFAULT_ROOT_PASSWORD;

/// This struct contains the properties of a user.
///
/// The type parameter `E` specifies the type of the extra data about the
/// user. If users are created without any extra data one can use the
/// provided `Empty` type.
#[derive(Clone, Debug, Deserialize)]
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

/// The `UserExtra' trait marks a type for being used as extra information
/// in the `User`, `NewUser` and `UserUpdate` structs.
pub trait UserExtra {}

impl UserExtra for Empty {}

impl<K, V> UserExtra for HashMap<K, V> {}

impl UserExtra for Value {}

/// This struct specifies the properties for a new user that is going to be
/// created.
///
/// The type parameter `E` defines the type of the extra data about the user.
/// If users are created without any extra data one can use the provided
/// `Empty` type.
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    pub fn new(name: String, password: Option<String>, active: Option<bool>, extra: Option<E>) -> Self {
        NewUser {
            user: name,
            passwd: password,
            active,
            extra,
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
    pub fn set_extra(&mut self, extra: Option<E>) {
        self.extra = extra;
    }

    /// Sets the active flag for this `NewUser`.
    pub fn set_active(&mut self, active: Option<bool>) {
        self.active = active;
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
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    pub fn new(password: Option<String>, active: Option<bool>, extra: Option<E>) -> Self {
        UserUpdate {
            passwd: password,
            active,
            extra,
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
    pub fn set_password(&mut self, password: Option<String>) {
        self.passwd = password;
    }

    /// Sets the extra data for this `UserUpdate`.
    pub fn set_extra(&mut self, extra: Option<E>) {
        self.extra = extra;
    }

    /// Sets the active flag for this `UserUpdate`.
    pub fn set_active(&mut self, active: Option<bool>) {
        self.active = active;
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
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    pub fn from_str(value: &str) -> Result<Self, String> {
        use self::Permission::*;
        match value {
            "rw"   => Ok(ReadWrite),
            "ro"   => Ok(ReadOnly),
            "none" => Ok(None),
            _      => Err(format!("Not a valid permission string: {}", value)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        use self::Permission::*;
        match *self {
            ReadWrite => "rw",
            ReadOnly  => "ro",
            None      => "none",
        }
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
        deserializer.deserialize_str(PermissionVisitor)
    }
}

struct PermissionVisitor;

impl<'de> Visitor<'de> for PermissionVisitor {
    type Value = Permission;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid permission string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Permission::from_str(value).map_err(|err| E::custom(err))
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

#[cfg(test)]
mod tests {

    use serde_json;

    use api::types::EMPTY;
    use super::*;

    #[test]
    fn serialize_new_user_without_info_to_json() {
        let new_user: NewUser<Empty> = NewUser::with_name("cesar", "s3cr3t");
        let json_str = serde_json::to_string(&new_user).unwrap();
        assert_eq!(r#"{"user":"cesar","passwd":"s3cr3t"}"#, &json_str);
    }

    #[test]
    fn deserialize_user_without_info_from_json() {
        let json_str = r#"{"user":"cesar","active":true,"extra":{}}"#;
        let user: User<Empty> = serde_json::from_str(json_str).unwrap();
        assert_eq!("cesar", user.name());
        assert!(user.is_active());
        assert_eq!(&EMPTY, user.extra());
    }

    #[test]
    fn serialize_inactive_new_user_to_json() {
        let mut new_user: NewUser<Empty> = NewUser::with_name("cesar", "s3cr3t");
        new_user.set_active(Some(false));
        let json_str = serde_json::to_string(&new_user).unwrap();
        assert_eq!(r#"{"user":"cesar","passwd":"s3cr3t","active":false}"#, &json_str);
    }

}
