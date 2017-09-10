
use statement::{Method, Operation, Prepare, PreparedStatement};
use super::types::*;

#[derive(Debug, PartialEq, Eq)]
pub struct GetTargetVersion {}

impl GetTargetVersion {
    pub fn new() -> Self {
        GetTargetVersion {}
    }
}

impl Method for GetTargetVersion {
    type Result = TargetVersion;
}

impl Prepare<Self> for GetTargetVersion {
    fn prepare(self) -> PreparedStatement<Self> {
        let path = "/_api/target/version";
        PreparedStatement::new(self, Operation::Read, path.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GetServerVersion {
    details: bool,
}

impl GetServerVersion {
    pub fn new() -> Self {
        GetServerVersion {
            details: false,
        }
    }

    pub fn with_details() -> Self {
        GetServerVersion {
            details: true,
        }
    }
}

impl Method for GetServerVersion {
    type Result = ServerVersion;
}
