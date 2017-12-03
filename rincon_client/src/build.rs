//! Provides info about the crate from Cargo environment variables set
//! at build time.
//!
//! The environment variables are taken from
//! [Cargo environment variables set for crates](http://doc.crates.io/environment-variables.html#environment-variables-cargo-sets-for-crates)

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAJOR_VERSION: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const MINOR_VERSION: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const PATCH_VERSION: &str = env!("CARGO_PKG_VERSION_PATCH");
pub const PRE_VERSION: &str = env!("CARGO_PKG_VERSION_PRE");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_build_info() {
        println!("BUILD_INFO: {} {}: major={}, minor={}, patch={}, pre={}; homepage={}",
            NAME, VERSION, MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION, PRE_VERSION, HOMEPAGE);

        assert_eq!("rincon_client", NAME);
        assert_eq!("0", MAJOR_VERSION);
        assert_eq!("1", MINOR_VERSION);
    }

}
