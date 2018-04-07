
pub mod methods;
pub mod types;

pub mod prelude {
    pub use super::methods::*;
    pub use super::types::*;
}

const DEFAULT_ROOT_PASSWORD: &str = "ARANGODB_DEFAULT_ROOT_PASSWORD";
