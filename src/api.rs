
use std::collections::HashMap;

use serde::de::DeserializeOwned;

pub trait Method {
    type Result: DeserializeOwned + 'static;
}

pub trait Prepare {
    fn operation(&self) -> Operation;
    fn path(&self) -> &str;
    fn parameters(&self) -> Parameters;
}

#[derive(Debug)]
pub enum Operation {
    Create,
    Read,
    Modify,
    Replace,
    Delete,
}

#[derive(Clone, Debug)]
pub struct Parameters {
    map: HashMap<String, String>,
}

impl Parameters {
    pub fn empty() -> Self {
        Parameters {
            map: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Parameters {
            map: HashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.map.get(name)
    }

    pub fn set_str(&mut self, name: &str, value: &str) {
        self.map.insert(name.to_owned(), value.to_owned());
    }

    pub fn set_string(&mut self, name: String, value: String) {
        self.map.insert(name, value);
    }
}

impl From<HashMap<String, String>> for Parameters {
    fn from(map: HashMap<String, String>) -> Self {
        Parameters {
            map,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Document {
    text: String,
}

impl Document {
    pub fn from_str(text: &str) -> Self {
        Document {
            text: text.to_owned(),
        }
    }

    pub fn from_string(text: String) -> Self {
        Document {
            text,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
