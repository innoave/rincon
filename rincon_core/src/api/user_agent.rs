
use std::fmt::Debug;

pub trait UserAgent: Debug {
    fn name(&self) -> &str;
    fn version(&self) -> &Version;
    fn homepage(&self) -> &str;
}

pub trait Version {
    fn major(&self) -> &str;
    fn minor(&self) -> &str;
    fn patch(&self) -> &str;
    fn pre(&self) -> &str;
}
