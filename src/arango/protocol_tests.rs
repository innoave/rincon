
use super::protocol::*;

#[test]
fn get_handle_key_from_str() {
    let handle_key = HandleKey::from_string("index id", "12341".to_owned()).unwrap();
    assert_eq!(String::from("12341"), handle_key.deconstruct());
}

#[test]
fn get_handle_key_from_str_with_slash_character_in_the_middle() {
    let result = HandleKey::from_string("index id", "mine/12341".to_owned());
    assert_eq!(Err("A index id key must not contain any '/' character, but got: \"mine/12341\"".to_owned()), result);
}

#[test]
fn get_handle_key_from_str_with_slash_character_at_the_beginning() {
    let result = HandleKey::from_string("index id", "/12341".to_owned());
    assert_eq!(Err("A index id key must not contain any '/' character, but got: \"/12341\"".to_owned()), result);
}

#[test]
fn get_handle_key_from_str_with_slash_character_at_the_end() {
    let result = HandleKey::from_string("index id", "12341/".to_owned());
    assert_eq!(Err("A index id key must not contain any '/' character, but got: \"12341/\"".to_owned()), result);
}

#[test]
fn get_handle_from_str() {
    let handle = Handle::from_str("index id", "mine/12341").unwrap();
    assert_eq!("mine/12341", &handle.to_string());
    let (context, key) = handle.deconstruct();
    assert_eq!(String::from("mine"), context);
    assert_eq!(String::from("12341"), key);
}

#[test]
fn get_handle_from_str_without_context() {
    let result = Handle::from_str("index id", "12341");
    assert_eq!(Err("index id does not have a context: \"12341\"".to_owned()), result);
}

#[test]
fn get_handle_from_str_with_empty_context() {
    let result = Handle::from_str("index id", "/12341");
    assert_eq!(Err("Invalid index id: \"/12341\"".to_owned()), result);
}

#[test]
fn get_handle_from_str_with_empty_key() {
    let result = Handle::from_str("index id", "mine/");
    assert_eq!(Err("Invalid index id: \"mine/\"".to_owned()), result);
}

#[test]
fn get_handle_option_from_str_with_context_and_key() {
    let handle_option = HandleOption::from_str("index id", "mine/12341").unwrap();
    assert_eq!(HandleOption::Qualified(Handle { context: "mine".to_owned(), key: "12341".to_owned() }), handle_option);
}

#[test]
fn get_handle_option_from_str_with_key_only() {
    let handle_option = HandleOption::from_str("index id", "12341").unwrap();
    assert_eq!(HandleOption::Local(HandleKey("12341".to_owned())), handle_option);
}
