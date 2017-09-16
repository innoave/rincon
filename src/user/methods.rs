
use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use api::{Method, Operation, Parameters, Prepare, Result};
use super::types::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ListAvailableUsers<T> {
    _t: PhantomData<T>,
}

impl<T> ListAvailableUsers<T> {
    pub fn new() -> Self {
        ListAvailableUsers {
            _t: PhantomData,
        }
    }
}

impl<T> Method for ListAvailableUsers<T>
    where T: UserInfo + DeserializeOwned + 'static
{
    type Result = Result<Vec<User<T>>>;
}

impl<T> Prepare for ListAvailableUsers<T> {
    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> &str {
        "/_api/user"
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }
}
