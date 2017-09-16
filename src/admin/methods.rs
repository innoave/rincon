
use api::{Method, Operation, Parameters, Prepare};
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

impl Prepare for GetTargetVersion {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> &str {
        "/_admin/database/target-version"
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
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

impl Prepare for GetServerVersion {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> &str {
        "/_api/version"
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if self.details {
            params.push("details", "true");
        }
        params
    }
}
