
# ArangoDB Rust Driver

[![Build Status][tcb]][tcl]
[![codevoc.io][cvb]][cvl]
[![Apache-2.0][lib]][lil]
[![Join the chat][gcb]][gcl]
<!--TODO uncomment once this resources are activated!
[![Crates.io][crb]][crl]
[![Docs.rs][dcb]][dcl]
-->

[crb]: https://img.shields.io/crates/v/arangodb_client.svg?style=flat-square
[dcb]: https://docs.rs/arangodb_client/badge.svg
[tcb]: https://img.shields.io/travis/innoave/arangodb-rust-driver/master.svg?style=flat-square
[cvb]: https://img.shields.io/codecov/c/github/innoave/arangodb-rust-driver/master.svg?style=flat-square
[lib]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg?style=flat-square
[gcb]: https://badges.gitter.im/innoave/general.svg?style=flat-square

[crl]: https://crates.io/crates/arangodb_client/
[dcl]: https://docs.rs/arangodb_client
[tcl]: https://travis-ci.org/innoave/arangodb-rust-driver/
[cvl]: https://codecov.io/github/innoave/arangodb-rust-driver?branch=master
[lil]: https://www.apache.org/licenses/LICENSE-2.0
[gcl]: https://gitter.im/innoave/arangodb_client

An ArangoDB client for Rust. It enables low level access to ArangoDB in a
typesafe and Rust idiomatic manner. 

[Documentation](https://docs.rs/arangodb_client)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
arangodb_client = "0.1"
```

Next, add this to your crate:

```rust
extern crate arangodb_client;
```

See the [client example](./examples/client.rs) for a working example.

## License

Licensed under Apache License, Version 2.0<br/>
see [LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0) for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any
additional terms or conditions.
