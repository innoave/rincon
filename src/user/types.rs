
use super::DEFAULT_ROOT_PASSWORD;

pub trait UserInfo {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User<T>
    where T: UserInfo
{
    user: String,
    active: bool,
    extra: T,
}

impl<T> User<T>
    where T: UserInfo
{
    pub fn name(&self) -> &str {
        &self.user
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn info(&self) -> &T {
        &self.extra
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser<'a, T>
    where T: 'a + UserInfo
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
    where T: UserInfo
{
    pub fn with_name(name: &'a str, password: &'a str) -> Self {
        NewUser {
            user: name,
            passwd: password,
            active: None,
            extra: None,
        }
    }

    pub fn with_default_root_password(name: &'a str) -> Self {
        NewUser {
            user: name,
            passwd: DEFAULT_ROOT_PASSWORD,
            active: None,
            extra: None,
        }
    }

    pub fn with_info(mut self, info: &'a T) -> Self {
        self.extra = Some(Box::from(info));
        self
    }

    pub fn set_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn name(&self) -> &str {
        self.user
    }

    pub fn password(&self) -> &str {
        self.passwd
    }

    pub fn is_active(&self) -> Option<bool> {
        self.active
    }

    pub fn info(&self) -> &Option<Box<&T>> {
        &self.extra
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmptyUserInfo {}

impl UserInfo for EmptyUserInfo {}


#[cfg(test)]
mod tests {

    use serde_json;

    use super::*;

    #[test]
    fn serialize_new_user_without_info_to_json() {
        let new_user: NewUser<EmptyUserInfo> = NewUser::with_name("cesar", "s3cr3t");
        let json_str = serde_json::to_string(&new_user).unwrap();
        assert_eq!(r#"{"user":"cesar","passwd":"s3cr3t"}"#, &json_str);
    }

    #[test]
    fn deserialize_user_without_info_from_json() {
        let json_str = r#"{"user":"cesar","active":true,"extra":{}}"#;
        let user: User<EmptyUserInfo> = serde_json::from_str(json_str).unwrap();
        assert_eq!("cesar", user.name());
        assert!(user.is_active());
        assert_eq!(&EmptyUserInfo {}, user.info());
    }

    #[test]
    fn serialize_inactive_new_user_to_json() {
        let new_user: NewUser<EmptyUserInfo> = NewUser::with_name("cesar", "s3cr3t").set_active(false);
        let json_str = serde_json::to_string(&new_user).unwrap();
        assert_eq!(r#"{"user":"cesar","passwd":"s3cr3t","active":false}"#, &json_str);
    }

}
