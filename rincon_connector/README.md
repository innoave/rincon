
# Rincon Connector

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon_connector.svg
[docs_badge]: https://docs.rs/rincon_connector/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon_connector
[documentation]: https://docs.rs/rincon_connector
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: ../LICENSE
[rincon]: https://github.com/innoave/rincon
[rincon_connector]: ../rincon_connector
[rincon_client]: ../rincon_client
[rincon_session]: ../rincon_session
[rincon_session_async]: ../rincon_session_async

The [rincon_connector] [crate] provides the communication layer of the driver. Currently this crate
provides one basic connector which uses JSON over HTTP or HTTPS. In the future also other connectors
may be provided. 

This crate is separated from the [rincon_session], [rincon_session_async] and [rincon_client] crates
for a flexible and modular design. This enables one to implement a custom connector with some
sophisticated or project specific features with a minimum effort and combine it with the existing
functionality.

The [rincon_connector] [crate] is part of the [Rincon] [ArangoDB] Rust driver project.

## Usage

The [rincon_connector] [create] is needed regardless whether the session APIs provided by the
[rincon_session] and [rincon_session_async] crates or the low level API provided by the
[rincon_client] are used.

Additionally to one of the mentioned crates add this to your `Cargo.toml`:

```toml
[dependencies]
rincon_connector = "0.1"
```

And this to your crate root:

```rust
extern crate rincon_connector;
```

## License

Licensed under Apache License, Version 2.0<br/>
see [LICENSE] or http://www.apache.org/licenses/LICENSE-2.0 for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.


[ArangoDB]: https://www.arangodb.org
[AQL]: https://docs.arangodb.com/3.2/AQL/index.html
[Rust]: https://www.rust-lang.org
