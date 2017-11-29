
mod methods;
mod types;
#[cfg(test)] mod types_tests;

pub use self::methods::*;
pub use self::types::*;

const DEFAULT_ROOT_PASSWORD: &str = "ARANGODB_DEFAULT_ROOT_PASSWORD";
//const NO_PASSWORD: &str = "";
