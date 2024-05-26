<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/serde_yml/images/logos/serde_yml.webp"
alt="Serde YML logo" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->
# Serde YML: Seamless YAML Serialization for Rust

Serde YML is a Rust library that simplifies YAML serialization and deserialization using Serde. Effortlessly convert Rust types to YAML and vice versa. Supports custom structs, enums, and error handling.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Banner of Serde YML][banner]

[![Made With Rust][made-with-rust-badge]][08] [![Crates.io][crates-badge]][05] [![Lib.rs][libs-badge]][07] [![Docs.rs][docs-badge]][06] [![License][license-badge]][02] [![Codecov][codecov-badge]][09]

• [Website][01] • [Documentation][06] • [Report Bug][03] • [Request Feature][03] • [Contributing Guidelines][04]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

<!-- markdownlint-enable MD033 -->

![divider][divider]

## Overview

`Serde YML` is a robust Rust library that simplifies the serialization and deserialization of Rust data structures to and from YAML format using the widely-used Serde framework. With Serde YML, you can effortlessly convert your Rust types into YAML strings and vice versa, streamlining the process of storing, transmitting, and manipulating structured data.providing style guides for your library.

## Features

- Serialize Rust data structures to YAML format
- Deserialize YAML data into Rust types
- Support for custom structs and enums using Serde's derive macros
- Handling of YAML's `!tag` syntax for representing enum variants
- Direct access to YAML values through the `Value` type and related types
- Comprehensive error handling with `Error`, `Location`, and `Result` types
- Well-documented with examples and explanations

## Changelog 📚

[01]: https://serdeyml.com "Serde YML Website"
[02]: http://opensource.org/licenses/MIT "MIT license"
[03]: https://github.com/sebastienrousseau/serde_yml/issues "Issues"
[04]: https://github.com/sebastienrousseau/serde_yml/blob/main/CONTRIBUTING.md "Contributing"
[05]: https://crates.io/crates/serde_yml "Serde YML on crates.io"
[06]: https://docs.rs/serde_yml "Serde YML on docs.rs"
[07]: https://lib.rs/crates/serde_yml "Serde YML on lib.rs"
[08]: https://www.rust-lang.org "The Rust Programming Language"
[09]: https://codecov.io/gh/sebastienrousseau/serde_yml "Serde YML on Codecov"

[banner]: https://kura.pro/serde_yml/images/titles/title-serde_yml.svg "Serde YML Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/serde_yml?style=for-the-badge&token=Q9KJ6XXL67 "Codecov Badge"
[crates-badge]: https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge "Crates.io Badge"
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/serde_yml.svg?style=for-the-badge "Docs.rs Badge"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.7-orange.svg?style=for-the-badge "Lib.rs Badge"
[license-badge]: https://img.shields.io/crates/l/serde_yml.svg?style=for-the-badge "License Badge"
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust "Made With Rust Badge"
