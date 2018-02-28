
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use rincon_core::arango::protocol::{Handle, HandleOption};

const INDEX_TYPE_PRIMARY: &str = "primary";
const INDEX_TYPE_HASH: &str = "hash";
const INDEX_TYPE_SKIP_LIST: &str = "skiplist";
const INDEX_TYPE_PERSISTENT: &str = "persistent";
const INDEX_TYPE_GEO: &str = "geo";
const INDEX_TYPE_GEO1: &str = "geo1";
const INDEX_TYPE_GEO2: &str = "geo2";
const INDEX_TYPE_FULLTEXT: &str = "fulltext";
const INDEX_TYPE_EDGE: &str = "edge";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexIdOption {
    Qualified(IndexId),
    Local(IndexKey),
}

impl FromStr for IndexIdOption {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, String> {
        let handle_option = HandleOption::from_str("index id", value)?;
        Ok(match handle_option {
            HandleOption::Qualified(handle) => {
                let (collection_name, index_key) = handle.unwrap();
                IndexIdOption::Qualified(IndexId {
                    collection_name,
                    index_key,
                })
            },
            HandleOption::Local(handle_key) => {
                let value = handle_key.unwrap();
                IndexIdOption::Local(IndexKey(value))
            },
        })
    }
}

impl Serialize for IndexIdOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::IndexIdOption::*;
        match *self {
            Qualified(ref index_id) => index_id.serialize(serializer),
            Local(ref index_key) => index_key.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for IndexIdOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        IndexIdOption::from_str(&value).map_err(D::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexId {
    collection_name: String,
    index_key: String,
}

impl IndexId {
    pub fn new<C, K>(collection_name: C, index_key: K) -> Self
        where C: Into<String>, K: Into<String>
    {
        let collection_name = collection_name.into();
        assert!(!collection_name.contains('/'), "A collection name must not contain any '/' character");
        let index_key = index_key.into();
        assert!(!index_key.contains('/'), "An index key must not contain any '/' character");
        IndexId {
            collection_name,
            index_key,
        }
    }

    pub fn unwrap(self) -> (String, String) {
        (self.collection_name, self.index_key)
    }

    pub fn into_string(self) -> String {
        self.collection_name + "/" + &self.index_key
    }

    pub fn to_string(&self) -> String {
        self.collection_name.to_owned() + "/" + &self.index_key
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn index_key(&self) -> &str {
        &self.index_key
    }
}

impl FromStr for IndexId {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, String> {
        let handle = Handle::from_str("index id", value)?;
        let (collection_name, index_key) = handle.unwrap();
        Ok(IndexId {
            collection_name,
            index_key,
        })
    }
}

impl From<IndexId> for IndexIdOption {
    fn from(index_id: IndexId) -> Self {
        IndexIdOption::Qualified(index_id)
    }
}

impl Serialize for IndexId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for IndexId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        IndexId::from_str(&value).map_err(D::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexKey(String);

impl IndexKey {
    pub fn new<K>(index_key: K) -> Self
        where K: Into<String>
    {
        let index_key = index_key.into();
        assert!(!index_key.contains('/'), "An index key must not contain any '/' character, but got: {:?}", &index_key);
        IndexKey(index_key)
    }

    pub fn from_string(value: String) -> Result<Self, String> {
        if value.contains('/') {
            Err(format!("An index key must not contain any '/' character, but got: {:?}", &value))
        } else {
            Ok(IndexKey(value))
        }
    }

    pub fn unwrap(self) -> String {
        self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for IndexKey {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, String> {
        IndexKey::from_string(value.to_owned())
    }
}

impl From<IndexId> for IndexKey {
    fn from(index_id: IndexId) -> Self {
        IndexKey(index_id.index_key)
    }
}

impl From<IndexKey> for IndexIdOption {
    fn from(index_key: IndexKey) -> Self {
        IndexIdOption::Local(index_key)
    }
}

impl Serialize for IndexKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for IndexKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        IndexKey::from_string(value).map_err(D::Error::custom)
    }
}

#[derive(Debug, Clone, Deserialize)]
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

pub trait IndexDetails {
    fn id(&self) -> &IndexIdOption;

    fn fields(&self) -> &[String];

    fn is_newly_created(&self) -> bool;

    fn is_unique(&self) -> bool;

    fn is_sparse(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
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
    fn unwrap_details(&self) -> &IndexDetails {
        use self::Index::*;
        match *self {
            Primary(ref details) => details,
            Hash(ref details) => details,
            SkipList(ref details) => details,
            Persistent(ref details) => details,
            Geo1(ref details) => details,
            Geo2(ref details) => details,
            Fulltext(ref details) => details,
            Edge(ref details) => details,
        }
    }
}

impl IndexDetails for Index {
    fn id(&self) -> &IndexIdOption {
        self.unwrap_details().id()
    }

    fn fields(&self) -> &[String] {
        self.unwrap_details().fields()
    }

    fn is_newly_created(&self) -> bool {
        self.unwrap_details().is_newly_created()
    }

    fn is_unique(&self) -> bool {
        self.unwrap_details().is_unique()
    }

    fn is_sparse(&self) -> bool {
        self.unwrap_details().is_sparse()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimaryIndex {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    selectivity_estimate: u32,
}

impl PrimaryIndex {
    pub fn selectivity_estimate(&self) -> u32 {
        self.selectivity_estimate
    }
}

impl IndexDetails for PrimaryIndex {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<PrimaryIndex> for Index {
    fn from(index: PrimaryIndex) -> Self {
        Index::Primary(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashIndex {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    deduplicate: bool,
    selectivity_estimate: u32,
}

impl HashIndex {
    pub fn new<Flds, Fld>(
        id: IndexIdOption,
        fields: Flds,
        unique: bool,
        sparse: bool,
        deduplicate: bool,
        selectivity_estimate: u32,
    ) -> Self
        where
            Flds: IntoIterator<Item=Fld>,
            Fld: Into<String>,
    {
        HashIndex {
            newly_created: false,
            id,
            fields: Vec::from_iter(fields.into_iter().map(|f| f.into())),
            unique,
            sparse,
            deduplicate,
            selectivity_estimate,
        }
    }

    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }

    pub fn selectivity_estimate(&self) -> u32 {
        self.selectivity_estimate
    }
}

impl IndexDetails for HashIndex {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<HashIndex> for Index {
    fn from(index: HashIndex) -> Self {
        Index::Hash(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkipListIndex {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    deduplicate: bool,
}

impl SkipListIndex {
    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }
}

impl IndexDetails for SkipListIndex {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<SkipListIndex> for Index {
    fn from(index: SkipListIndex) -> Self {
        Index::SkipList(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PersistentIndex {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    deduplicate: bool,
}

impl PersistentIndex {
    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }
}

impl IndexDetails for PersistentIndex {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<PersistentIndex> for Index {
    fn from(index: PersistentIndex) -> Self {
        Index::Persistent(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Geo1Index {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    constraint: bool,
    geo_json: bool,
}

impl Geo1Index {
    pub fn is_constraint(&self) -> bool {
        self.constraint
    }

    pub fn is_geo_json(&self) -> bool {
        self.geo_json
    }
}

impl IndexDetails for Geo1Index {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<Geo1Index> for Index {
    fn from(index: Geo1Index) -> Self {
        Index::Geo1(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Geo2Index {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    constraint: bool,
}

impl Geo2Index {
    pub fn is_constraint(&self) -> bool {
        self.constraint
    }
}

impl IndexDetails for Geo2Index {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<Geo2Index> for Index {
    fn from(index: Geo2Index) -> Self {
        Index::Geo2(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FulltextIndex {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    min_length: u32,
}

impl FulltextIndex {
    pub fn min_length(&self) -> u32 {
        self.min_length
    }
}

impl IndexDetails for FulltextIndex {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<FulltextIndex> for Index {
    fn from(index: FulltextIndex) -> Self {
        Index::Fulltext(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeIndex {
    newly_created: bool,
    id: IndexIdOption,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
}

impl IndexDetails for EdgeIndex {
    fn id(&self) -> &IndexIdOption {
        &self.id
    }

    fn fields(&self) -> &[String] {
        &self.fields
    }

    fn is_newly_created(&self) -> bool {
        self.newly_created
    }

    fn is_unique(&self) -> bool {
        self.unique
    }

    fn is_sparse(&self) -> bool {
        self.sparse
    }
}

impl From<EdgeIndex> for Index {
    fn from(index: EdgeIndex) -> Self {
        Index::Edge(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NewIndex {
    Hash(NewHashIndex),
    SkipList(NewSkipListIndex),
    Persistent(NewPersistentIndex),
    Geo(NewGeoIndex),
    Fulltext(NewFulltextIndex),
}

impl Serialize for NewIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::NewIndex::*;
        match *self {
            Hash(ref index) => index.serialize(serializer),
            SkipList(ref index) => index.serialize(serializer),
            Persistent(ref index) => index.serialize(serializer),
            Geo(ref index) => index.serialize(serializer),
            Fulltext(ref index) => index.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewHashIndex {
    #[serde(rename = "type")]
    kind: IndexType,
    fields: Vec<String>,
    unique: bool,
    sparse: bool,
    deduplicate: bool,
}

impl NewHashIndex {
    pub fn new<F>(fields: F, unique: bool, sparse: bool, deduplicate: bool) -> Self
        where F: IntoIterator<Item=String>
    {
        NewHashIndex {
            kind: IndexType::Hash,
            fields: Vec::from_iter(fields.into_iter()),
            sparse,
            unique,
            deduplicate,
        }
    }

    pub fn fields_mut(&mut self) -> &mut Vec<String> {
        &mut self.fields
    }

    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    pub fn set_sparse(&mut self, sparse: bool) {
        self.sparse = sparse;
    }

    pub fn is_sparse(&self) -> bool {
        self.sparse
    }

    pub fn set_unique(&mut self, unique: bool) {
        self.unique = unique;
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }

    pub fn set_deduplicate(&mut self, deduplicate: bool) {
        self.deduplicate = deduplicate;
    }

    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }
}

impl From<NewHashIndex> for NewIndex {
    fn from(index: NewHashIndex) -> Self {
        NewIndex::Hash(index)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSkipListIndex {
    #[serde(rename = "type")]
    kind: IndexType,
    fields: Vec<String>,
    sparse: bool,
    unique: bool,
    deduplicate: bool,
}

impl NewSkipListIndex {
    pub fn new<F>(fields: F, unique: bool, sparse: bool, deduplicate: bool) -> Self
        where F: IntoIterator<Item=String>
    {
        NewSkipListIndex {
            kind: IndexType::SkipList,
            fields: Vec::from_iter(fields.into_iter()),
            sparse,
            unique,
            deduplicate,
        }
    }

    pub fn fields_mut(&mut self) -> &mut Vec<String> {
        &mut self.fields
    }

    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    pub fn set_sparse(&mut self, sparse: bool) {
        self.sparse = sparse;
    }

    pub fn is_sparse(&self) -> bool {
        self.sparse
    }

    pub fn set_unique(&mut self, unique: bool) {
        self.unique = unique;
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }

    pub fn set_deduplicate(&mut self, deduplicate: bool) {
        self.deduplicate = deduplicate;
    }

    pub fn is_deduplicate(&self) -> bool {
        self.deduplicate
    }
}

impl From<NewSkipListIndex> for NewIndex {
    fn from(index: NewSkipListIndex) -> Self {
        NewIndex::SkipList(index)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPersistentIndex {
    #[serde(rename = "type")]
    kind: IndexType,
    fields: Vec<String>,
    sparse: bool,
    unique: bool,
}

impl NewPersistentIndex {
    pub fn new<F>(fields: F, unique: bool, sparse: bool) -> Self
        where F: IntoIterator<Item=String>
    {
        NewPersistentIndex {
            kind: IndexType::Persistent,
            fields: Vec::from_iter(fields.into_iter()),
            sparse,
            unique,
        }
    }

    pub fn fields_mut(&mut self) -> &mut Vec<String> {
        &mut self.fields
    }

    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    pub fn set_sparse(&mut self, sparse: bool) {
        self.sparse = sparse;
    }

    pub fn is_sparse(&self) -> bool {
        self.sparse
    }

    pub fn set_unique(&mut self, unique: bool) {
        self.unique = unique;
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }
}

impl From<NewPersistentIndex> for NewIndex {
    fn from(index: NewPersistentIndex) -> Self {
        NewIndex::Persistent(index)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGeoIndex {
    #[serde(rename = "type")]
    kind: IndexType,
    fields: Vec<String>,
    geo_json: bool,
}

impl NewGeoIndex {
    pub fn new<F>(fields: F, geo_json: bool) -> Self
        where F: IntoIterator<Item=String>
    {
        NewGeoIndex {
            kind: IndexType::Geo,
            fields: Vec::from_iter(fields.into_iter()),
            geo_json,
        }
    }

    pub fn with_location_field<L>(location_field: L, geo_json: bool) -> Self
        where L: Into<String>
    {
        NewGeoIndex::new(vec![location_field.into()], geo_json)
    }

    pub fn with_lat_lng_fields<LAT, LNG>(lat_field: LAT, lng_field: LNG) -> Self
        where LAT: Into<String>, LNG: Into<String>
    {
        NewGeoIndex::new(vec![lat_field.into(), lng_field.into()], false)
    }

    pub fn fields_mut(&mut self) -> &mut Vec<String> {
        &mut self.fields
    }

    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    pub fn set_geo_json(&mut self, geo_json: bool) {
        self.geo_json = geo_json;
    }

    pub fn is_geo_json(&self) -> bool {
        self.geo_json
    }
}

impl From<NewGeoIndex> for NewIndex {
    fn from(index: NewGeoIndex) -> Self {
        NewIndex::Geo(index)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFulltextIndex {
    #[serde(rename = "type")]
    kind: IndexType,
    fields: Vec<String>,
    min_length: u32,
}

impl NewFulltextIndex {
    pub fn new<F>(fields: F, min_length: u32) -> Self
        where F: IntoIterator<Item=String>
    {
        NewFulltextIndex {
            kind: IndexType::Fulltext,
            fields: Vec::from_iter(fields.into_iter()),
            min_length,
        }
    }

    pub fn with_field<F>(field: F, min_length: u32) -> Self
        where F: Into<String>
    {
        NewFulltextIndex {
            kind: IndexType::Fulltext,
            fields: vec![field.into()],
            min_length,
        }
    }

    pub fn fields_mut(&mut self) -> &mut Vec<String> {
        &mut self.fields
    }

    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    pub fn set_min_length(&mut self, min_length: u32) {
        self.min_length = min_length;
    }

    pub fn min_length(&self) -> u32 {
        self.min_length
    }
}

impl From<NewFulltextIndex> for NewIndex {
    fn from(index: NewFulltextIndex) -> Self {
        NewIndex::Fulltext(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum IndexType {
    Primary,
    Hash,
    SkipList,
    Persistent,
    Geo1,
    Geo2,
    Geo,
    Fulltext,
    Edge,
}

impl Serialize for IndexType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::IndexType::*;
        let type_str = match *self {
            Primary => INDEX_TYPE_PRIMARY,
            Hash => INDEX_TYPE_HASH,
            SkipList => INDEX_TYPE_SKIP_LIST,
            Persistent => INDEX_TYPE_PERSISTENT,
            Geo1 => INDEX_TYPE_GEO1,
            Geo2 => INDEX_TYPE_GEO2,
            Geo => INDEX_TYPE_GEO,
            Fulltext => INDEX_TYPE_FULLTEXT,
            Edge => INDEX_TYPE_EDGE,
        };
        serializer.serialize_str(type_str)
    }
}

impl<'de> Deserialize<'de> for IndexType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use self::IndexType::*;
        let value = String::deserialize(deserializer)?;
        match &value[..] {
            INDEX_TYPE_PRIMARY => Ok(Primary),
            INDEX_TYPE_HASH => Ok(Hash),
            INDEX_TYPE_SKIP_LIST => Ok(SkipList),
            INDEX_TYPE_PERSISTENT => Ok(Persistent),
            INDEX_TYPE_GEO1 => Ok(Geo1),
            INDEX_TYPE_GEO2 => Ok(Geo2),
            INDEX_TYPE_GEO => Ok(Geo),
            INDEX_TYPE_FULLTEXT => Ok(Fulltext),
            INDEX_TYPE_EDGE => Ok(Edge),
            _ => Err(D::Error::custom(format!("Unsupported index type: {:?}", value))),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenericIndex {
    #[serde(rename = "type")]
    kind: IndexType,
    id: IndexIdOption,
    fields: Vec<String>,
    selectivity_estimate: Option<u32>,
    is_newly_created: Option<bool>,
    unique: Option<bool>,
    sparse: Option<bool>,
    deduplicate: Option<bool>,
    constraint: Option<bool>,
    min_length: Option<u32>,
    geo_json: Option<bool>,
}

impl<'de> Deserialize<'de> for Index {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use self::IndexType::*;
        let GenericIndex {
            kind,
            id,
            fields,
            selectivity_estimate,
            is_newly_created,
            unique,
            sparse,
            deduplicate,
            constraint,
            min_length,
            geo_json,
        } = GenericIndex::deserialize(deserializer)?;
        match kind {
            Primary => match (selectivity_estimate, sparse, unique) {
                (Some(selectivity_estimate), Some(sparse), Some(unique)) =>
                    Ok(Index::Primary(PrimaryIndex {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                        selectivity_estimate,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            }
            Hash => match (deduplicate, selectivity_estimate, sparse, unique) {
                (Some(deduplicate), Some(selectivity_estimate), Some(sparse), Some(unique)) =>
                    Ok(Index::Hash(HashIndex {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                        deduplicate,
                        selectivity_estimate,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            SkipList =>match (deduplicate, sparse, unique) {
                (Some(deduplicate), Some(sparse), Some(unique)) =>
                    Ok(Index::SkipList(SkipListIndex {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                        deduplicate,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            Persistent => match (deduplicate, sparse, unique) {
                (Some(deduplicate), Some(sparse), Some(unique)) =>
                    Ok(Index::Persistent(PersistentIndex {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                        deduplicate,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            Geo1 => match (geo_json, constraint, sparse, unique) {
                (Some(geo_json), Some(constraint), Some(sparse), Some(unique)) =>
                    Ok(Index::Geo1(Geo1Index {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                        constraint,
                        geo_json,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            Geo2 => match (constraint, sparse, unique) {
                (Some(constraint), Some(sparse), Some(unique)) =>
                    Ok(Index::Geo2(Geo2Index {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                        constraint,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            Fulltext => match (min_length, sparse, unique) {
                (Some(min_length), Some(sparse), Some(unique)) =>
                    Ok(Index::Fulltext(FulltextIndex {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                        min_length,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            Edge => match (sparse, unique) {
                (Some(sparse), Some(unique)) =>
                    Ok(Index::Edge(EdgeIndex {
                        newly_created: is_newly_created.unwrap_or(false),
                        id,
                        fields,
                        sparse,
                        unique,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            }
            Geo => Err(D::Error::custom("Unsupported index type")),
        }
    }
}
