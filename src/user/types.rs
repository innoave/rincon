
use api::{Empty};
use super::DEFAULT_ROOT_PASSWORD;

pub trait UserExtra {}

impl UserExtra for Empty {}

/// The `User` struct contains information about a user.
///
/// The type parameter `T` specifies the type of the extra data about the
/// user. If users are created without any extra data one can use the
/// provided `EmptyUserInfo` type.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User<T>
    where T: UserExtra
{
    /// The name of the user as a string.
    user: String,
    /// A flag that specifies whether the user is active.
    active: bool,
    /// An object with arbitrary extra data about the user.
    extra: T,
}

impl<T> User<T>
    where T: UserExtra
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
    pub fn extra(&self) -> &T {
        &self.extra
    }
}

/// The `NewUser` struct specifies the attributes used for creating a new user.
///
/// The type parameter `T` defines the type of the extra data about the user.
/// If users are created without any extra data one can use the provided
/// `EmptyUserInfo` type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser<'a, T>
    where T: 'a + UserExtra
{
    /// The name of the user as a string. This is mandatory.
    user: &'a str,

    /// The user password as a string. If no password is specified, the empty string will be used.
    ///
    /// If you pass the special value ARANGODB_DEFAULT_ROOT_PASSWORD, then the password will be set
    /// the value stored in the environment variable ARANGODB_DEFAULT_ROOT_PASSWORD. This can be
    /// used to pass an instance variable into ArangoDB. For example, the instance identifier from
    /// Amazon.
    passwd: &'a str,

    /// Specifies whether the user is active. If not specified, this will
    /// default to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// An optional object with arbitrary extra data about the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Box<&'a T>>,
}

impl<'a, T> NewUser<'a, T>
    where T: UserExtra
{
    /// Constructs a new instance of `NewUser` with given name and password.
    ///
    /// The user will be active by default and will not have any extra data
    /// assigned.
    pub fn with_name(name: &'a str, password: &'a str) -> Self {
        NewUser {
            user: name,
            passwd: password,
            active: None,
            extra: None,
        }
    }

    /// Constructs a new instance of `NewUser` with given name and the default
    /// root password configured for the ArangoDB-Server.
    ///
    /// The user will be active by default and will not have any extra data
    /// assigned.
    pub fn with_default_root_password(name: &'a str) -> Self {
        NewUser {
            user: name,
            passwd: DEFAULT_ROOT_PASSWORD,
            active: None,
            extra: None,
        }
    }

    /// Returns a copy of this `NewUser` but with given extra data.
    pub fn with_extra(&self, extra: &'a T) -> Self {
        NewUser {
            user: self.user,
            passwd: self.passwd,
            active: self.active,
            extra: Some(Box::new(extra)),
        }
    }

    /// Returns a copy of this `NewUser` but its `active` attribute will be
    /// set to the given value.
    pub fn set_active(&self, active: bool) -> Self {
        NewUser {
            user: self.user,
            passwd: self.passwd,
            active: Some(active),
            extra: self.extra.clone(),
        }
    }

    /// Returns the name of the user to be created.
    pub fn name(&self) -> &str {
        self.user
    }

    /// Returns the password of the user to be created.
    pub fn password(&self) -> &str {
        self.passwd
    }

    /// Returns whether the user will be created as active or inactive.
    pub fn is_active(&self) -> Option<bool> {
        self.active
    }

    /// Returns the extra data that will be stored with the user to be
    /// created.
    pub fn extra(&self) -> Option<&Box<&T>> {
        self.extra.as_ref()
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    use api::EMPTY;
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
        let new_user: NewUser<Empty> = NewUser::with_name("cesar", "s3cr3t").set_active(false);
        let json_str = serde_json::to_string(&new_user).unwrap();
        assert_eq!(r#"{"user":"cesar","passwd":"s3cr3t","active":false}"#, &json_str);
    }

}
