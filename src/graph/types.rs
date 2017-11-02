
use document::{DocumentId, DocumentKey, Revision};

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge<T> {
    #[serde(rename = "_id")]
    id: DocumentId,
    #[serde(rename = "_key")]
    key: DocumentKey,
    #[serde(rename = "_rev")]
    revision: Revision,
    #[serde(rename = "_from")]
    from: DocumentId,
    #[serde(rename = "_to")]
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

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewEdge<T> {
    #[serde(rename = "_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<DocumentKey>,
    #[serde(rename = "_from")]
    from: DocumentId,
    #[serde(rename = "_to")]
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

    pub fn with_key<K>(mut self, key: K) -> Self
        where K: Into<Option<DocumentKey>>
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
