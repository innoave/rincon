
# Rincon Core

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon_core.svg
[docs_badge]: https://docs.rs/rincon_core/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon_core
[documentation]: https://docs.rs/rincon_core
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: ../LICENSE
[rincon]: https://github.com/innoave/rincon
[rincon_core]: https://github.com/innoave/rincon/rincon_core
[rincon_core API]: https://docs.rs/rincon_core
[rincon_client]: https://github.com/innoave/rincon/rincon_client
[rincon_session]: https://github.com/innoave/rincon/rincon_session
[rincon_session_async]: https://github.com/innoave/rincon/rincon_session_async

The [rincon_core] [crate] defines common types and functions used by all crates of the [Rincon]
project. This crate is the foundation of the modular design and extensibility of the
[Rincon] [ArangoDB] Rust driver.

The aim is that any connector implementing the [rincon_core API] can be used in combination with
the existing [rincon_session], [rincon_session_async] and [rincon_client] crates. 

It should also be possible to implement methods of the [ArangoDB] REST API which are not supported
yet or will be added in future versions of [ArangoDB] and combine the custom implementations with
the methods provided by the existing crates.

The [rincon_core] [crate] is part of the [Rincon] [ArangoDB] Rust driver project.

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
