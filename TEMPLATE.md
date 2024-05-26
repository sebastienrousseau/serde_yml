# Serde YML (a fork of Serde YAML)

[![GitHub][github-badge]][05] [![Crates.io][crates-badge]][06] [![Docs.rs][docs-badge]][07] [![Codecov][codecov-badge]][08] [![Build Status][build-badge]][09]

A Rust library for using the [Serde][01] serialization framework with data in [YAML][04] file format. This project, has been renamed to [Serde YML][00] to avoid confusion with the original Serde YAML crate which is now archived and no longer maintained.

## Credits and Acknowledgements

This library is a continuation of the excellent work done by [David Tolnay][03] and the maintainers of the [serde-yaml][02] library.

While Serde YML started as a fork of serde-yaml, it has now evolved into a separate library with its own goals and direction in mind and does not intend to replace the original serde-yaml crate.

If you are currently using serde-yaml in your projects, we recommend carefully evaluating your requirements and considering the stability and maturity of the original library as well as looking at the features and improvements offered by other YAML libraries in the Rust ecosystem.

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Changelog

[00]: https://serdeyml.com
[01]: https://github.com/serde-rs/serde
[02]: https://github.com/dtolnay/serde-yaml
[03]: https://github.com/dtolnay
[04]: https://yaml.org/
[05]: https://github.com/sebastienrousseau/serde_yml
[06]: https://crates.io/crates/serde_yml
[07]: https://docs.rs/serde_yml
[08]: https://codecov.io/gh/sebastienrousseau/serde_yml
[09]: https://github.com/sebastienrousseau/serde-yml/actions?query=branch%3Amain
[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/serde_yml/release.yml?branch=master&style=for-the-badge "Build Status"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/serde_yml?style=for-the-badge&token=Q9KJ6XXL67 "Codecov"
[crates-badge]: https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge&color=fc8d62&logo=rust "Crates.io"
[docs-badge]: https://img.shields.io/badge/docs.rs-serde__yml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs "Docs.rs"
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/serde--yml-8da0cb?style=for-the-badge&labelColor=555555&logo=github "GitHub"

## What's Changed

### Enhancements

#### Forked Serde YAML

-   **Hard reset**: Hard reset to the latest @dtolnay [latest release](https://github.com/dtolnay/serde-yaml/commit/2009506d33767dfc88e979d6bc0d53d09f941c94) to keep traceability and retain commits history to the original [Serde YAML](https://github.com/dtolnay/serde-yaml) codebase and credits to the maintainers.
-   **Renaming**: This project, has been renamed to `Serde YML` to avoid confusion with the original Serde YAML crate which is now archived and no longer maintained. While `Serde YML` started as a fork of serde-yaml, it has now evolved into a separate library with its own goals and direction in mind and does not intend to replace the original serde-yaml crate.

#### CI Improvements

-   **ci(serde-yaml)**: Added a missing release workflow and made minor tweaks to the README for better clarity and documentation. This update ensures smoother and more reliable release processes.
    -   Commit: `ci(serde-yaml): :green_heart: add missing release workflow and minor tweaks in README`

#### Testing Enhancements

-   **test(serde-yaml)**: Enhanced test coverage by adding new unit tests for `mapping.rs`. These tests ensure the robustness and reliability of the `Mapping` struct and its associated methods.

    -   Commit: `test(serde-yaml): :white_check_mark: add new tests for `mapping.rs``

-   **test(serde-yaml)**: Expanded the test suite by adding comprehensive unit tests for the `ser.rs` module. The new tests cover various serialization scenarios, including scalar values, sequences, maps, nested structures, optional fields, and custom serializers.
    -   Commit: `test(serde-yaml): :white_check_mark: add unit tests for the `ser.rs` module`

**Full Changelog**: https://github.com/sebastienrousseau/serde_yml/commits/v0.0.9
