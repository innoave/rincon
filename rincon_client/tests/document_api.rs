
#[macro_use] extern crate serde_derive;
extern crate tokio_core;

extern crate rincon_core;
extern crate rincon_connector;
extern crate rincon_client;
extern crate rincon_test_helper;

use rincon_core::api::ErrorCode;
use rincon_core::api::connector::{Error, Execute};
use rincon_core::api::types::JsonString;
use rincon_client::document::methods::*;
use rincon_client::document::types::*;

use rincon_test_helper::*;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Customer {
    name: String,
    contact: Vec<Contact>,
    gender: Gender,
    age: u16,
    active: bool,
    groups: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Contact {
    address: String,
    kind: ContactType,
    tag: Option<Tag>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Tag(String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum ContactType {
    Email,
    Phone,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct VipCustomer {
    name: String,
    contact: Vec<Contact>,
    age: u16,
    status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct CustomerUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Vec<Contact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<String>>,
}

#[test]
fn insert_struct_document_without_key() {
    arango_test_with_document_collection("customers01", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer);
        let method = InsertDocument::new("customers01", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers01", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
    });
}

#[test]
fn insert_struct_document_without_key_and_return_new() {
    arango_test_with_document_collection("customers02", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer.clone());
        let method = InsertDocumentReturnNew::new("customers02", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers02", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn insert_struct_document_with_key() {
    arango_test_with_document_collection("customers03", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer)
            .with_key(DocumentKey::new("94711"));
        let method = InsertDocument::new("customers03", new_document)
            .with_force_wait_for_sync(true);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers03/94711", &document.id().to_string());
        assert_eq!("customers03", document.id().collection_name());
        assert_eq!("94711", document.id().document_key());
        assert_eq!("94711", document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
    });
}

#[test]
fn insert_struct_document_with_key_and_return_new() {
    arango_test_with_document_collection("customers04", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer.clone())
            .with_key(DocumentKey::new("94712"));
        let method = InsertDocumentReturnNew::new("customers04", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers04/94712", &document.id().to_string());
        assert_eq!("customers04", document.id().collection_name());
        assert_eq!("94712", document.id().document_key());
        assert_eq!("94712", document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn insert_json_document_with_key_and_return_new() {
    arango_test_with_document_collection("customers05", |conn, ref mut core| {

        let json_doc = r#"{
            "name": "Jane Doe",
            "contact": [
                {
                    "address": "1-555-234523",
                    "kind": "Phone",
                    "tag": "work"
                }
            ],
            "gender": "Female",
            "age": 42,
            "active": true,
            "groups": []
        }"#;

        let new_document = NewDocument::from_content(JsonString::from_str(json_doc))
            .with_key(DocumentKey::new("7713996"));
        let method = InsertDocumentReturnNew::new("customers05", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers05", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert!(document.content().as_str().starts_with(r#"{"_id":"customers05/7713996","_key":"7713996","_rev":""#));
        assert!(document.content().as_str().ends_with(r#"","active":true,"age":42,"contact":[{"address":"1-555-234523","kind":"Phone","tag":"work"}],"gender":"Female","groups":[],"name":"Jane Doe"}"#));
    });
}

#[test]
fn insert_multiple_struct_documents_without_key() {
    arango_test_with_document_collection("customers06", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1);
        let new_document2 = NewDocument::from_content(customer2);
        let method = InsertDocuments::new("customers06", vec![new_document1, new_document2])
            .with_force_wait_for_sync(true);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref header1) = documents.get(0).unwrap() {
            assert_eq!("customers06", header1.id().collection_name());
            assert!(!header1.id().document_key().is_empty());
            assert_eq!(header1.id().document_key(), header1.key().as_str());
            assert!(!header1.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 1, but got: {:?}", documents.get(0));
        }

        if let Ok(ref header2) = documents.get(1).unwrap() {
            assert_eq!("customers06", header2.id().collection_name());
            assert!(!header2.id().document_key().is_empty());
            assert_eq!(header2.id().document_key(), header2.key().as_str());
            assert!(!header2.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 2, but got: {:?}", documents.get(1));
        }
    });
}

#[test]
fn insert_multiple_struct_documents_without_key_and_return_new() {
    arango_test_with_document_collection("customers07", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1.clone());
        let new_document2 = NewDocument::from_content(customer2.clone());
        let method = InsertDocumentsReturnNew::new("customers07", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref document1) = documents.get(0).unwrap() {
            assert_eq!("customers07", document1.id().collection_name());
            assert!(!document1.id().document_key().is_empty());
            assert_eq!(document1.id().document_key(), document1.key().as_str());
            assert!(!document1.revision().as_str().is_empty());
            assert_eq!(&customer1, document1.content());
        } else {
            panic!("Expected document 1, but got: {:?}", documents.get(0));
        }
        if let Ok(ref document2) = documents.get(1).unwrap() {
            assert_eq!("customers07", document2.id().collection_name());
            assert!(!document2.id().document_key().is_empty());
            assert_eq!(document2.id().document_key(), document2.key().as_str());
            assert!(!document2.revision().as_str().is_empty());
            assert_eq!(&customer2, document2.content());
        } else {
            panic!("Expected document 2, but got: {:?}", documents.get(1));
        }
    });
}

#[test]
fn insert_multiple_struct_documents_with_key() {
    arango_test_with_document_collection("customers08", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1)
            .with_key(DocumentKey::new("94711"));
        let new_document2 = NewDocument::from_content(customer2)
            .with_key(DocumentKey::new("90815"));
        let method = InsertDocuments::new("customers08", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref header1) = documents.get(0).unwrap() {
            assert_eq!("customers08/94711", &header1.id().to_string());
            assert_eq!("customers08", header1.id().collection_name());
            assert_eq!("94711", header1.id().document_key());
            assert_eq!("94711", header1.key().as_str());
            assert!(!header1.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 1, but got: {:?}", documents.get(0))
        }

        if let Ok(ref header2) = documents.get(1).unwrap() {
            assert_eq!("customers08/90815", &header2.id().to_string());
            assert_eq!("customers08", header2.id().collection_name());
            assert_eq!("90815", header2.id().document_key());
            assert_eq!("90815", header2.key().as_str());
            assert!(!header2.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 2, but got: {:?}", documents.get(1))
        }
    });
}

#[test]
fn insert_multiple_struct_documents_with_key_and_return_new() {
    arango_test_with_document_collection("customers09", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1.clone())
            .with_key(DocumentKey::new("94712"));
        let new_document2 = NewDocument::from_content(customer2.clone())
            .with_key(DocumentKey::new("90815"));
        let method = InsertDocumentsReturnNew::new("customers09", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref document1) = documents.get(0).unwrap() {
            assert_eq!("customers09/94712", &document1.id().to_string());
            assert_eq!("customers09", document1.id().collection_name());
            assert_eq!("94712", document1.id().document_key());
            assert_eq!("94712", document1.key().as_str());
            assert!(!document1.revision().as_str().is_empty());
            assert_eq!(&customer1, document1.content());
        } else {
            panic!("Expected document 1, but got: {:?}", documents.get(0));
        }

        if let Ok(ref document2) = documents.get(1).unwrap() {
            assert_eq!("customers09/90815", &document2.id().to_string());
            assert_eq!("customers09", document2.id().collection_name());
            assert_eq!("90815", document2.id().document_key());
            assert_eq!("90815", document2.key().as_str());
            assert!(!document2.revision().as_str().is_empty());
            assert_eq!(&customer2, document2.content());
        } else {
            panic!("Expected document 2, but got: {:?}", documents.get(1));
        }
    });
}

#[test]
fn get_document_as_struct_inserted_as_struct() {
    arango_test_with_document_collection("customers10", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers10", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::with_id(document_id.clone());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers10", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_as_struct_inserted_as_json_string() {
    arango_test_with_document_collection("customers11", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let json_doc = r#"{
            "name": "Jane Doe",
            "contact": [
                {
                    "address": "1-555-234523",
                    "kind": "Phone",
                    "tag": "work"
                }
            ],
            "gender": "Female",
            "age": 42,
            "active": true,
            "groups": []
        }"#;

        let header = core.run(conn.execute(InsertDocument::new(
            "customers11", NewDocument::from_content(JsonString::new(json_doc))
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::with_id(document_id.clone());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers11", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_as_json_string_inserted_as_struct() {
    arango_test_with_document_collection("customers12", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let header = core.run(conn.execute(InsertDocument::new(
            "customers12", NewDocument::from_content(customer.clone())
                .with_key(DocumentKey::new("7713996"))
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::with_id(document_id.clone());
        let document: Document<JsonString> = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers12", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        let expected = r#"{"active":true,"age":42,"contact":[{"address":"1-555-234523","kind":"Phone","tag":"work"}],"gender":"Female","groups":[],"name":"Jane Doe"}"#;
        assert_eq!(expected, document.content().as_str());
    });
}

#[test]
fn get_document_if_revision_matches() {
    arango_test_with_document_collection("customers13", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers13", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::with_id(document_id.clone())
            .with_if_match(revision.as_str().to_owned());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers13", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_if_revision_is_not_a_match() {
    arango_test_with_document_collection("customers14", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers14", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::with_id(document_id.clone())
            .with_if_non_match(String::from("not") + revision.as_str());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers14", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_but_revision_does_not_match() {
    arango_test_with_document_collection("customers15", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers15", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, _, revision) = header.deconstruct();

        let method = GetDocument::<Customer>::with_id(document_id)
            .with_if_match(String::from("not") + revision.as_str());
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::Method(error)) => {
                assert_eq!(412, error.status_code());
                assert_eq!(ErrorCode::ArangoConflict, error.error_code());
                assert_eq!("precondition failed", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[test]
fn get_document_for_id_that_does_not_exist() {
    arango_test_with_document_collection("customers16", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers16", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (_, document_key, _) = header.deconstruct();

        let method = GetDocument::<Customer>::with_id(DocumentId::new("customers16", "not_existing99"));
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::Method(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoDocumentNotFound, error.error_code());
                assert_eq!("document not found", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }

        let method = GetDocument::<Customer>::with_id(DocumentId::new("not_existing99", document_key.as_str()));
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::Method(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoCollectionNotFound, error.error_code());
                assert_eq!("collection not found: not_existing99", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[ignore] //TODO refactor get document header to document exists (with possibly returning the revision)
#[test]
fn get_document_header() {
    arango_test_with_document_collection("customers20", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let inserted = core.run(conn.execute(InsertDocument::new(
            "customers20", NewDocument::from_content(customer.clone())
                .with_key(DocumentKey::new("7721264"))
        ))).unwrap();

        let method = GetDocumentHeader::with_id(inserted.id().clone());
        let result = core.run(conn.execute(method)).unwrap();

        assert_eq!((), result);
    });
}

#[test]
fn replace_with_struct_document_without_revision() {
    arango_test_with_document_collection("customers30", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers30", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement);
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers30", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_ne!(&revision, updated.revision());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_revision() {
    arango_test_with_document_collection("customers31", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers31", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement)
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers31", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type() {
    arango_test_with_document_collection("customers32", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers32", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement)
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers32", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type_return_old() {
    arango_test_with_document_collection("customers33", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers33", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_return_old(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers33", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type_return_new() {
    arango_test_with_document_collection("customers34", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers34", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_return_new(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers34", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type_return_old_and_new() {
    arango_test_with_document_collection("customers35", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers35", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(revision.clone());
        let method = ReplaceDocument::new(document_id.clone(), document_update)
            .with_return_new(true)
            .with_return_old(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers35", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_ignore_revisions_return_old_and_new() {
    arango_test_with_document_collection("customers36", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers36", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(Revision::new("wrong_revision"));
        let method = ReplaceDocument::new(document_id.clone(), document_update)
            .with_ignore_revisions(true)
            .with_return_old(true)
            .with_return_new(true)
            .with_force_wait_for_sync(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers36", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_not_existing_revision() {
    arango_test_with_document_collection("customers37", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers37", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, _) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement)
            .with_revision(Revision::new("wrong_revision"));
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_ignore_revisions(false)
            .with_return_old(true)
            .with_return_new(true)
            .with_force_wait_for_sync(true);
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::Method(error)) => {
                assert_eq!(412, error.status_code());
                assert_eq!(ErrorCode::ArangoConflict, error.error_code());
                assert_eq!("precondition failed", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[test]
fn replace_with_struct_document_with_if_match_return_old_and_new() {
    arango_test_with_document_collection("customers38", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers38", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone());
        let method = ReplaceDocument::new(document_id.clone(), document_update)
            .with_if_match(revision.as_str().to_owned())
            .with_return_old(true)
            .with_return_new(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers38", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_if_match_unknown_revision() {
    arango_test_with_document_collection("customers39", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers39", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, _) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement);
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_if_match("wrong_revision".to_owned())
            .with_return_old(true)
            .with_return_new(true);
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::Method(error)) => {
                assert_eq!(412, error.status_code());
                assert_eq!(ErrorCode::ArangoConflict, error.error_code());
                assert_eq!("precondition failed", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[test]
fn modify_struct_document() {
    arango_test_with_document_collection("customers40", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: None,
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers40", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let update = CustomerUpdate {
            name: None,
            contact: Some(vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ]),
            gender: None,
            age: Some(43),
            active: None,
            groups: None,
        };

        let document_update = DocumentUpdate::new(document_key.clone(), update);
        let method = ModifyDocument::<_, Customer, Customer>::new(document_id.clone(), document_update)
            .with_return_new(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers40", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_ne!(&revision, updated.revision());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        let updated_content = updated.new_content().unwrap();
        assert_eq!("Jane Doe", &updated_content.name);
        assert_eq!(&Gender::Female, &updated_content.gender);
        assert_eq!(43, updated_content.age);
        assert_eq!(true, updated_content.active);
        assert_eq!(&Vec::<String>::new(), &updated_content.groups);
        let updated_contact = &updated_content.contact[0];
        assert_eq!("1-555-8212494", updated_contact.address);
        assert_eq!(&ContactType::Phone, &updated_contact.kind);
        assert_eq!(Some(&Tag("mobile".to_owned())), updated_contact.tag.as_ref());
    });
}

#[test]
fn insert_two_struct_documents_with_same_key() {
    arango_test_with_document_collection("customers50", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1)
            .with_key(DocumentKey::new("94711"));
        let new_document2 = NewDocument::from_content(customer2)
            .with_key(DocumentKey::new("94711"));
        let method = InsertDocuments::new("customers50", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref header1) = documents.get(0).unwrap() {
            assert_eq!("customers50/94711", &header1.id().to_string());
            assert_eq!("customers50", header1.id().collection_name());
            assert_eq!("94711", header1.id().document_key());
            assert_eq!("94711", header1.key().as_str());
            assert!(!header1.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 1, but got: {:?}", documents.get(0))
        }

        if let Err(ref error) = documents.get(1).unwrap() {
            assert_eq!(ErrorCode::ArangoUniqueConstraintViolated, error.code());
            assert_eq!("unique constraint violated - in index 0 of type primary over [\"_key\"]", error.message());
        } else {
            panic!("Expected method error, but got: {:?}", documents.get(1))
        }
    });
}

#[test]
fn insert_two_struct_documents_with_same_key_and_return_new() {
    arango_test_with_document_collection("customers51", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1.clone())
            .with_key(DocumentKey::new("94712"));
        let new_document2 = NewDocument::from_content(customer2.clone())
            .with_key(DocumentKey::new("94712"));
        let method = InsertDocumentsReturnNew::new("customers51", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref document1) = documents.get(0).unwrap() {
            assert_eq!("customers51/94712", &document1.id().to_string());
            assert_eq!("customers51", document1.id().collection_name());
            assert_eq!("94712", document1.id().document_key());
            assert_eq!("94712", document1.key().as_str());
            assert!(!document1.revision().as_str().is_empty());
            assert_eq!(&customer1, document1.content());
        } else {
            panic!("Expected document 1, but got: {:?}", documents.get(0));
        }

        if let Err(ref error) = documents.get(1).unwrap() {
            assert_eq!(ErrorCode::ArangoUniqueConstraintViolated, error.code());
            assert_eq!("unique constraint violated - in index 0 of type primary over [\"_key\"]", error.message());
        } else {
            panic!("Expected method error, but got: {:?}", documents.get(1))
        }
    });
}

#[test]
fn replace_multiple_struct_documents_without_revision() {
    arango_test_with_document_collection("customers60", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers60", vec![
                NewDocument::from_content(customer1),
                NewDocument::from_content(customer2),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let replacement1 = Customer {
            name: "Nicolas Smith".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-3948294".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 32,
            active: true,
            groups: vec![],
        };

        let replacement2 = Customer {
            name: "Cece Kutrapali".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-1334908".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let method = ReplaceDocuments::<Customer, _>::new("customers60", vec![
            DocumentUpdate::new(original1.key().clone(), replacement1),
            DocumentUpdate::new(original2.key().clone(), replacement2),
        ]);
        let updates = core.run(conn.execute(method)).unwrap();

        if let Ok(ref updated1) = updates.get(0).unwrap() {
            assert_eq!(original1.id(), updated1.id());
            assert_eq!(original1.key(), updated1.key());
            assert_ne!(original1.revision(), updated1.revision());
            assert_eq!(None, updated1.old_content());
            assert_eq!(None, updated1.new_content());
        } else {
            panic!("Expected document header 1, but got: {:?}", updates.get(0));
        }

        if let Ok(ref updated2) = updates.get(1).unwrap() {
            assert_eq!(original2.id(), updated2.id());
            assert_eq!(original2.key(), updated2.key());
            assert_ne!(original2.revision(), updated2.revision());
            assert_eq!(None, updated2.old_content());
            assert_eq!(None, updated2.new_content());
        } else {
            panic!("Expected document header 2, but got: {:?}", updates.get(1));
        }
    });
}

#[test]
fn replace_multiple_struct_documents_with_revision() {
    arango_test_with_document_collection("customers61", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers61", vec![
                NewDocument::from_content(customer1),
                NewDocument::from_content(customer2),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let replacement1 = Customer {
            name: "Nicolas Smith".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-3948294".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 32,
            active: true,
            groups: vec![],
        };

        let replacement2 = Customer {
            name: "Cece Kutrapali".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-1334908".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let method = ReplaceDocuments::<Customer, _>::new("customers61", vec![
            DocumentUpdate::new(original1.key().clone(), replacement1)
                .with_revision(original1.revision().clone()),
            DocumentUpdate::new(original2.key().clone(), replacement2)
                .with_revision(Revision::new("not_existing").to_owned()),
        ]).with_ignore_revisions(false);
        let updates = core.run(conn.execute(method)).unwrap();

        if let Ok(ref updated1) = updates.get(0).unwrap() {
            assert_eq!(original1.id(), updated1.id());
            assert_eq!(original1.key(), updated1.key());
            assert_ne!(original1.revision(), updated1.revision());
            assert_eq!(None, updated1.old_content());
            assert_eq!(None, updated1.new_content());
        } else {
            panic!("Expected document header 1, but got: {:?}", updates.get(0));
        }

        if let Err(error) = updates.get(1).unwrap() {
            assert_eq!(ErrorCode::ArangoConflict, error.code());
            assert_eq!("conflict", error.message());
        } else {
            panic!("Expected error, but got: {:?}", updates.get(1));
        }
    });
}

#[test]
fn delete_document() {
    arango_test_with_document_collection("customers80", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let header = core.run(conn.execute(InsertDocument::new(
            "customers80", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = DeleteDocument::new("customers80", document_key.clone());
        let deleted = core.run(conn.execute(method)).unwrap();

        assert_eq!(&document_id, deleted.id());
        assert_eq!(&document_key, deleted.key());
        assert_eq!(&revision, deleted.revision());
    });
}

#[test]
fn delete_document_return_old() {
    arango_test_with_document_collection("customers81", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let header = core.run(conn.execute(InsertDocument::new(
            "customers81", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = DeleteDocumentReturnOld::with_id(document_id.clone());
        let deleted = core.run(conn.execute(method)).unwrap();

        assert_eq!(&document_id, deleted.id());
        assert_eq!(&document_key, deleted.key());
        assert_eq!(&revision, deleted.revision());
        assert_eq!(&customer, deleted.content());
    });
}

#[test]
fn delete_document_match_revision() {
    arango_test_with_document_collection("customers82", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let header = core.run(conn.execute(InsertDocument::new(
            "customers82", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = DeleteDocument::new("customers82", document_key.clone())
            .with_if_match(revision.as_str().to_owned());
        let deleted = core.run(conn.execute(method)).unwrap();

        assert_eq!(&document_id, deleted.id());
        assert_eq!(&document_key, deleted.key());
        assert_eq!(&revision, deleted.revision());
    });
}

#[test]
fn delete_document_with_not_existing_revision() {
    arango_test_with_document_collection("customers83", |conn, ref mut core| {

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let header = core.run(conn.execute(InsertDocument::new(
            "customers83", NewDocument::from_content(customer)
        ))).unwrap();
        let (_, document_key, _) = header.deconstruct();

        let method = DeleteDocument::new("customers83", document_key)
            .with_if_match("not-existing".to_owned());
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::Method(error)) => {
                assert_eq!(412, error.status_code());
                assert_eq!(ErrorCode::ArangoConflict, error.error_code());
                assert_eq!("precondition failed", error.message());
            },
            _ => panic!("Expected error, but got: {:?}", result)
        }
    });
}

#[test]
fn delete_multiple_documents_by_ids() {
    arango_test_with_document_collection("customers180", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers180", vec![
                NewDocument::from_content(customer1),
                NewDocument::from_content(customer2),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocuments::with_ids("customers180", vec![
            original1.id().clone(),
            original2.id().clone(),
        ]);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Ok(ref deleted) = result_list.get(0).unwrap() {
            assert_eq!(original1.id(), deleted.id());
            assert_eq!(original1.key(), deleted.key());
            assert_eq!(original1.revision(), deleted.revision());
        } else {
            panic!("Expected document header 1, but got: {:?}", result_list.get(0));
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}

#[test]
fn delete_multiple_documents_by_keys() {
    arango_test_with_document_collection("customers181", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers181", vec![
                NewDocument::from_content(customer1),
                NewDocument::from_content(customer2),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocuments::with_keys("customers181", vec![
            original1.key().clone(),
            original2.key().clone(),
        ]);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Ok(ref deleted) = result_list.get(0).unwrap() {
            assert_eq!(original1.id(), deleted.id());
            assert_eq!(original1.key(), deleted.key());
            assert_eq!(original1.revision(), deleted.revision());
        } else {
            panic!("Expected document header 1, but got: {:?}", result_list.get(0));
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}

#[test]
fn delete_multiple_documents_by_headers() {
    arango_test_with_document_collection("customers182", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers182", vec![
                NewDocument::from_content(customer1),
                NewDocument::from_content(customer2),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocuments::with_headers("customers182", vec![
            original1.clone(),
            original2.clone(),
        ]).with_ignore_revisions(false);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Ok(ref deleted) = result_list.get(0).unwrap() {
            assert_eq!(original1.id(), deleted.id());
            assert_eq!(original1.key(), deleted.key());
            assert_eq!(original1.revision(), deleted.revision());
        } else {
            panic!("Expected document header 1, but got: {:?}", result_list.get(0));
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}

#[test]
fn delete_multiple_documents_by_headers_ignore_revisions() {
    arango_test_with_document_collection("customers183", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers183", vec![
                NewDocument::from_content(customer1),
                NewDocument::from_content(customer2),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocuments::with_headers("customers183", vec![
            DocumentHeader::new(
                original1.id().clone(),
                original1.key().clone(),
                Revision::new("not-existing")
            ),
            original2.clone(),
        ]).with_ignore_revisions(true);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Ok(ref deleted) = result_list.get(0).unwrap() {
            assert_eq!(original1.id(), deleted.id());
            assert_eq!(original1.key(), deleted.key());
            assert_eq!(original1.revision(), deleted.revision());
        } else {
            panic!("Expected document header 1, but got: {:?}", result_list.get(0));
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}

#[test]
fn delete_multiple_documents_by_headers_one_not_existing_revision() {
    arango_test_with_document_collection("customers184", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers184", vec![
                NewDocument::from_content(customer1),
                NewDocument::from_content(customer2),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocuments::with_headers("customers184", vec![
            DocumentHeader::new(
                original1.id().clone(),
                original1.key().clone(),
                Revision::new("not-existing")
            ),
            original2.clone(),
        ]).with_ignore_revisions(false);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Err(error) = result_list.get(0).unwrap() {
            assert_eq!(ErrorCode::ArangoConflict, error.code());
            assert_eq!("conflict", error.message());
        } else {
            panic!("Expected error, but got: {:?}", result_list.get(0))
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}

#[test]
fn delete_multiple_documents_by_ids_return_old() {
    arango_test_with_document_collection("customers185", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers185", vec![
                NewDocument::from_content(customer1.clone()),
                NewDocument::from_content(customer2.clone()),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocumentsReturnOld::with_ids("customers185", vec![
            original1.id().clone(),
            original2.id().clone(),
        ]);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Ok(ref deleted) = result_list.get(0).unwrap() {
            assert_eq!(original1.id(), deleted.id());
            assert_eq!(original1.key(), deleted.key());
            assert_eq!(original1.revision(), deleted.revision());
            assert_eq!(&customer1, deleted.content());
        } else {
            panic!("Expected document header 1, but got: {:?}", result_list.get(0));
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
            assert_eq!(&customer2, deleted.content());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}

#[test]
fn delete_multiple_documents_by_keys_return_old() {
    arango_test_with_document_collection("customers186", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers186", vec![
                NewDocument::from_content(customer1.clone()),
                NewDocument::from_content(customer2.clone()),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocumentsReturnOld::with_keys("customers186", vec![
            original1.key().clone(),
            original2.key().clone(),
        ]);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Ok(ref deleted) = result_list.get(0).unwrap() {
            assert_eq!(original1.id(), deleted.id());
            assert_eq!(original1.key(), deleted.key());
            assert_eq!(original1.revision(), deleted.revision());
            assert_eq!(&customer1, deleted.content());
        } else {
            panic!("Expected document header 1, but got: {:?}", result_list.get(0));
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
            assert_eq!(&customer2, deleted.content());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}

#[test]
fn delete_multiple_documents_by_headers_return_old() {
    arango_test_with_document_collection("customers187", |conn, ref mut core| {

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let result_list = core.run(conn.execute(InsertDocuments::new(
            "customers187", vec![
                NewDocument::from_content(customer1.clone()),
                NewDocument::from_content(customer2.clone()),
            ],
        ))).unwrap();
        let original1 = result_list.get(0).unwrap().unwrap();
        let original2 = result_list.get(1).unwrap().unwrap();

        let method = DeleteDocumentsReturnOld::with_headers("customers187", vec![
            original1.clone(),
            original2.clone(),
        ]).with_ignore_revisions(false);
        let result_list = core.run(conn.execute(method)).unwrap();

        if let Ok(ref deleted) = result_list.get(0).unwrap() {
            assert_eq!(original1.id(), deleted.id());
            assert_eq!(original1.key(), deleted.key());
            assert_eq!(original1.revision(), deleted.revision());
            assert_eq!(&customer1, deleted.content());
        } else {
            panic!("Expected document header 1, but got: {:?}", result_list.get(0));
        }
        if let Ok(ref deleted) = result_list.get(1).unwrap() {
            assert_eq!(original2.id(), deleted.id());
            assert_eq!(original2.key(), deleted.key());
            assert_eq!(original2.revision(), deleted.revision());
            assert_eq!(&customer2, deleted.content());
        } else {
            panic!("Expected document header 2, but got: {:?}", result_list.get(1));
        }
    });
}
