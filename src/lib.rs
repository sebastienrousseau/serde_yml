//! # Serde YML
//!
//! [![Serde YML Logo](https://kura.pro/serde_yml/images/banners/banner-serde_yml.svg)](https://serde_yml.one "Serde YML: Seamless YAML Serialization for Rust")
//!
//! ## Seamless YAML Serialization for [Rust][rust-lang]
//!
//! [![Crates.io](https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge&color=success&labelColor=27A006)][crates-io]
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.9-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)][lib-rs]
//! [![License](https://img.shields.io/crates/l/serde_yml.svg?style=for-the-badge&color=007EC6&labelColor=03589B)][license]
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)][rust-lang]
//!
//! [Serde YML][serde-yml] is a Rust library that simplifies YAML serialization and deserialization using the popular [Serde][serde] framework. It provides a convenient and efficient way to convert Rust data structures to YAML format and vice versa.
//!
//! ## Features
//!
//! - Serialization and deserialization of Rust data structures to/from YAML format
//! - Support for custom structs and enums using Serde's derive macros
//! - Handling of YAML's `!tag` syntax for representing enum variants
//! - Direct access to YAML values through the `Value` type and related types like `Mapping` and `Sequence`
//! - Comprehensive error handling with `Error`, `Location`, and `Result` types
//! - Serialization to YAML using `to_string` and `to_writer` functions
//! - Deserialization from YAML using `from_str`, `from_slice`, and `from_reader` functions
//! - Customizable serialization and deserialization behavior using Serde's `#[serde(with = ...)]` attribute
//! - Support for serializing/deserializing enums using a YAML map with a single key-value pair through the `singleton_map` module
//! - Recursive application of `singleton_map` serialization/deserialization to all enums within a data structure using the `singleton_map_recursive` module
//! - Serialization and deserialization of optional enum fields using the `singleton_map_optional` module
//! - Handling of nested enum structures with optional inner enums using the `singleton_map_recursive` module
//! - Customization of serialization and deserialization logic for enums using the `singleton_map_with` module and custom helper functions
//!
//! ## Rust Version Compatibility
//!
//! This library is compatible with Rust 1.60 and above.
//!
//! ## Installation
//!
//! Add the following dependency to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! serde_yml = "0.0.9"
//! ```
//!
//! ## Usage
//!
//! Serde YML offers a straightforward and intuitive API for working with YAML data in Rust. Here's a quick example of how to serialize and deserialize a Rust type:
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize,Debug,PartialEq)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! fn main() -> Result<(), serde_yml::Error> {
//!     let point = Point { x: 1.0, y: 2.0 };
//!
//!     // Serialize to YAML
//!     let yaml = serde_yml::to_string(&point)?;
//!     assert_eq!(yaml, "x: 1.0\n'y': 2.0\n");
//!
//!     // Deserialize from YAML
//!     let deserialized_point: Point = serde_yml::from_str(&yaml)?;
//!     assert_eq!(point, deserialized_point);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! Serde YML provides a set of comprehensive examples to demonstrate its usage and capabilities. You can find them in the `examples` directory of the project.
//!
//! To run the examples, clone the repository and execute the following command in your terminal from the project root directory:
//!
//! ```shell
//! cargo run --example example
//! ```
//!
//! The examples cover various scenarios, including serializing and deserializing structs, enums, optional fields, custom structs, and more.
//!
//! ## Best Practices and Common Pitfalls
//!
//! - When serializing large datasets, consider using `serde_yml::to_writer` to write the YAML output directly to a file or a writer instead of keeping the entire serialized string in memory.
//! - Be cautious when deserializing untrusted YAML input, as it may contain unexpected or malicious data. Always validate and handle the deserialized data appropriately.
//! - When working with custom structs or enums, ensure that they implement the necessary Serde traits (`Serialize` and `Deserialize`) for proper serialization and deserialization.
//! - If you encounter any issues or have questions, refer to the library's documentation and examples for guidance. If the problem persists, consider opening an issue on the library's [GitHub repository][repo].
//!
//! ## Credits and Acknowledgements
//!
//! Serde YML draws inspiration from the excellent work done by [David Tolnay][dtolnay] and the maintainers of the [serde-yaml][serde-yaml] library. While Serde YML started as a fork of serde-yaml, it has now evolved into a separate library with its own goals and direction in mind.
//!
//! If you are currently using serde-yaml in your projects, we recommend carefully evaluating your requirements and considering the stability and maturity of the original library before migrating to Serde YML.
//!
//! Finally, we would like to express our sincere gratitude to [David Tolnay][dtolnay] and the [serde-yaml][serde-yaml] team for their valuable contributions to the Rust community and for inspiring this project.
//!
//! [serde-yml]: https://serdeyml.com "Serde YML"
//! [serde]: https://github.com/serde-rs/serde
//! [rust-lang]: https://www.rust-lang.org/ "Rust"
//! [dtolnay]: https://github.com/dtolnay "David Tolnay"
//! [serde-yaml]: https://github.com/dtolnay/serde-yaml "Serde YAML"
//! [crates-io]: https://crates.io/crates/serde_yml "Crates.io"
//! [lib-rs]: https://lib.rs/crates/serde_yml "Lib.rs"
//! [license]: https://opensource.org/license/apache-2-0/ "MIT or Apache License, Version 2.0"
//! [repo]: https://github.com/your-repo/serde_yml "Serde YML Repository"
//!
#![deny(missing_docs)]
#![doc(
    html_favicon_url = "https://kura.pro/serde_yml/images/favicon.ico",
    html_logo_url = "https://kura.pro/serde_yml/images/logos/serde_yml.svg",
    html_root_url = "https://docs.rs/serde_yml"
)]
#![crate_name = "serde_yml"]
#![crate_type = "lib"]

// Re-export commonly used items from other modules
pub use crate::de::{from_reader, from_slice, from_str, Deserializer}; // Deserialization functions
pub use crate::modules::error::{Error, Location, Result}; // Error handling types
pub use crate::ser::{to_string, to_writer, Serializer, State}; // Serialization functions
#[doc(inline)]
pub use crate::value::{
    from_value, to_value, Index, Number, Sequence, Value,
}; // Value manipulation functions

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// The `utilities` module contains utility functions for the library.
pub mod utilities;

#[doc(inline)]
pub use crate::mapping::Mapping; // Re-export the Mapping type for YAML mappings

/// The `de` module contains the library's YAML deserializer.
pub mod de;

/// The `libyml` module contains the library's YAML parser and emitter.
pub mod libyml;

/// The `loader` module contains the `Loader` type for YAML loading.
pub mod loader;

/// The `mapping` module contains the `Mapping` type for YAML mappings.
pub mod mapping;

/// The `modules` module contains the library's modules.
pub mod modules;

/// The `number` module contains the `Number` type for YAML numbers.
pub mod number;

/// The `ser` module contains the library's YAML serializer.
pub mod ser;

/// The `value` module contains the `Value` type for YAML values.
pub mod value;

/// The `with` module contains the `With` type for YAML values.
pub mod with;

// Prevent downstream code from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl Sealed for crate::Value {}
    impl<T> Sealed for &T where T: ?Sized + Sealed {}
}
