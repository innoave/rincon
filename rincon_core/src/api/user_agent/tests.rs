
use super::*;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[test]
fn rincon_version_to_string() {
    assert_eq!(&RinconVersion.to_string(), PKG_VERSION);
}

#[test]
fn rincon_user_agent_to_string() {
    assert_eq!(&RinconUserAgent.to_string(), &("rincon v".to_string() + PKG_VERSION + ", https://github.com/innoave/rincon"));
}
