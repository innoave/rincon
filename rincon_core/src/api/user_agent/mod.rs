
#[cfg(test)]
mod tests;

use std::fmt::{self, Debug, Display, Write};

pub trait UserAgent: Debug {
    fn name(&self) -> &str;

    fn version(&self) -> &Version;

    fn homepage(&self) -> &str;

    /// Formats this `UserAgent` as a string.
    ///
    /// This method is intended to be used as the default implementation of
    /// the `Display` trait for your own `UserAgent` type. Your implementation
    /// of the `Display` trait for your own `UserAgent` type can simple delegate
    /// to calling this method.
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())?;
        f.write_str(" v")?;
        self.version().format(f)?;
        if !self.homepage().is_empty() {
            f.write_str(", ")?;
            f.write_str(self.homepage())?;
        }
        Ok(())
    }
}

pub trait Version {
    fn major(&self) -> &str;

    fn minor(&self) -> &str;

    fn patch(&self) -> &str;

    fn pre(&self) -> &str;

    /// Formats this `Version` as a string.
    ///
    /// This method is intended to be used as the default implementation of
    /// the `Display` trait on your own `Version` type. Your implementation
    /// of the `Display` trait for your own `Version` type can simple delegate
    /// to calling this method.
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.major())?;
        f.write_char('.')?;
        f.write_str(self.minor())?;
        f.write_char('.')?;
        f.write_str(self.patch())?;
        if !self.pre().is_empty() {
            f.write_char('-')?;
            f.write_str(self.pre())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RinconUserAgent;

impl UserAgent for RinconUserAgent {
    fn name(&self) -> &str {
        super::super::LIB_NAME
    }

    fn version(&self) -> &Version {
        &RinconVersion
    }

    fn homepage(&self) -> &str {
        super::super::LIB_HOMEPAGE
    }
}

impl Display for RinconUserAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RinconVersion;

impl Version for RinconVersion {
    fn major(&self) -> &str {
        super::super::LIB_VERSION_MAJOR
    }

    fn minor(&self) -> &str {
        super::super::LIB_VERSION_MINOR
    }

    fn patch(&self) -> &str {
        super::super::LIB_VERSION_PATCH
    }

    fn pre(&self) -> &str {
        super::super::LIB_VERSION_PRE
    }
}

impl Display for RinconVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
