
use rincon_core::api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use rincon_core::arango::protocol::{FIELD_CODE, PARAM_DETAILS, PATH_ADMIN,
    PATH_API_VERSION, PATH_DATABASE, PATH_TARGET_VERSION};
use super::types::*;

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq)]
pub struct GetTargetVersion {}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
impl GetTargetVersion {
    pub fn new() -> Self {
        GetTargetVersion {}
    }
}

impl Method for GetTargetVersion {
    type Result = TargetVersion;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetTargetVersion {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_ADMIN) + PATH_DATABASE + PATH_TARGET_VERSION
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq)]
pub struct GetServerVersion {
    details: bool,
}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
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
        String::from(PATH_API_VERSION)
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if self.details {
            params.insert(PARAM_DETAILS, true);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}
