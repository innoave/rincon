
use api::{Method, Operation, Parameters, Prepare, RpcErrorType};
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
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl Prepare for GetTargetVersion {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_admin/database/target-version")
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

    pub fn details(&self) -> bool {
        self.details
    }
}

impl Method for GetServerVersion {
    type Result = ServerVersion;
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: None,
        code_field: None,
    };
}

impl Prepare for GetServerVersion {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/version")
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if self.details {
            params.push("details", "true");
        }
        params
    }
}
