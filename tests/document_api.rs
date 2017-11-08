
extern crate dotenv;
extern crate futures;
extern crate log4rs;
#[macro_use] extern crate serde_derive;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::method::ErrorCode;
use arangodb_client::api::types::JsonString;
use arangodb_client::collection::CreateCollection;
use arangodb_client::connection::Error;
use arangodb_client::document::*;

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

#[test]
fn insert_struct_document_without_key() {
    arango_user_db_test("test_document_user1", "test_document_db11", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocument::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
    });
}

#[test]
fn insert_struct_document_without_key_and_return_new() {
    arango_user_db_test("test_document_user2", "test_document_db21", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocumentReturnNew::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn insert_struct_document_with_key() {
    arango_user_db_test("test_document_user3", "test_document_db31", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocument::new("customers", new_document)
            .with_force_wait_for_sync(true);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers/94711", &document.id().as_string());
        assert_eq!("customers", document.id().collection_name());
        assert_eq!("94711", document.id().document_key());
        assert_eq!("94711", document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
    });
}

#[test]
fn insert_struct_document_with_key_and_return_new() {
    arango_user_db_test("test_document_user4", "test_document_db41", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocumentReturnNew::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers/94712", &document.id().as_string());
        assert_eq!("customers", document.id().collection_name());
        assert_eq!("94712", document.id().document_key());
        assert_eq!("94712", document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn insert_json_document_with_key_and_return_new() {
    arango_user_db_test("test_document_user5", "test_document_db51", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocumentReturnNew::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert!(document.content().as_str().starts_with(r#"{"_id":"customers/7713996","_key":"7713996","_rev":""#));
        assert!(document.content().as_str().ends_with(r#"","active":true,"age":42,"contact":[{"address":"1-555-234523","kind":"Phone","tag":"work"}],"gender":"Female","groups":[],"name":"Jane Doe"}"#));
    });
}

#[test]
fn insert_multiple_struct_documents_without_key() {
    arango_user_db_test("test_document_user6", "test_document_db61", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocuments::new("customers", vec![new_document1, new_document2])
            .with_force_wait_for_sync(true);
        let documents = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", documents[0].id().collection_name());
        assert!(!documents[0].id().document_key().is_empty());
        assert_eq!(documents[0].id().document_key(), documents[0].key().as_str());
        assert!(!documents[0].revision().as_str().is_empty());

        assert_eq!("customers", documents[1].id().collection_name());
        assert!(!documents[1].id().document_key().is_empty());
        assert_eq!(documents[1].id().document_key(), documents[1].key().as_str());
        assert!(!documents[1].revision().as_str().is_empty());
    });
}

#[test]
fn insert_multiple_struct_documents_without_key_and_return_new() {
    arango_user_db_test("test_document_user7", "test_document_db71", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocumentsReturnNew::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", documents[0].id().collection_name());
        assert!(!documents[0].id().document_key().is_empty());
        assert_eq!(documents[0].id().document_key(), documents[0].key().as_str());
        assert!(!documents[0].revision().as_str().is_empty());
        assert_eq!(&customer1, documents[0].content());

        assert_eq!("customers", documents[1].id().collection_name());
        assert!(!documents[1].id().document_key().is_empty());
        assert_eq!(documents[1].id().document_key(), documents[1].key().as_str());
        assert!(!documents[1].revision().as_str().is_empty());
        assert_eq!(&customer2, documents[1].content());
    });
}

#[test]
fn insert_multiple_struct_documents_with_key() {
    arango_user_db_test("test_document_user8", "test_document_db81", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocuments::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers/94711", &documents[0].id().as_string());
        assert_eq!("customers", documents[0].id().collection_name());
        assert_eq!("94711", documents[0].id().document_key());
        assert_eq!("94711", documents[0].key().as_str());
        assert!(!documents[0].revision().as_str().is_empty());

        assert_eq!("customers/90815", &documents[1].id().as_string());
        assert_eq!("customers", documents[1].id().collection_name());
        assert_eq!("90815", documents[1].id().document_key());
        assert_eq!("90815", documents[1].key().as_str());
        assert!(!documents[1].revision().as_str().is_empty());
    });
}

#[test]
fn insert_multiple_struct_documents_with_key_and_return_new() {
    arango_user_db_test("test_document_user9", "test_document_db91", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
        let method = InsertDocumentsReturnNew::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers/94712", &documents[0].id().as_string());
        assert_eq!("customers", documents[0].id().collection_name());
        assert_eq!("94712", documents[0].id().document_key());
        assert_eq!("94712", documents[0].key().as_str());
        assert!(!documents[0].revision().as_str().is_empty());
        assert_eq!(&customer1, documents[0].content());

        assert_eq!("customers/90815", &documents[1].id().as_string());
        assert_eq!("customers", documents[1].id().collection_name());
        assert_eq!("90815", documents[1].id().document_key());
        assert_eq!("90815", documents[1].key().as_str());
        assert!(!documents[1].revision().as_str().is_empty());
        assert_eq!(&customer2, documents[1].content());
    });
}

#[test]
fn get_document_as_struct_inserted_as_struct() {
    arango_user_db_test("test_document_user10", "test_document_db101", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_as_struct_inserted_as_json_string() {
    arango_user_db_test("test_document_user11", "test_document_db111", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(JsonString::new(json_doc))
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_as_json_string_inserted_as_struct() {
    arango_user_db_test("test_document_user12", "test_document_db121", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(customer.clone())
                .with_key(DocumentKey::new("7713996"))
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone());
        let document: Document<JsonString> = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        let expected = r#"{"active":true,"age":42,"contact":[{"address":"1-555-234523","kind":"Phone","tag":"work"}],"gender":"Female","groups":[],"name":"Jane Doe"}"#;
        assert_eq!(expected, document.content().as_str());
    });
}

#[test]
fn get_document_if_revision_matches() {
    arango_user_db_test("test_document_user13", "test_document_db131", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone())
            .with_if_match(revision.as_str().to_owned());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_if_revision_is_not_a_match() {
    arango_user_db_test("test_document_user14", "test_document_db141", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone())
            .with_if_non_match(String::from("not") + revision.as_str());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_but_revision_does_not_match() {
    arango_user_db_test("test_document_user15", "test_document_db151", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, _, revision) = header.deconstruct();

        let method = GetDocument::<Customer>::new(document_id)
            .with_if_match(String::from("not") + revision.as_str());
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
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
    arango_user_db_test("test_document_user16", "test_document_db161", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (_, document_key, _) = header.deconstruct();

        let method = GetDocument::<Customer>::new(DocumentId::new("customers", "notexisting999"));
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoDocumentNotFound, error.error_code());
                assert_eq!("document not found", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }

        let method = GetDocument::<Customer>::new(DocumentId::new("notexisting99", document_key.as_str()));
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoCollectionNotFound, error.error_code());
                assert_eq!("collection not found: notexisting99", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[ignore] //TODO refactor get document header to document exists (with possibly returning the revision)
#[test]
fn get_document_header() {
    arango_user_db_test("test_document_user20", "test_document_db201", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

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
            "customers", NewDocument::from_content(customer.clone())
                .with_key(DocumentKey::new("7721264"))
        ))).unwrap();

        let method = GetDocumentHeader::new(inserted.id().clone());
        let result = core.run(conn.execute(method)).unwrap();

        assert_eq!((), result);
    });
}
