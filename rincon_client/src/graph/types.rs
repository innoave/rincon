//! Types used in methods for managing graphs.

use std::fmt::{self, Debug};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{Deserialize, DeserializeOwned, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use document::types::{DocumentId, DocumentKey, Revision};
use rincon_core::arango::protocol::{
    FIELD_EDGE_DEFINITIONS, FIELD_ENTITY_FROM, FIELD_ENTITY_ID, FIELD_ENTITY_KEY,
    FIELD_ENTITY_OLD_REVISION, FIELD_ENTITY_REVISION, FIELD_ENTITY_TO, FIELD_NAME,
    FIELD_ORPHAN_COLLECTIONS,
};
use rincon_core::arango::protocol::{FIELD_IS_SMART, FIELD_SMART_GRAPH_ATTRIBUTE};
use rincon_core::arango::protocol::{FIELD_NUMBER_OF_SHARDS, FIELD_REPLICATION_FACTOR};

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    id: DocumentId,
    key: DocumentKey,
    revision: Revision,
    name: String,
    edge_definitions: Vec<EdgeDefinition>,
    orphan_collections: Vec<String>,
    #[cfg(feature = "enterprise")]
    smart: bool,
    #[cfg(feature = "enterprise")]
    smart_graph_attribute: String,
    #[cfg(feature = "cluster")]
    number_of_shards: u16,
    #[cfg(feature = "cluster")]
    replication_factor: u64,
}

impl Graph {
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn key(&self) -> &DocumentKey {
        &self.key
    }

    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn edge_definitions(&self) -> &[EdgeDefinition] {
        &self.edge_definitions
    }

    pub fn orphan_collections(&self) -> &[String] {
        &self.orphan_collections
    }

    #[cfg(feature = "enterprise")]
    pub fn is_smart(&self) -> bool {
        self.smart
    }

    #[cfg(feature = "enterprise")]
    pub fn smart_graph_attribute(&self) -> &str {
        &self.smart_graph_attribute
    }

    #[cfg(feature = "cluster")]
    pub fn number_of_shards(&self) -> u16 {
        self.number_of_shards
    }

    #[cfg(feature = "cluster")]
    pub fn replication_factor(&self) -> u64 {
        self.replication_factor
    }
}

impl<'de> Deserialize<'de> for Graph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;

        enum GraphField {
            Id,
            Key,
            Revision,
            Name,
            EdgeDefinitions,
            OrphanCollections,
            Smart,
            SmartGraphAttribute,
            NumberOfShards,
            ReplicationFactor,
            Other(String),
        }

        impl<'de> Deserialize<'de> for GraphField {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                use serde::de::Error;

                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = GraphField;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a string representing a graph field name")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        Ok(match value {
                            FIELD_ENTITY_ID => GraphField::Id,
                            FIELD_ENTITY_KEY => GraphField::Key,
                            FIELD_ENTITY_REVISION => GraphField::Revision,
                            FIELD_NAME => GraphField::Name,
                            FIELD_EDGE_DEFINITIONS => GraphField::EdgeDefinitions,
                            FIELD_ORPHAN_COLLECTIONS => GraphField::OrphanCollections,
                            FIELD_IS_SMART => GraphField::Smart,
                            FIELD_SMART_GRAPH_ATTRIBUTE => GraphField::SmartGraphAttribute,
                            FIELD_NUMBER_OF_SHARDS => GraphField::NumberOfShards,
                            FIELD_REPLICATION_FACTOR => GraphField::ReplicationFactor,
                            _ => GraphField::Other(value.to_owned()),
                        })
                    }
                }

                deserializer.deserialize_str(FieldVisitor)
            }
        }

        struct GraphVisitor;

        impl<'de> Visitor<'de> for GraphVisitor {
            type Value = Graph;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("at least fields '_id', '_rev' and either '_key' or 'name'")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut key: Option<String> = None;
                let mut revision: Option<String> = None;
                let mut name: Option<String> = None;
                let mut edge_definitions: Option<Vec<EdgeDefinition>> = None;
                let mut orphan_collections: Option<Vec<String>> = None;
                #[cfg(feature = "enterprise")]
                let mut smart: Option<bool> = None;
                #[cfg(feature = "enterprise")]
                let mut smart_graph_attribute: Option<String> = None;
                #[cfg(feature = "cluster")]
                let mut number_of_shards: Option<u16> = None;
                #[cfg(feature = "cluster")]
                let mut replication_factor: Option<u64> = None;

                let mut fields = map;
                while let Some(field_name) = fields.next_key()? {
                    match field_name {
                        GraphField::Id => {
                            id = fields.next_value()?;
                        },
                        GraphField::Key => {
                            key = fields.next_value()?;
                        },
                        GraphField::Revision => {
                            revision = fields.next_value()?;
                        },
                        GraphField::Name => {
                            name = fields.next_value()?;
                        },
                        GraphField::EdgeDefinitions => {
                            edge_definitions = fields.next_value()?;
                        },
                        GraphField::OrphanCollections => {
                            orphan_collections = fields.next_value()?;
                        },
                        #[cfg(feature = "enterprise")]
                        GraphField::Smart => {
                            smart = fields.next_value()?;
                        },
                        #[cfg(not(feature = "enterprise"))]
                        GraphField::Smart => {
                            let _: Option<bool> = fields.next_value()?;
                        },
                        #[cfg(feature = "enterprise")]
                        GraphField::SmartGraphAttribute => {
                            smart_graph_attribute = fields.next_value()?;
                        },
                        #[cfg(not(feature = "enterprise"))]
                        GraphField::SmartGraphAttribute => {
                            let _: Option<String> = fields.next_value()?;
                        },
                        #[cfg(feature = "cluster")]
                        GraphField::NumberOfShards => {
                            number_of_shards = fields.next_value()?;
                        },
                        #[cfg(not(feature = "cluster"))]
                        GraphField::NumberOfShards => {
                            let _: Option<u16> = fields.next_value()?;
                        },
                        #[cfg(feature = "cluster")]
                        GraphField::ReplicationFactor => {
                            replication_factor = fields.next_value()?;
                        },
                        #[cfg(not(feature = "cluster"))]
                        GraphField::ReplicationFactor => {
                            let _: Option<u64> = fields.next_value()?;
                        },
                        GraphField::Other(_) => {
                            //ignore
                            let _: Value = fields.next_value()?;
                        },
                    }
                }

                #[cfg(all(feature = "enterprise", feature = "cluster"))]
                match (
                    id,
                    key,
                    revision,
                    name,
                    edge_definitions,
                    orphan_collections,
                    smart,
                    smart_graph_attribute,
                    number_of_shards,
                    replication_factor,
                ) {
                    (
                        Some(id),
                        _key,
                        Some(revision),
                        _name,
                        Some(edge_definitions),
                        Some(orphan_collections),
                        Some(smart),
                        Some(smart_graph_attribute),
                        Some(number_of_shards),
                        Some(replication_factor),
                    ) => {
                        let (key, name) = match (_key, _name) {
                            (Some(_key), Some(_name)) => (_key, _name),
                            (Some(_key), None) => (_key.clone(), _key),
                            (None, Some(_name)) => (_name.clone(), _name),
                            (None, None) => return Err(A::Error::missing_field("name or _key")),
                        };
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        Ok(Graph {
                            id,
                            key,
                            revision,
                            name,
                            edge_definitions,
                            orphan_collections,
                            smart,
                            smart_graph_attribute,
                            number_of_shards,
                            replication_factor,
                        })
                    },
                    (None, _, _, _, _, _, _, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_ENTITY_ID))
                    },
                    (_, _, None, _, _, _, _, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_ENTITY_REVISION))
                    },
                    (_, _, _, _, None, _, _, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_EDGE_DEFINITIONS))
                    },
                    (_, _, _, _, _, None, _, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_ORPHAN_COLLECTIONS))
                    },
                    (_, _, _, _, _, _, None, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_IS_SMART))
                    },
                    (_, _, _, _, _, _, _, None, _, _) => {
                        Err(A::Error::missing_field(FIELD_SMART_GRAPH_ATTRIBUTE))
                    },
                    (_, _, _, _, _, _, _, _, None, _) => {
                        Err(A::Error::missing_field(FIELD_NUMBER_OF_SHARDS))
                    },
                    (_, _, _, _, _, _, _, _, _, None) => {
                        Err(A::Error::missing_field(FIELD_REPLICATION_FACTOR))
                    },
                }

                #[cfg(not(any(feature = "enterprise", feature = "cluster")))]
                match (
                    id,
                    key,
                    revision,
                    name,
                    edge_definitions,
                    orphan_collections,
                ) {
                    (
                        Some(id),
                        _key,
                        Some(revision),
                        _name,
                        Some(edge_definitions),
                        Some(orphan_collections),
                    ) => {
                        let (key, name) = match (_key, _name) {
                            (Some(_key), Some(_name)) => (_key, _name),
                            (Some(_key), None) => (_key.clone(), _key),
                            (None, Some(_name)) => (_name.clone(), _name),
                            (None, None) => return Err(A::Error::missing_field("name or _key")),
                        };
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        Ok(Graph {
                            id,
                            key,
                            revision,
                            name,
                            edge_definitions,
                            orphan_collections,
                        })
                    },
                    (None, _, _, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_ID)),
                    (_, _, None, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_REVISION)),
                    (_, _, _, _, None, _) => Err(A::Error::missing_field(FIELD_EDGE_DEFINITIONS)),
                    (_, _, _, _, _, None) => Err(A::Error::missing_field(FIELD_ORPHAN_COLLECTIONS)),
                }

                #[cfg(all(feature = "enterprise", not(feature = "cluster")))]
                match (
                    id,
                    key,
                    revision,
                    name,
                    edge_definitions,
                    orphan_collections,
                    smart,
                    smart_graph_attribute,
                ) {
                    (
                        Some(id),
                        _key,
                        Some(revision),
                        _name,
                        Some(edge_definitions),
                        Some(orphan_collections),
                        Some(smart),
                        Some(smart_graph_attribute),
                    ) => {
                        let (key, name) = match (_key, _name) {
                            (Some(_key), Some(_name)) => (_key, _name),
                            (Some(_key), None) => (_key.clone(), _key),
                            (None, Some(_name)) => (_name.clone(), _name),
                            (None, None) => return Err(A::Error::missing_field("name or _key")),
                        };
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        Ok(Graph {
                            id,
                            key,
                            revision,
                            name,
                            edge_definitions,
                            orphan_collections,
                            smart,
                            smart_graph_attribute,
                        })
                    },
                    (None, _, _, _, _, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_ID)),
                    (_, _, None, _, _, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_ENTITY_REVISION))
                    },
                    (_, _, _, _, None, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_EDGE_DEFINITIONS))
                    },
                    (_, _, _, _, _, None, _, _) => {
                        Err(A::Error::missing_field(FIELD_ORPHAN_COLLECTIONS))
                    },
                    (_, _, _, _, _, _, None, _) => Err(A::Error::missing_field(FIELD_IS_SMART)),
                    (_, _, _, _, _, _, _, None) => {
                        Err(A::Error::missing_field(FIELD_SMART_GRAPH_ATTRIBUTE))
                    },
                }

                #[cfg(all(not(feature = "enterprise"), feature = "cluster"))]
                match (
                    id,
                    key,
                    revision,
                    name,
                    edge_definitions,
                    orphan_collections,
                    number_of_shards,
                    replication_factor,
                ) {
                    (
                        Some(id),
                        _key,
                        Some(revision),
                        _name,
                        Some(edge_definitions),
                        Some(orphan_collections),
                        Some(number_of_shards),
                        Some(replication_factor),
                    ) => {
                        let (key, name) = match (_key, _name) {
                            (Some(_key), Some(_name)) => (_key, _name),
                            (Some(_key), None) => (_key.clone(), _key),
                            (None, Some(_name)) => (_name.clone(), _name),
                            (None, None) => return Err(A::Error::missing_field("name or _key")),
                        };
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        Ok(Graph {
                            id,
                            key,
                            revision,
                            name,
                            edge_definitions,
                            orphan_collections,
                            number_of_shards,
                            replication_factor,
                        })
                    },
                    (None, _, _, _, _, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_ID)),
                    (_, _, None, _, _, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_ENTITY_REVISION))
                    },
                    (_, _, _, _, None, _, _, _) => {
                        Err(A::Error::missing_field(FIELD_EDGE_DEFINITIONS))
                    },
                    (_, _, _, _, _, None, _, _) => {
                        Err(A::Error::missing_field(FIELD_ORPHAN_COLLECTIONS))
                    },
                    (_, _, _, _, _, _, None, _) => {
                        Err(A::Error::missing_field(FIELD_NUMBER_OF_SHARDS))
                    },
                    (_, _, _, _, _, _, _, None) => {
                        Err(A::Error::missing_field(FIELD_REPLICATION_FACTOR))
                    },
                }
            }
        }

        deserializer.deserialize_map(GraphVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGraph {
    name: String,
    edge_definitions: Vec<EdgeDefinition>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    orphan_collections: Vec<String>,
    #[cfg(feature = "enterprise")]
    #[serde(rename = "isSmart")]
    smart: bool,
    #[cfg(any(feature = "enterprise", feature = "cluster"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GraphOptions>,
}

impl NewGraph {
    #[cfg(not(feature = "enterprise"))]
    pub fn new<Name, Edges>(name: Name, edges: Edges) -> Self
    where
        Name: Into<String>,
        Edges: IntoIterator<Item = EdgeDefinition>,
    {
        NewGraph {
            name: name.into(),
            edge_definitions: Vec::from_iter(edges.into_iter()),
            orphan_collections: Vec::new(),
            #[cfg(any(feature = "enterprise", feature = "cluster"))]
            options: None,
        }
    }

    #[cfg(feature = "enterprise")]
    pub fn new<Name, Edges>(name: Name, edges: Edges, smart: bool) -> Self
    where
        Name: Into<String>,
        Edges: IntoIterator<Item = EdgeDefinition>,
    {
        NewGraph {
            name: name.into(),
            edge_definitions: Vec::from_iter(edges.into_iter()),
            orphan_collections: Vec::new(),
            smart,
            options: None,
        }
    }

    pub fn with_name<Name>(name: Name) -> Self
    where
        Name: Into<String>,
    {
        #[cfg(not(feature = "enterprise"))]
        {
            NewGraph::new(name, Vec::new())
        }
        #[cfg(feature = "enterprise")]
        {
            NewGraph::new(name, Vec::new(), false)
        }
    }

    pub fn with_edge_definitions<Edges>(mut self, edges: Edges) -> Self
    where
        Edges: IntoIterator<Item = EdgeDefinition>,
    {
        self.edge_definitions = Vec::from_iter(edges.into_iter());
        self
    }

    pub fn with_orphan_collections<O>(mut self, orphan_collections: O) -> Self
    where
        O: IntoIterator<Item = String>,
    {
        self.orphan_collections = Vec::from_iter(orphan_collections.into_iter());
        self
    }

    pub fn edge_definitions_mut(&mut self) -> &mut Vec<EdgeDefinition> {
        &mut self.edge_definitions
    }

    pub fn orphan_collections_mut(&mut self) -> &mut Vec<String> {
        &mut self.orphan_collections
    }

    #[cfg(feature = "enterprise")]
    pub fn set_smart(&mut self, smart: bool) {
        self.smart = smart;
    }

    #[cfg(any(feature = "enterprise", feature = "cluster"))]
    pub fn options_mut(&mut self) -> &mut GraphOptions {
        self.options.get_or_insert_with(GraphOptions::default)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn edge_definitions(&self) -> &[EdgeDefinition] {
        &self.edge_definitions
    }

    pub fn orphan_collections(&self) -> &[String] {
        &self.orphan_collections
    }

    #[cfg(feature = "enterprise")]
    pub fn is_smart(&self) -> bool {
        self.smart
    }

    #[cfg(any(feature = "enterprise", feature = "cluster"))]
    pub fn options(&self) -> Option<&GraphOptions> {
        self.options.as_ref()
    }
}

#[cfg(any(feature = "enterprise", feature = "cluster"))]
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphOptions {
    #[cfg(feature = "enterprise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    smart_graph_attribute: Option<String>,
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_shards: Option<u16>,
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_factor: Option<u64>,
}

#[cfg(any(feature = "enterprise", feature = "cluster"))]
impl GraphOptions {
    fn empty() -> Self {
        GraphOptions {
            #[cfg(feature = "enterprise")]
            smart_graph_attribute: None,
            #[cfg(feature = "cluster")]
            number_of_shards: None,
            #[cfg(feature = "cluster")]
            replication_factor: None,
        }
    }

    #[cfg(feature = "enterprise")]
    pub fn set_smart_graph_attribute<Attr>(&mut self, smart_graph_attribute: Attr)
    where
        Attr: Into<Option<String>>,
    {
        self.smart_graph_attribute = smart_graph_attribute.into();
    }

    #[cfg(feature = "cluster")]
    pub fn set_number_of_shards<S>(&mut self, number_of_shards: S)
    where
        S: Into<Option<u16>>,
    {
        self.number_of_shards = number_of_shards.into();
    }

    #[cfg(feature = "cluster")]
    pub fn set_replication_factor<R>(&mut self, replication_factor: R)
    where
        R: Into<Option<u64>>,
    {
        self.replication_factor = replication_factor.into();
    }

    #[cfg(feature = "enterprise")]
    pub fn smart_graph_attribute(&self) -> Option<&String> {
        self.smart_graph_attribute.as_ref()
    }

    #[cfg(feature = "cluster")]
    pub fn number_of_shards(&self) -> Option<u16> {
        self.number_of_shards
    }

    #[cfg(feature = "cluster")]
    pub fn replication_factor(&self) -> Option<u64> {
        self.replication_factor
    }
}

#[cfg(any(feature = "enterprise", feature = "cluster"))]
impl Default for GraphOptions {
    fn default() -> Self {
        GraphOptions::empty()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VertexCollection {
    collection: String,
}

impl VertexCollection {
    pub fn new<Coll>(collection: Coll) -> Self
    where
        Coll: Into<String>,
    {
        VertexCollection {
            collection: collection.into(),
        }
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeCollection {
    collection: String,
}

impl EdgeCollection {
    pub fn new<Coll>(collection: Coll) -> Self
    where
        Coll: Into<String>,
    {
        EdgeCollection {
            collection: collection.into(),
        }
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeDefinition {
    collection: String,
    from: Vec<String>,
    to: Vec<String>,
}

impl EdgeDefinition {
    pub fn new<Coll, From, To>(collection: Coll, from: From, to: To) -> Self
    where
        Coll: Into<String>,
        From: IntoIterator<Item = String>,
        To: IntoIterator<Item = String>,
    {
        EdgeDefinition {
            collection: collection.into(),
            from: Vec::from_iter(from.into_iter()),
            to: Vec::from_iter(to.into_iter()),
        }
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn from(&self) -> &[String] {
        &self.from
    }

    pub fn to(&self) -> &[String] {
        &self.to
    }
}

/// All the possible keys, to avoid allocating memory if it is a key we
/// recognize. Private.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EdgeField {
    Id,
    Key,
    Revision,
    OldRevision,
    From,
    To,
    Other(String),
}

impl<'de> Deserialize<'de> for EdgeField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = EdgeField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an edge field name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(match value {
                    FIELD_ENTITY_ID => EdgeField::Id,
                    FIELD_ENTITY_KEY => EdgeField::Key,
                    FIELD_ENTITY_REVISION => EdgeField::Revision,
                    FIELD_ENTITY_OLD_REVISION => EdgeField::OldRevision,
                    FIELD_ENTITY_FROM => EdgeField::From,
                    FIELD_ENTITY_TO => EdgeField::To,
                    _ => EdgeField::Other(value.to_owned()),
                })
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge<T> {
    id: DocumentId,
    key: DocumentKey,
    revision: Revision,
    from: DocumentId,
    to: DocumentId,
    content: T,
}

impl<T> Edge<T> {
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn key(&self) -> &DocumentKey {
        &self.key
    }

    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    pub fn from(&self) -> &DocumentId {
        &self.from
    }

    pub fn to(&self) -> &DocumentId {
        &self.to
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn unwrap_content(self) -> T {
        self.content
    }
}

impl<'de, T> Deserialize<'de> for Edge<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::{from_value, Map, Value};

        struct EdgeVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for EdgeVisitor<T>
        where
            T: DeserializeOwned,
        {
            type Value = Edge<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("at least fields '_id', '_key', '_rev', '_from' and '_to'")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut key: Option<String> = None;
                let mut revision: Option<String> = None;
                let mut from: Option<String> = None;
                let mut to: Option<String> = None;
                let mut other = Map::new();

                let mut fields = map;
                while let Some(name) = fields.next_key()? {
                    match name {
                        EdgeField::Id => {
                            id = fields.next_value()?;
                        },
                        EdgeField::Key => {
                            key = fields.next_value()?;
                        },
                        EdgeField::Revision => {
                            revision = fields.next_value()?;
                        },
                        EdgeField::OldRevision => {
                            other
                                .insert(FIELD_ENTITY_OLD_REVISION.to_owned(), fields.next_value()?);
                        },
                        EdgeField::From => {
                            from = fields.next_value()?;
                        },
                        EdgeField::To => {
                            to = fields.next_value()?;
                        },
                        EdgeField::Other(name) => {
                            other.insert(name, fields.next_value()?);
                        },
                    }
                }

                match (id, key, revision, from, to) {
                    (Some(id), Some(key), Some(revision), Some(from), Some(to)) => {
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        let from = DocumentId::from_str(&from).map_err(A::Error::custom)?;
                        let to = DocumentId::from_str(&to).map_err(A::Error::custom)?;
                        let content = from_value(Value::Object(other)).map_err(A::Error::custom)?;
                        Ok(Edge {
                            id,
                            key,
                            revision,
                            from,
                            to,
                            content,
                        })
                    },
                    (None, _, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_ID)),
                    (_, None, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_KEY)),
                    (_, _, None, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_REVISION)),
                    (_, _, _, None, _) => Err(A::Error::missing_field(FIELD_ENTITY_FROM)),
                    (_, _, _, _, None) => Err(A::Error::missing_field(FIELD_ENTITY_TO)),
                }
            }
        }

        deserializer.deserialize_map(EdgeVisitor(PhantomData))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewEdge<T> {
    key: Option<DocumentKey>,
    from: DocumentId,
    to: DocumentId,
    content: T,
}

impl<T> NewEdge<T> {
    pub fn new(from: DocumentId, to: DocumentId, content: T) -> Self {
        NewEdge {
            key: None,
            from,
            to,
            content,
        }
    }

    pub fn with_key<Key>(mut self, key: Key) -> Self
    where
        Key: Into<Option<DocumentKey>>,
    {
        self.key = key.into();
        self
    }

    pub fn key(&self) -> Option<&DocumentKey> {
        self.key.as_ref()
    }

    pub fn from(&self) -> &DocumentId {
        &self.from
    }

    pub fn to(&self) -> &DocumentId {
        &self.to
    }

    pub fn content(&self) -> &T {
        &self.content
    }
}

impl<T> Serialize for NewEdge<T>
where
    T: Serialize + Debug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        use serde_json::{self, Value};

        let mut json_value = serde_json::to_value(&self.content).map_err(S::Error::custom)?;
        match json_value {
            Value::Object(ref mut fields) => {
                if let Some(ref key) = self.key {
                    fields.insert(FIELD_ENTITY_KEY.to_owned(), Value::String(key.as_str().to_owned()));
                }
                fields.insert(FIELD_ENTITY_FROM.to_owned(), Value::String(self.from.to_string()));
                fields.insert(FIELD_ENTITY_TO.to_owned(), Value::String(self.to.to_string()));
            },
            _ => return Err(S::Error::custom(format!("Invalid edge content! Only types that serialize into valid Json objects are supported. But got: {:?}", &self.content))),
        }
        let json_value = json_value;
        json_value.serialize(serializer)
    }
}
