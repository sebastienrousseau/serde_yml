#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/serde_yml/images/favicon.ico",
    html_logo_url = "https://kura.pro/serde_yml/images/logos/serde_yml.svg",
    html_root_url = "https://docs.rs/serde_yml"
)]
#![crate_name = "serde_yml"]
#![crate_type = "lib"]

// Re-export commonly used items from modules
pub use crate::de::{
    from_reader, from_slice, from_str, Deserializer, DocumentAnchor,
};
#[doc(inline)]
pub use crate::mapping::Mapping;
pub use crate::modules::error::{Error, Location, Result};
pub use crate::ser::{to_string, to_writer, Serializer, State};
#[doc(inline)]
pub use crate::value::{
    from_value, to_value, Index, Number, Sequence, Value,
};

/// YAML deserialization module
pub mod de;
/// YAML parsing and emitting
pub mod libyml;
/// YAML loader utilities
pub mod loader;
/// YAML mappings
pub mod mapping;
/// Library modules
pub mod modules;
/// YAML numeric types
pub mod number;
/// YAML serialization module
pub mod ser;
/// YAML value representation
pub mod value;
/// YAML helper utilities
pub mod with;

// Prevent downstream code from implementing the Index trait
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl Sealed for crate::Value {}
    impl<T> Sealed for &T where T: ?Sized + Sealed {}
}

/// Specifies the version of the **serde_yml** library.
///
/// This constant is automatically aligned with the crate’s version defined in
/// `Cargo.toml`. It is commonly referenced in diagnostic logs, user-facing messages,
/// and documentation outputs, ensuring consistent visibility of the library's
/// release status.
///
/// # Examples
///
/// ```
/// // Retrieve and print the library version:
/// println!("Current serde_yml version: {}", serde_yml::VERSION);
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
