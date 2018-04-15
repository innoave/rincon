
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
[Rincon project]: https://github.com/innoave/rincon
[license]: ../LICENSE
[rincon]: ../rincon
[rincon_core]: ../rincon_core
[rincon_connector]: ../rincon_connector
[rincon_client]: ../rincon_client

**Type safe interaction with the ArangoDB REST API**

The [rincon_client] [crate] provides types and functions to interact with the REST API of the
[ArangoDB] server.

In [rincon_client] the REST methods are represented by structs. A method is instantiated with the 
desired parameters and data to get a method call. The method call is executed against an [ArangoDB] 
server on a connection provided by a connector. This concept allows applications to queue, 
distribute or batch process method calls.

For example, inserting a new document into an existing collection looks like:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Customer {
    name: String,
    age: u8,
}

let customer = Customer {
    name: "Jane Doe".to_owned(),
    age: 42,
};

// create a new document with the customer struct as content
let new_document = NewDocument::from_content(customer);

// create the method call to insert new_document into the 'customers' collection.
let method = InsertDocument::new("customers", new_document);

// execute the method call
let document = core.run(connection.execute(method)).unwrap();
```

The REST API of [ArangoDB] comprises a lot of methods. An overview of the currently implemented
methods can be found [here](../docs/arangodb_rest_api_methods.md).

The [rincon_client] [crate] is part of the [Rincon ArangoDB Rust driver project][Rincon project].

## Usage

### Crate Features

The [rincon_client] [crate] can be compiled with optional features to adapt to the configuration
of the [ArangoDB] server to be used. These optional features support attributes on the method calls
and their results that are specific to the related [ArangoDB] configuration.

The provided crate features are:

* `mmfiles` : support for MMFiles storage engine specific attributes (optional)
* `rocksdb` : support for RocksDB storage engine specific attributes (optional)
* `cluster` : support for cluster specific attributes (optional)
* `enterprise` : support for [ArangoDB] enterprise specific attributes (optional)

Note1: A deployed [ArangoDB] server uses either MMFiles or RocksDB storage
       engine. Therefore only one of the features `mmfiles` and `rocksdb`
       features may be activated, but not both.
         
Note2: If [rincon_client] is compiled with the `cluster` feature some API
       methods which return cluster specific fields do not work with an
       [ArangoDB] server that is not configured in a cluster. This is due to
       the [ArangoDB] server does not return cluster specific fields in a 
       single server configuration.
       
It is not necessary to activate any of the optional crate features if an
application does not need to access the feature related attributes.

### Examples

**Using MMFiles storage engine**

If you want to make use of the MMFiles related attributes and the [ArangoDB]
server is configured to use the MMFiles storage engine, [rincon_client] can be
compiled with the `mmfiles` feature to support MMFiles specific attributes
and fields within the API methods.

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["mmfiles"] }
```

**Using RocksDB storage engine**

If the [ArangoDB] server is configured to use the RocksDB storage engine,
[rincon_client] can be compiled with the `rocksdb` feature to support
RocksDB specific attributes and fields within the API methods.

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["rocksdb"] }
```

**Using an [ArangoDB] Cluster**

To use the [ArangoDB] cluster specific features of the API, [rincon_client]
must be compiled with the `cluster` feature enabled.

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["cluster"] }
```

**Using [ArangoDB] Enterprise features**

To add support for [ArangoDB] enterprise features in the client API add this to
your dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["enterprise"] }
```

**Using an [ArangoDB] Cluster with Enterprise features**

The optional features may be combined, but only one storage engine feature may
be enabled at a time.

To use enterprise, cluster and MMFiles specific features add this to your
dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["mmfiles", "enterprise", "cluster"] }
```

To use enterprise, cluster and RocksDB specific features add this to your
dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["rocksdb", "enterprise", "cluster"] }
```

### Connector and core types

In any case we also need a connector like one provided by the [rincon_connector] crate and some
types defined in the [rincon_core] crate. This means we need to add additional dependencies:

```toml
[dependencies]
rincon_core = "0.1"
rincon_connector = "0.1"
# plus dependency as explained above
rincon_client = "0.1" 
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
