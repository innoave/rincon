
use rincon_core::api::user_agent::{UserAgent, Version};
use build;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RinconUserAgent;

impl UserAgent for RinconUserAgent {
    fn name(&self) -> &str {
        build::NAME
    }

    fn version(&self) -> &Version {
        &RinconVersion
    }

    fn homepage(&self) -> &str {
        build::HOMEPAGE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RinconVersion;

impl Version for RinconVersion {
    fn major(&self) -> &str {
        build::MAJOR_VERSION
    }

    fn minor(&self) -> &str {
        build::MINOR_VERSION
    }

    fn patch(&self) -> &str {
        build::PATCH_VERSION
    }

    fn pre(&self) -> &str {
        build::PRE_VERSION
    }
}
