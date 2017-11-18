
use serde_json;
use super::types::*;

#[test]
fn get_index_key_from_str() {
    let index_key = IndexKey::from_str("12341").unwrap();
    assert_eq!("12341", index_key.as_str());
}

#[test]
fn get_index_key_from_str_with_slash_character_in_the_middle() {
    let result = IndexKey::from_str("mine/12341");
    assert_eq!(Err("An index key must not contain any '/' character, but got: \"mine/12341\"".to_owned()), result);
}

#[test]
fn get_index_key_from_str_with_slash_character_at_the_beginning() {
    let result = IndexKey::from_str("/12341");
    assert_eq!(Err("An index key must not contain any '/' character, but got: \"/12341\"".to_owned()), result);
}

#[test]
fn get_index_key_from_str_with_slash_character_at_the_end() {
    let result = IndexKey::from_str("12341/");
    assert_eq!(Err("An index key must not contain any '/' character, but got: \"12341/\"".to_owned()), result);
}

#[test]
fn get_index_id_from_str() {
    let index_id = IndexId::from_str("mine/12341").unwrap();
    assert_eq!("mine", index_id.collection_name());
    assert_eq!("12341", index_id.index_key());
    assert_eq!("mine/12341", &index_id.to_string());
}

#[test]
fn get_index_id_from_str_without_collection_name() {
    let result = IndexId::from_str("12341");
    assert_eq!(Err("index id does not have a context: \"12341\"".to_owned()), result);
}

#[test]
fn get_index_id_from_str_with_empty_collection_name() {
    let result = IndexId::from_str("/12341");
    assert_eq!(Err("Invalid index id: \"/12341\"".to_owned()), result);
}

#[test]
fn get_index_id_from_str_with_empty_index_key() {
    let result = IndexId::from_str("mine/");
    assert_eq!(Err("Invalid index id: \"mine/\"".to_owned()), result);
}

#[test]
fn get_index_id_option_from_str_with_collection_name_and_index_key() {
    let index_id_option = IndexIdOption::from_str("mine/12341").unwrap();
    assert_eq!(IndexIdOption::from(IndexId::new("mine", "12341")), index_id_option);
}

#[test]
fn get_index_id_option_from_str_with_index_key_only() {
    let index_id_option = IndexIdOption::from_str("12341").unwrap();
    assert_eq!(IndexIdOption::from(IndexKey::new("12341")), index_id_option);
}

#[test]
fn deserialize_primary_index() {
    let index_json = r#"{
            "fields" : [
                "_key"
            ],
            "id" : "products/0",
            "selectivityEstimate" : 1,
            "sparse" : false,
            "type" : "primary",
            "unique" : true
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::Primary(ref primary_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("0", index_id.index_key());
        assert_eq!(&vec!("_key".to_owned())[..], primary_index.fields());
        assert_eq!(false, primary_index.is_newly_created());
        assert_eq!(1, primary_index.selectivity_estimate());
        assert_eq!(true, primary_index.is_unique());
    } else {
        panic!("Primary index expected, but got {:?}", index);
    }
}

#[test]
fn deserialize_hash_index() {
    let index_json = r#"{
            "deduplicate" : true,
            "fields" : [
                "a"
            ],
            "id" : "products/11582",
            "isNewlyCreated" : true,
            "selectivityEstimate" : 1,
            "sparse" : true,
            "type" : "hash",
            "unique" : false,
            "error" : false,
            "code" : 201
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::Hash(ref hash_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("11582", index_id.index_key());
        assert_eq!(&vec!("a".to_owned())[..], hash_index.fields());
        assert_eq!(true, hash_index.is_newly_created());
        assert_eq!(true, hash_index.is_deduplicate());
        assert_eq!(1, hash_index.selectivity_estimate());
        assert_eq!(true, hash_index.is_sparse());
        assert_eq!(false, hash_index.is_unique());
    } else {
        panic!("Hash index expected, but got {:?}", index);
    }
}

#[test]
fn deserialize_skip_list_index() {
    let index_json = r#"{
            "deduplicate" : true,
            "fields" : [
                "a",
                "b"
            ],
            "id" : "products/11556",
            "isNewlyCreated" : false,
            "sparse" : false,
            "type" : "skiplist",
            "unique" : false
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::SkipList(ref skip_list_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("11556", index_id.index_key());
        assert_eq!(&vec!("a".to_owned(), "b".to_owned())[..], skip_list_index.fields());
        assert_eq!(false, skip_list_index.is_newly_created());
        assert_eq!(true, skip_list_index.is_deduplicate());
        assert_eq!(false, skip_list_index.is_sparse());
        assert_eq!(false, skip_list_index.is_unique());
    } else {
        panic!("SkipList index expected, but got {:?}", index);
    }
}

#[test]
fn deserialize_persistent_index() {
    let index_json = r#"{
            "deduplicate" : false,
            "fields" : [
                "a",
                "b"
            ],
            "id" : "products/11595",
            "isNewlyCreated" : true,
            "sparse" : true,
            "type" : "persistent",
            "unique" : true
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::Persistent(ref persistent_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("11595", index_id.index_key());
        assert_eq!(&vec!("a".to_owned(), "b".to_owned())[..], persistent_index.fields());
        assert_eq!(true, persistent_index.is_newly_created());
        assert_eq!(false, persistent_index.is_deduplicate());
        assert_eq!(true, persistent_index.is_sparse());
        assert_eq!(true, persistent_index.is_unique());
    } else {
        panic!("Persistent index expected, but got {:?}", index);
    }
}

#[test]
fn deserialize_geo1_index() {
    let index_json = r#"{
            "constraint" : false,
            "fields" : [
                "b"
            ],
            "geoJson" : true,
            "id" : "products/11504",
            "ignoreNull" : true,
            "isNewlyCreated" : true,
            "sparse" : true,
            "type" : "geo1",
            "unique" : false
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::Geo1(ref geo1_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("11504", index_id.index_key());
        assert_eq!(&vec!("b".to_owned())[..], geo1_index.fields());
        assert_eq!(true, geo1_index.is_newly_created());
        assert_eq!(true, geo1_index.is_geo_json());
        assert_eq!(false, geo1_index.is_constraint());
        assert_eq!(true, geo1_index.is_sparse());
    } else {
        panic!("Geo1 index expected, but got {:?}", index);
    }
}

#[test]
fn deserialize_geo2_index() {
    let index_json = r#"{
            "constraint" : true,
            "fields" : [
                "e",
                "f"
            ],
            "id" : "products/11491",
            "ignoreNull" : true,
            "isNewlyCreated" : true,
            "sparse" : true,
            "type" : "geo2",
            "unique" : false
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::Geo2(ref geo2_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("11491", index_id.index_key());
        assert_eq!(&vec!("e".to_owned(), "f".to_owned())[..], geo2_index.fields());
        assert_eq!(true, geo2_index.is_newly_created());
        assert_eq!(true, geo2_index.is_constraint());
        assert_eq!(true, geo2_index.is_sparse());
    } else {
        panic!("Geo2 index expected, but got {:?}", index);
    }
}

#[test]
fn deserialize_fulltext_index() {
    let index_json = r#" {
            "fields" : [
                "description"
            ],
            "id" : "products/11476",
            "minLength": 2,
            "sparse" : false,
            "type" : "fulltext",
            "unique" : false
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::Fulltext(ref fulltext_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("11476", index_id.index_key());
        assert_eq!(&vec!("description".to_owned())[..], fulltext_index.fields());
        assert_eq!(false, fulltext_index.is_newly_created());
        assert_eq!(2, fulltext_index.min_length());
    } else {
        panic!("Fulltext index expected, but got {:?}", index);
    }
}

#[test]
fn deserialize_edge_index() {
    let index_json = r#" {
            "fields" : [
                "_from",
                "_to"
            ],
            "id" : "products/2834226",
            "sparse" : false,
            "type" : "edge",
            "unique" : false
        }"#;

    let index: Index = serde_json::from_str(index_json).unwrap();
    let index_id = match *index.id() {
        IndexIdOption::Qualified(ref index_id) => index_id,
        _ => panic!("Qualified index id expected!"),
    };

    if let Index::Edge(ref edge_index) = index {
        assert_eq!("products", index_id.collection_name());
        assert_eq!("2834226", index_id.index_key());
        assert_eq!(&vec!("_from".to_owned(), "_to".to_owned())[..], edge_index.fields());
        assert_eq!(false, edge_index.is_newly_created());
    } else {
        panic!("Edge index expected, but got {:?}", index);
    }
}
