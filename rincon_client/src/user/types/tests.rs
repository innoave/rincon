
use serde_json;

use rincon_core::api::types::{Empty, EMPTY};
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
