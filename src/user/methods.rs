
use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use api::{Method, Operation, Parameters, Prepare, RpcErrorType};
use super::types::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ListAvailableUsers<T> {
    user_info_type: PhantomData<T>,
}

impl<T> ListAvailableUsers<T> {
    pub fn new() -> Self {
        ListAvailableUsers {
            user_info_type: PhantomData,
        }
    }
}

impl<T> Method for ListAvailableUsers<T>
    where T: UserInfo + DeserializeOwned + 'static
{
    type Result = Vec<User<T>>;
    const ERROR_TYPE: RpcErrorType = RpcErrorType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl<T> Prepare for ListAvailableUsers<T> {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/user")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }
}
