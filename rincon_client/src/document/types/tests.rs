use std::collections::HashMap;

use serde_json;

use super::*;
use rincon_core::api::types::JsonString;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MyContent {
    a: String,
    b: i32,
}

#[test]
fn serialize_struct_document_without_key() {
    let content = MyContent {
        a: "Hugo".to_owned(),
        b: 42,
    };

    let new_document = NewDocument::from_content(content);
    let json = serde_json::to_string(&new_document).unwrap();

    assert_eq!(r#"{"a":"Hugo","b":42}"#, &json);
}

#[test]
fn serialize_struct_document_with_key() {
    let content = MyContent {
        a: "Hugo".to_owned(),
        b: 42,
    };

    let new_document = NewDocument::from_content(content).with_key(DocumentKey::new("29384"));
    let json = serde_json::to_string(&new_document).unwrap();

    assert_eq!(r#"{"_key":"29384","a":"Hugo","b":42}"#, &json);
}

#[test]
fn serialize_hashmap_document_without_key() {
    let mut content = HashMap::new();
    content.insert("a", json!("Hugo"));
    content.insert("b", json!(42));
    let content = content;

    let new_document = NewDocument::from_content(content);
    let json = serde_json::to_string(&new_document).unwrap();

    assert!(r#"{"a":"Hugo","b":42}"# == &json || r#"{"b":42,"a":"Hugo"}"# == &json);
}

#[test]
fn serialize_hashmap_document_with_key() {
    let mut content = HashMap::new();
    content.insert("a", json!("Hugo"));
    content.insert("b", json!(42));
    let content = content;

    let new_document = NewDocument::from_content(content).with_key(DocumentKey::new("29384"));
    let json = serde_json::to_string(&new_document).unwrap();

    assert!(
        r#"{"_key":"29384","a":"Hugo","b":42}"# == &json
            || r#"{"_key":"29384","b":42,"a":"Hugo"}"# == &json
    );
}

#[test]
fn serialize_raw_json_document_without_key() {
    let content = JsonString::from_str_unchecked(r#"{"a":"Hugo","b":42}"#);

    let new_document = NewDocument::from_content(content);
    let json = serde_json::to_string(&new_document).unwrap();

    assert_eq!(r#"{"a":"Hugo","b":42}"#, &json);
}

#[test]
fn serialize_raw_json_document_with_key() {
    let content = JsonString::from_str_unchecked(r#"{"a":"Hugo","b":42}"#);

    let new_document = NewDocument::from_content(content).with_key(DocumentKey::new("29384"));
    let json = serde_json::to_string(&new_document).unwrap();

    assert_eq!(r#"{"_key":"29384","a":"Hugo","b":42}"#, &json);
}

#[test]
fn serialize_primitive_content_document_without_key() {
    let content = 42;

    let new_document = NewDocument::from_content(content);
    let json = serde_json::to_string(&new_document).unwrap();

    assert_eq!("42", &json);
}

#[test]
fn serialize_primitive_content_document_with_key() {
    let content = 42;

    let new_document = NewDocument::from_content(content).with_key(DocumentKey::new("29384"));
    let result = serde_json::to_string(&new_document);

    if let Err(error) = result {
        assert_eq!("Error(\"Invalid document content! Only types that serialize into valid Json objects are supported. But got: 42\", line: 0, column: 0)", format!("{:?}", error));
    } else {
        panic!("Error expected, but got: {:?}", result);
    }
}

#[test]
fn deserialize_struct_document() {
    let json_string =
        r#"{"_id":"customers/29384","_key":"29384","_rev":"aOIey283aew","a":"Hugo","b":42}"#;

    let document = serde_json::from_str(json_string).unwrap();

    let expected = Document::new(
        DocumentId::new("customers", "29384"),
        DocumentKey::new("29384"),
        Revision::new("aOIey283aew"),
        MyContent {
            a: "Hugo".to_owned(),
            b: 42,
        },
    );
    assert_eq!(expected, document);
}

#[test]
fn deserialize_struct_document_just_inserted() {
    let json_string = r#"{
            "_id": "customers/29384",
            "_key": "29384",
            "_rev": "aOIey283aew",
            "new": {
                "_id": "customers/29384",
                "_key": "29384",
                "_rev": "aOIey283aew",
                "a": "Hugo",
                "b":42
            }
        }"#;

    let document = serde_json::from_str(json_string).unwrap();

    let expected = Document::new(
        DocumentId::new("customers", "29384"),
        DocumentKey::new("29384"),
        Revision::new("aOIey283aew"),
        MyContent {
            a: "Hugo".to_owned(),
            b: 42,
        },
    );
    assert_eq!(expected, document);
}

#[test]
fn deserialize_json_document_just_inserted() {
    let json_string = r#"{
            "_id": "customers/29384",
            "_key": "29384",
            "_rev": "aOIey283aew",
            "new": {
                "_id": "customers/29384",
                "_key": "29384",
                "_rev": "aOIey283aew",
                "a": "Hugo",
                "b":42
            }
        }"#;

    let document = serde_json::from_str(json_string).unwrap();

    let expected = JsonString::from_str_unchecked(
        "{\
         \"_id\":\"customers/29384\",\
         \"_key\":\"29384\",\
         \"_rev\":\"aOIey283aew\",\
         \"new\":{\
         \"_id\":\"customers/29384\",\
         \"_key\":\"29384\",\
         \"_rev\":\"aOIey283aew\",\
         \"a\":\"Hugo\",\
         \"b\":42\
         }\
         }",
    );
    assert_eq!(expected, document);
}

#[test]
fn serialize_struct_document_update_without_revision() {
    let update = MyContent {
        a: "Hugo".to_owned(),
        b: 42,
    };

    let new_document = DocumentUpdate::new(DocumentKey::new("770815"), update);
    let json = serde_json::to_string(&new_document).unwrap();

    assert_eq!(r#"{"_key":"770815","a":"Hugo","b":42}"#, &json);
}

#[test]
fn serialize_struct_document_update_with_revision() {
    let update = MyContent {
        a: "Hugo".to_owned(),
        b: 42,
    };

    let new_document = DocumentUpdate::new(DocumentKey::new("770815"), update)
        .with_revision(Revision::new("_WkyoIaj--_"));
    let json = serde_json::to_string(&new_document).unwrap();

    assert_eq!(
        r#"{"_key":"770815","_rev":"_WkyoIaj--_","a":"Hugo","b":42}"#,
        &json
    );
}
