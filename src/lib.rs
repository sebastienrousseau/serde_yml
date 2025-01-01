#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/serde_yml/images/favicon.ico",
    html_logo_url = "https://kura.pro/serde_yml/images/logos/serde_yml.svg",
    html_root_url = "https://docs.rs/serde_yml"
)]
#![crate_name = "serde_yml"]
#![crate_type = "lib"]

// Re-exports organized by functionality
pub use crate::{
    // Core serialization/deserialization
    de::{
        from_reader, from_slice, from_str, Deserializer, DocumentAnchor,
    },
    // Data structures and types
    mapping::Mapping,
    // Error handling
    modules::error::{Error, Location, Result},
    ser::{to_string, to_writer, Serializer, State},

    value::{from_value, to_value, Index, Number, Sequence, Value},
};

// ------------------------------------------------------------
// Core serialization/deserialization functionality
// ------------------------------------------------------------

/// YAML deserialisation module
pub mod de;
/// YAML serialisation module
pub mod ser;

// ------------------------------------------------------------
// Data representation
// ------------------------------------------------------------

/// YAML mappings
pub mod mapping;
/// YAML numeric types
pub mod number;
/// YAML value representation
pub mod value;

// ------------------------------------------------------------
// Implementation internals
// ------------------------------------------------------------
/// YAML parsing and emitting
pub mod libyml;
/// YAML loader utilities
pub mod loader;
/// Library modules
pub mod modules;

// ------------------------------------------------------------
// Helper utilities
// ------------------------------------------------------------
/// YAML helper utilities
pub mod with;

// Private implementation details
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl Sealed for crate::Value {}
    impl<T> Sealed for &T where T: ?Sized + Sealed {}
}

// ------------------------------------------------------------
// Version information
// ------------------------------------------------------------

/// Current version of the Serde YML library.
///
/// This constant is automatically aligned with the crate's version defined in
/// `Cargo.toml`. It is commonly referenced in diagnostic logs, user-facing messages,
/// and documentation outputs.
///
/// # Examples
///
/// ```
/// println!("Serde YML version: {}", serde_yml::VERSION);
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
