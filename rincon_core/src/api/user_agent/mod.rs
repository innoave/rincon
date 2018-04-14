//! The User Agent represents information about the client that communicates
//! with the ArangoDB server.
//!
//! Each application may define itself as the user agent that communicates with
//! the ArangoDB server by defining a struct that implements the `UserAgent`
//! trait and providing this struct when creating a `Connector` by calling the
//! `Connector::with_user_agent()` function.
//!
//! If an application creates a `Connector` without specifying its own
//! `UserAgent` then the default implementation of the rincon driver itself is
//! used, which is the `RinconUserAgent` struct.

#[cfg(test)]
mod tests;

use std::fmt::{self, Debug, Display, Write};

/// A `UserAgent` provides information about itself when communicating with the
/// server.
///
/// For an example on how to implement this trait see the `RinconUserAgent`
/// struct.
pub trait UserAgent: Debug {
    /// Returns the name of the user agent.
    fn name(&self) -> &str;

    /// Returns the version of the user agent.
    fn version(&self) -> &Version;

    /// Returns the homepage of the user agent.
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

/// A `Version` of a user agent provides details about its version according the
/// semantic versioning specification.
///
/// For an example of how to implement this trait see the `RinconVersion`
/// struct.
pub trait Version {
    /// Returns the major part of the version
    fn major(&self) -> &str;

    /// Returns the minor part of the version
    fn minor(&self) -> &str;

    /// Returns the patch part of the version
    fn patch(&self) -> &str;

    /// Returns the pre release part of the version
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

/// The `RinconUserAgent` implements the `UserAgent` trait for the rincon driver
/// library.
///
/// This user agent is used by `Connector`s provided by the `rincon_connector`
/// crate if an application does not provide its own user agent when it creates
/// a `Connector`.
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

/// The `RinconVersion` implements the `Version` trait for the rincon user agent
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
