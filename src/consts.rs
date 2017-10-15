
pub const FIELD_CODE: &str = "code";
pub const FIELD_ID: &str = "id";
pub const FIELD_RESULT: &str = "result";

pub const PATH_ADMIN: &str = "/_admin";
pub const PATH_API_COLLECTION: &str = "/_api/collection";
pub const PATH_API_CURSOR: &str = "/_api/cursor";
pub const PATH_API_DATABASE: &str = "/_api/database";
pub const PATH_API_INDEX: &str = "/_api/index";
pub const PATH_API_USER: &str = "/_api/user";
pub const PATH_API_VERSION: &str = "/_api/version";
pub const PATH_OPEN_AUTH: &str = "/_open/auth";

pub const PATH_CURRENT: &str = "current";
pub const PATH_DATABASE: &str = "database";
pub const PATH_PROPERTIES: &str = "properties";
pub const PATH_RENAME: &str = "rename";
pub const PATH_TARGET_VERSION: &str = "target-version";
pub const PATH_USER: &str = "user";

pub const PARAM_COLLECTION: &str = "collection";
pub const PARAM_DETAILS: &str = "details";
pub const PARAM_EXCLUDE_SYSTEM: &str = "excludeSystem";
#[cfg(feature = "cluster")]
pub const PARAM_WAIT_FOR_SYNC_REPLICATION: &str = "waitForSyncReplication";

pub const VALUE_TRUE: &str = "true";
#[cfg(feature = "cluster")]
pub const VALUE_ZERO: &str = "0";
