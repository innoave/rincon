
# Rincon AQL

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon_aql.svg
[docs_badge]: https://docs.rs/rincon_aql/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon_aql
[documentation]: https://docs.rs/rincon_aql
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: ../LICENSE
[rincon]: https://github.com/innoave/rincon
[rincon_aql]: https://github.com/innoave/rincon/rincon_aql

The [rincon_aql] [crate] aims to easily build [AQL] queries in a typesafe way. Composing queries in a
typesafe manner has some great advantages:

* The compiler checks for correct syntax of [AQL] queries.
* The autocompletion feature of an editor or IDE gives aid on the [AQL] syntax.
* Refactoring of document fields are applied to [AQL] queries as well.

The [rincon_aql] [crate] is part of the [Rincon] [ArangoDB] Rust driver project.

**Status: Experimental**

**This crate is mainly an idea that is planned to be implemented after other crates of the [Rincon]
project have reached a major milestone.** 


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
