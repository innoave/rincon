
# Rincon Test Helper

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon_test_helper.svg
[docs_badge]: https://docs.rs/rincon_test_helper/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon_test_helper
[documentation]: https://docs.rs/rincon_test_helper
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[Rincon project]: https://github.com/innoave/rincon
[license]: ../LICENSE
[rincon]: ../rincon
[rincon_test_helper]: ../rincon_test_helper

The [rincon_test_helper] [crate] provides functions to support integration tests with an [ArangoDB]
server. This crate is mainly for being used internally in this project to factor out common setup
and teardown functionality of the integration tests.

The [rincon_test_helper] [crate] is part of the [Rincon ArangoDB Rust driver project][Rincon project].

## Usage

To use the test helpers for integration tests in your own project add this to your `Cargo.toml`:

```toml
[dev-dependencies]
rincon_test_helper = "0.1"
```

And this to your crate root:
```rust
#[cfg(test)] extern crate rincon_test_helper;
```

## License

Licensed under Apache License, Version 2.0<br/>
see [LICENSE] or http://www.apache.org/licenses/LICENSE-2.0 for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.


[ArangoDB]: https://www.arangodb.com
[AQL]: https://docs.arangodb.com/3.2/AQL/index.html
[Rust]: https://www.rust-lang.org
