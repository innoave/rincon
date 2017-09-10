
use statement::{Method, Operation, Parameters, Prepare, PreparedStatement};
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
        let path = "/_admin/database/target-version";
        PreparedStatement::new(self, Operation::Read, path)
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

impl Prepare<Self> for GetServerVersion {
    fn prepare(self) -> PreparedStatement<Self> {
        let path = "/_api/version";
        let mut params = Parameters::default();
        if self.details {
            params.set_str("details", "true");
        }
        PreparedStatement::with_parameters(self, Operation::Read, path, params)
    }
}
