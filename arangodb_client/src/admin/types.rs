
use arangodb_core::api::types::JsonValue;

/// Represents the database-version that this server requires.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetVersion {
    version: String,
}

impl TargetVersion {
    /// Returns the target version as `&str`.
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
    /// The string has the format 'major.minor.sub'. major and minor will be
    /// numeric, and sub may contain a number or a textual version.
    version: String,
    /// The license string.
    license: String,
    /// An optional JSON object with additional details. This is returned only
    /// if the details query parameter is set to true in the request.
    details: Option<JsonValue>,
}

impl ServerVersion {
    /// Returns the major part of the server version.
    ///
    /// The major part is the first part in the version format
    /// 'major.minor.sub'.
    pub fn major(&self) -> &str {
        self.version.split('.').nth(0).unwrap()
    }

    /// Returns the minor part of the server version.
    ///
    /// The minor part is the second part in the version format
    /// 'major.minor.sub'.
    pub fn minor(&self) -> &str {
        self.version.split('.').nth(1).unwrap()
    }

    /// Returns the sub part of the server version.
    ///
    /// The sub part is the first part in the version format
    /// 'major.minor.sub'.
    pub fn sub(&self) -> &str {
        self.version.split('.').nth(2).unwrap()
    }

    /// Returns the server name.
    ///
    /// This will always contain 'arango'.
    pub fn server(&self) -> &str {
        &self.server
    }

    /// Returns the server version as a '&str'.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the license string.
    pub fn license(&self) -> &str {
        &self.license
    }

    /// Returns additional details.
    ///
    /// The details are present only if the details method parameter was set
    /// to true.
    pub fn details(&self) -> Option<&JsonValue> {
        self.details.as_ref()
    }
}
