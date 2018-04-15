
# Rincon

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Linux Build Status][travis_badge]][Travis CI]
[![Windows Build Status][appveyor_badge]][Appveyor CI]
[![codevoc.io][codecov_badge]][codecoverage]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon.svg
[docs_badge]: https://docs.rs/rincon/badge.svg
[travis_badge]: https://travis-ci.org/innoave/rincon.svg?branch=master
[appveyor_badge]: https://ci.appveyor.com/api/projects/status/github/innoave/rincon?branch=master&svg=true
[codecov_badge]: https://codecov.io/gh/innoave/rincon/branch/master/graph/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon
[documentation]: https://docs.rs/rincon
[Travis CI]: https://travis-ci.org/innoave/rincon
[Appveyor CI]: https://ci.appveyor.com/project/innoave/arangodb-rust-driver
[codecoverage]: https://codecov.io/github/innoave/rincon?branch=master
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: LICENSE
[rincon]: rincon

[Rincon] is an [ArangoDB] driver for [Rust]. It enables low level access to
[ArangoDB] in a typesafe and [Rust] idiomatic manner. 

The name [Rincon][Rincon name] is derived from the [Avocado variety list].

The vision for this project is to provide a fast and typesafe client lib for
easy and flexible use of [ArangoDB] in applications.  

**Status: Experimental**

The first usable version is released. Yuppie!

For details about this first version and how to use the Rincon ArangoDB driver
see the [README](rincon/README.md) in the [rincon] crates subdirectory.

The project is continuously evolving. There may be breaking changes in upcoming
releases. All changes will be documented in the [CHANGELOG.md](CHANGELOG.md).
Breaking changes will be marked as such.

If you are interested in using this Rincon ArangoDB driver I would be happy to
receive feedback and here what everyone is thinking about it. Especially if
you have ideas about improving usability of the driver.

Please file an issue on Github for every idea you have, the difficulties you are
facing and naturally for all bugs you find.

You may also use the Gitter channel to ask questions and discuss things.

## What's next?

There is still lot of work todo. The next planned steps are:

* Implement more operation of the [ArangoDB REST API]. Details about the 
  planned and ready operations can be found in
  [docs/arangodb_rest_api_methods.md](docs/arangodb_rest_api_methods.md)

* Implement the asynchronous session API (`rincon_session_async`)

* Work on the documentation

## Multiple Crates

The functionality of the [Rincon] [ArangoDB] Rust driver is split up into several crates for a
modular design. This modular design enforces a clean code base. Applications can flexible combine
the functionality they need by adding different combinations of the crates to their project.
Developers can easily customize and extend the functionality of the existing crates.

For an overview of crates in this project see the [README](rincon/README.md) in the [rincon] crates
subdirectory.

Each crate comes with its own README file which describes the purpose of the crate and how to use
it.

## License

Licensed under Apache License, Version 2.0<br/>
see [LICENSE] or http://www.apache.org/licenses/LICENSE-2.0 for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.


[ArangoDB]: https://www.arangodb.com
[ArangoDB REST API]: https://docs.arangodb.com/3.2/HTTP/index.html
[AQL]: https://docs.arangodb.com/3.2/AQL/index.html
[Avocado variety list]: http://www.ucavo.ucr.edu/AvocadoVarieties/VarietyFrame.html
[Rincon name]: http://ucavo.ucr.edu/avocadovarieties/VarietyList/Rincon.html
[Rust]: https://www.rust-lang.org
