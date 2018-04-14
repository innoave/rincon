
# Rincon - ArangoDB Rust driver

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon.svg
[docs_badge]: https://docs.rs/rincon/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon
[documentation]: https://docs.rs/rincon
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: ../LICENSE
[Rincon project]: https://github.com/innoave/rincon
[rincon]: ../rincon
[rincon_core]: ../rincon_core
[rincon_connector]: ../rincon_connector
[rincon_client]: ../rincon_client
[rincon_session]: ../rincon_session
[rincon_session_async]: ../rincon_session_async
[rincon_aql]: ../rincon_aql
[rincon_test_helper]: ../rincon_test_helper

[rincon_core API]: https://docs.rs/rincon_core
[rincon_connector API]: https://docs.rs/rincon_connector
[rincon_client API]: https://docs.rs/rincon_client
[rincon_session API]: https://docs.rs/rincon_session
[rincon_session_async API]: https://docs.rs/rincon_session_async
[rincon_aql API]: https://docs.rs/rincon_aql
[rincon_test_helper API]: https://docs.rs/rincon_test_helper

The [rincon] [crate] itself does not provide any functionality. It is meant as an entry point
for the [Rincon project] which aims to provide a complete [ArangoDB] driver for [Rust].

The [Rincon project] provides several crates. You can choose which functionality you want for your
project and which level of abstraction you prefer. The separation into several crates allows to only
import those crates that are needed for your project.

The provided crates are:

* [rincon_core] : Defines the common API for the driver and is used by the other crates.
* [rincon_connector] : Implements the communication layer of the driver.
* [rincon_client] : Implements the methods of the REST API provided by [ArangoDB].
* [rincon_session] : Provides a synchronous higher level API on top of [rincon_client].
* [rincon_session_async] : Provides an asynchronous higher level API on top of [rincon_client].
* [rincon_aql] : Provides a DSL to build [AQL] queries in a typesafe manner. 
* [rincon_test_helper] : Provides utilities used in integration tests with an [ArangoDB] server.

Here is diagram that depicts the dependencies between the crates:

![Crate dependency structure](../docs/crate_structure.png)

## Important: Status of the project

**Currently not all crates listed above are released yet**

The ready and released crates are:

* [rincon_core] : Defines the common API for the driver and is used by the other crates.
* [rincon_connector] : Implements the communication layer of the driver.
* [rincon_client] : Implements the methods of the REST API provided by [ArangoDB].
* [rincon_session] : Provides a synchronous higher level API on top of [rincon_client].
* [rincon_test_helper] : Provides utilities used in integration tests with an [ArangoDB] server.

Crates which are planned but are not ready yet:

* [rincon_session_async] : Provides an asynchronous higher level API on top of [rincon_client].
* [rincon_aql] : Provides a DSL to build [AQL] queries in a typesafe manner. 

## Which crates should I add as dependencies to my project? 

First choose whether you want to use the lower level [rincon_client API] or the higher level 
[rincon_session_async API]. Both of these APIs are asynchronous. That is the methods return
`Future`s (of the Tokio project). If you prefer a synchronous API you might want to use the
[rincon_session API].

Additionally you need a so called connector to communicate with the [ArangoDB] server. Connectors
are provided by the [rincon_connector] crate.

To use the low level client API with a provided connector add this to your `Cargo.toml`:

```toml
[dependencies]
rincon_core = "0.1"
rincon_connector = "0.1"
rincon_client = "0.1"
```

__Note__: The [rincon_client] crate provides several optional crate features.
See the [README](../rincon_client/README.md) of this crate for a list of its crate features and
how to use them.

To use the synchronous session API with a provided connector add this to your `Cargo.toml`:

```toml
[dependencies]
rincon_core = "0.1"
rincon_connector = "0.1"
rincon_session = "0.1"
```

__Note__: As [rincon_session] builds on top of [rincon_client] it re-exports the optional crate
features of [rincon_client]. Thus the same features of [rincon_client] may be applied to the
dependency definition of the [rincon_session] crate as well.
See the [README](../rincon_client/README.md) of the [rincon_client] crate for a list of features and
how to use them.

To use the asynchronous session API with a provided connector add this to your `Cargo.toml`:

```toml
[dependencies]
rincon_core = "0.1"
rincon_connector = "0.1"
rincon_session_async = "0.1"
```

__Note__: As [rincon_session_async] builds on top of [rincon_client] it re-exports the optional crate
features of [rincon_client]. Thus the same features of [rincon_client] may be applied to the
dependency definition of the [rincon_session_async] crate as well.
See the [README](../rincon_client/README.md) of the [rincon_client] crate for a list of features and
how to use them.

With the minimal dependencies described above you can write AQL queries as strings. To make use of
the typesafe AQL query builder add this dependency to your `Cargo.toml`: 

```toml
[dependencies]
rincon_aql = "0.1"
#.. plus rincon dependencies as described above
```

In applications you will not use the [rincon_core] crate directly. But if you want to implement your
own connector instead of using one of the provided ones the custom implementation shall depend on
[rincon_core] only. [rincon_core] is also needed to implement a method of the [ArangoDB] REST API
that is not provided by [rincon_client] yet.

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
