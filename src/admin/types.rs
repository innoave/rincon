
use serde_json::Value;

/// Represents the database-version that this server requires.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetVersion {
    version: String,
}

impl TargetVersion {
    pub fn version(&self) -> &str {
        &self.version
    }
}

/// Represents the server name and version number.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerVersion {
    /// Will always contain arango.
    server: String,
    /// The server version string.
    ///
    /// The string has the format "major.minor.sub". major and minor will be numeric, and sub may
    /// contain a number or a textual version.
    version: String,
    /// The license string.
    license: String,
    /// An optional JSON object with additional details. This is returned only if the details query
    /// parameter is set to true in the request.
    details: Option<Value>,
}

impl ServerVersion {
    pub fn major(&self) -> &str {
        self.version.split('.').nth(0).unwrap()
    }

    pub fn minor(&self) -> &str {
        self.version.split('.').nth(1).unwrap()
    }

    pub fn sub(&self) -> &str {
        self.version.split('.').nth(2).unwrap()
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn license(&self) -> &str {
        &self.license
    }

    pub fn details(&self) -> Option<&Value> {
        self.details.as_ref()
    }
}
