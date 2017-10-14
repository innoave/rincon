
use std::collections::HashMap;

use serde::de::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
pub struct IndexList {
    indexes: Vec<Index>,
    identifiers: HashMap<String, Index>,
}

impl IndexList {
    pub fn indexes(&self) -> &[Index] {
        &self.indexes
    }

    pub fn identifiers(&self) -> &HashMap<String, Index> {
        &self.identifiers
    }
}

pub trait IndexInfo {
    fn id(&self) -> &str;

    fn fields(&self) -> &[String];

    fn is_newly_created(&self) -> bool;

    fn is_sparse(&self) -> bool;

    fn is_unique(&self) -> bool;
}

#[derive(Clone, Debug)]
pub enum Index {
    Primary(PrimaryIndex),
    Hash(HashIndex),
    SkipList(SkipListIndex),
    Persistent(PersistentIndex),
    Geo1(Geo1Index),
    Geo2(Geo2Index),
    Fulltext(FulltextIndex),
    Edge(EdgeIndex),
}

impl Index {
    fn unwrap_index_info(&self) -> &IndexInfo {
        use self::Index::*;
        match *self {
            Primary(ref info) => info,
            Hash(ref info) => info,
            SkipList(ref info) => info,
            Persistent(ref info) => info,
            Geo1(ref info) => info,
            Geo2(ref info) => info,
            Fulltext(ref info) => info,
            Edge(ref info) => info,
        }
    }
}

impl IndexInfo for Index {
    fn id(&self) -> &str {
        self.unwrap_index_info().id()
    }

    fn fields(&self) -> &[String] {
        self.unwrap_index_info().fields()
    }

    fn is_newly_created(&self) -> bool {
        self.unwrap_index_info().is_newly_created()
    }

    fn is_sparse(&self) -> bool {
        self.unwrap_index_info().is_sparse()
    }

    fn is_unique(&self) -> bool {
        self.unwrap_index_info().is_unique()
    }
}

#[derive(Clone, Debug)]
pub struct PrimaryIndex {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
    selectivity_estimate: u32,
    unique: bool,
}

impl PrimaryIndex {
    pub fn selectivity_estimate(&self) -> u32 {
        self.selectivity_estimate
    }
}

impl IndexInfo for PrimaryIndex {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        false
    }

    fn is_unique(&self) -> bool {
        self.unique
    }
}

#[derive(Clone, Debug)]
pub struct HashIndex {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
    deduplicate: bool,
    selectivity_estimate: u32,
    sparse: bool,
    unique: bool,
}

impl HashIndex {
    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }

    pub fn selectivity_estimate(&self) -> u32 {
        self.selectivity_estimate
    }
}

impl IndexInfo for HashIndex {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }

    fn is_unique(&self) -> bool {
        self.unique
    }
}

#[derive(Clone, Debug)]
pub struct SkipListIndex {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
    deduplicate: bool,
    sparse: bool,
    unique: bool,
}

impl SkipListIndex {
    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }
}

impl IndexInfo for SkipListIndex {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }

    fn is_unique(&self) -> bool {
        self.unique
    }
}

#[derive(Clone, Debug)]
pub struct PersistentIndex {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
    deduplicate: bool,
    sparse: bool,
    unique: bool,
}

impl PersistentIndex {
    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }
}

impl IndexInfo for PersistentIndex {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }

    fn is_unique(&self) -> bool {
        self.unique
    }
}

#[derive(Clone, Debug)]
pub struct Geo1Index {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
    geo_json: bool,
    constraint: bool,
    sparse: bool,
}

impl Geo1Index {
    pub fn is_geo_json(&self) -> bool {
        self.geo_json
    }

    pub fn is_constraint(&self) -> bool {
        self.constraint
    }
}

impl IndexInfo for Geo1Index {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct Geo2Index {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
    constraint: bool,
    sparse: bool,
}

impl Geo2Index {
    pub fn is_constraint(&self) -> bool {
        self.constraint
    }
}

impl IndexInfo for Geo2Index {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct FulltextIndex {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
    min_length: u32,
}

impl FulltextIndex {
    pub fn min_length(&self) -> u32 {
        self.min_length
    }
}

impl IndexInfo for FulltextIndex {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        false
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct EdgeIndex {
    id: String,
    fields: Vec<String>,
    newly_created: bool,
}

impl IndexInfo for EdgeIndex {
    fn id(&self) -> &str {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_sparse(&self) -> bool {
        false
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenericIndex {
    #[serde(rename = "type")]
    kind: String,
    id: String,
    fields: Vec<String>,
    selectivity_estimate: Option<u32>,
    is_newly_created: Option<bool>,
    min_length: Option<u32>,
    sparse: Option<bool>,
    unique: Option<bool>,
    constraint: Option<bool>,
    deduplicate: Option<bool>,
    geo_json: Option<bool>,
}

impl<'de> Deserialize<'de> for Index {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let GenericIndex {
            kind,
            id,
            fields,
            selectivity_estimate,
            is_newly_created,
            min_length,
            sparse,
            unique,
            constraint,
            deduplicate,
            geo_json,
        } = GenericIndex::deserialize(deserializer)?;
        match &kind[..] {
            "primary" => match (selectivity_estimate, unique) {
                (Some(selectivity_estimate), Some(unique)) =>
                    Ok(Index::Primary(PrimaryIndex {
                        id,
                        fields,
                        newly_created: is_newly_created.unwrap_or(false),
                        selectivity_estimate,
                        unique,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            }
            "hash" => match (deduplicate, selectivity_estimate, sparse, unique) {
                (Some(deduplicate), Some(selectivity_estimate), Some(sparse), Some(unique)) =>
                    Ok(Index::Hash(HashIndex {
                        id,
                        fields,
                        newly_created: is_newly_created.unwrap_or(false),
                        deduplicate,
                        selectivity_estimate,
                        sparse,
                        unique,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            "skiplist" =>match (deduplicate, sparse, unique) {
                (Some(deduplicate), Some(sparse), Some(unique)) =>
                    Ok(Index::SkipList(SkipListIndex {
                        id,
                        fields,
                        newly_created: is_newly_created.unwrap_or(false),
                        deduplicate,
                        sparse,
                        unique,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            "persistent" => match (deduplicate, sparse, unique) {
                (Some(deduplicate), Some(sparse), Some(unique)) =>
                    Ok(Index::Persistent(PersistentIndex {
                        id,
                        fields,
                        deduplicate,
                        newly_created: is_newly_created.unwrap_or(false),
                        sparse,
                        unique,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            "geo1" => match (geo_json, constraint, sparse) {
                (Some(geo_json), Some(constraint), Some(sparse)) =>
                    Ok(Index::Geo1(Geo1Index {
                        id,
                        fields,
                        newly_created: is_newly_created.unwrap_or(false),
                        geo_json,
                        constraint,
                        sparse,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            "geo2" => match (constraint, sparse) {
                (Some(constraint), Some(sparse)) =>
                    Ok(Index::Geo2(Geo2Index {
                        id,
                        fields,
                        newly_created: is_newly_created.unwrap_or(false),
                        constraint,
                        sparse,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            "fulltext" => match min_length {
                Some(min_length) =>
                    Ok(Index::Fulltext(FulltextIndex {
                        id,
                        fields,
                        newly_created: is_newly_created.unwrap_or(false),
                        min_length,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            "edge" =>
                Ok(Index::Edge(EdgeIndex {
                    id,
                    fields,
                    newly_created: is_newly_created.unwrap_or(false),
                })),
            _ => Err(D::Error::custom("Unsupported index type")),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::Primary(primary_index) = index {
            assert_eq!("products/0", primary_index.id());
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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::Hash(hash_index) = index {
            assert_eq!("products/11582", hash_index.id());
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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::SkipList(skip_list_index) = index {
            assert_eq!("products/11556", skip_list_index.id());
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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::Persistent(persistent_index) = index {
            assert_eq!("products/11595", persistent_index.id());
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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::Geo1(geo1_index) = index {
            assert_eq!("products/11504", geo1_index.id());
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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::Geo2(geo2_index) = index {
            assert_eq!("products/11491", geo2_index.id());
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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::Fulltext(fulltext_index) = index {
            assert_eq!("products/11476", fulltext_index.id());
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

        let index = serde_json::from_str(index_json).unwrap();

        if let Index::Edge(edge_index) = index {
            assert_eq!("products/2834226", edge_index.id());
            assert_eq!(&vec!("_from".to_owned(), "_to".to_owned())[..], edge_index.fields());
            assert_eq!(false, edge_index.is_newly_created());
        } else {
            panic!("Edge index expected, but got {:?}", index);
        }
    }
}
