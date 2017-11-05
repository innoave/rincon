
extern crate dotenv;
extern crate futures;
extern crate log4rs;
#[macro_use] extern crate serde_derive;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::collection::CreateCollection;
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
