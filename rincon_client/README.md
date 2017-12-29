
# Rincon Client

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon_client.svg
[docs_badge]: https://docs.rs/rincon_client/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon_client
[documentation]: https://docs.rs/rincon_client
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: ../LICENSE
[rincon]: https://github.com/innoave/rincon
[rincon_client]: https://github.com/innoave/rincon/rincon_client

The [rincon_client] [crate] provides types and functions to interact with the REST API of the
[ArangoDB] server.

The REST API of [ArangoDB] comprises a lot of methods. An overview of the currently implemented
methods can be found [here](../docs/arangodb_rest_api_methods.md).

The [rincon_client] [crate] is part of the [Rincon] [ArangoDB] Rust driver project.

## Usage

### Crate Features

The [rincon_client] [crate] can be compiled with optional features to adapt to the configuration
of the [ArangoDB] server to be used.

The provided crate features are:

* `mmfiles` : support for MMFiles storage engine specific features (default)
* `rocksdb` : support for RocksDB storage engine specific features (optional)
* `cluster` : support for cluster specific features (optional)
* `enterprise` : support for [ArangoDB] enterprise specific features (optional)

Note1: A deployed [ArangoDB] server uses either MMFiles or RocksDB storage
       engine. Therefore this crate must be compiled either with the
       `mmfiles` feature enabled or the `rocksdb` feature, but not both.
         
Note2: If [rincon_client] is compiled with the `cluster` feature some API
       methods which return cluster specific fields do not work with an
       [ArangoDB] server that is not configured in a cluster. This is due to
       the [ArangoDB] server does not return cluster specific fields in a 
       single server configuration.

### Examples

**Single server with MMFiles storage engine**

By default [rincon_client] is compiled with support for single server
configurations using the MMFiles storage engine.

To use this crate with the default features add this to your `Cargo.toml`:

```toml
[dependencies]
rincon_client = "0.1"
```

This is equivalent to:

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["mmfiles"] }
```

**Using RocksDB storage engine**

If the [ArangoDB] server is configured to use the RocksDB storage engine,
[rincon_client] should be compiled with the `rocksdb` feature to support
RocksDB specific attributes and fields within the API methods.

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["rocksdb"] }
```

**Using an [ArangoDB] Cluster**

To use the [ArangoDB] cluster specific features of the API, [rincon_client]
must be compiled with the `cluster` feature enabled.

To use a clustered server with MMFiles storage engine and enterprise features
add this to your dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["cluster"] }
```

To use a clustered server with RocksDB storage engine add this to your
dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["rocksdb", "cluster"] }
```

**Using [ArangoDB] Enterprise features**

To add support for [ArangoDB] enterprise features in the client API add this to
your dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["enterprise"] }
```

And with RocksDB storage engine instead of MMFiles:

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["rocksdb", "enterprise"] }
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
