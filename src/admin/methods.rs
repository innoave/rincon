
use api::{Method, Operation, Parameters, Prepare, RpcReturnType};
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct GetTargetVersion {}

impl GetTargetVersion {
    pub fn new() -> Self {
        GetTargetVersion {}
    }
}

impl Method for GetTargetVersion {
    type Result = TargetVersion;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl Prepare for GetTargetVersion {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_admin/database/target-version")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: None,
    };
}

impl Prepare for GetServerVersion {
    type Content = ();

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

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}
