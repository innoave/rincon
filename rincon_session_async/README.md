
# Rincon Session Async

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon_session_async.svg
[docs_badge]: https://docs.rs/rincon_session_async/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon_session_async
[documentation]: https://docs.rs/rincon_session_async
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: ../LICENSE
[rincon]: https://github.com/innoave/rincon
[rincon_session_async]: ../rincon_session_async
[rincon_session_async API]: https://docs.rs/rincon_session_async
[rincon_client API]: https://docs.rs/rincon_api
[rincon_session]: ../rincon_session

The [rincon_session_async] [crate] provides a convenient API for __asynchronous__ communication with an
[ArangoDB] server.

The [rincon_session_async API] is a higher level API on top of the [rincon_client API] and provides
additional features:

* Convenient API for applications to communicate to an [ArangoDB] server. (WIP)
<br/>E.g. no need to manually specify the database and collection on each and every request.
* Efficient handling of connections to the [ArangoDB] server (PLANNED)
* Efficient execution of batch operations (PLANNED)
* Convenient API for transaction handling (PLANNED)


The [rincon_session_async] [crate] is part of the [Rincon] [ArangoDB] Rust driver project.

__Note__: A similar but synchronous API is provided by the [rincon_session] crate which is
also part of the [Rincon] project.   

## Usage

To use the asynchronous session API of this crate add this to your `Cargo.toml`:

```toml
[dependencies]
rincon_session_async = "0.1"
```

And this to your crate root:

```rust
extern crate rincon_session_async;
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
