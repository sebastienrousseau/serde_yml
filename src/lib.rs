// SPDX-License-Identifier: MIT OR Apache-2.0

//! # ⚠️ `serde_yml` is deprecated — migrate to [`noyalib`](https://crates.io/crates/noyalib)
//!
//! This crate is **unmaintained**. The `0.0.14` release is a thin
//! compatibility shim that forwards every call to [`noyalib`], an
//! actively-maintained, pure-Rust YAML library with
//! `#![forbid(unsafe_code)]` enforced across the entire workspace.
//!
//! ## Why migrate?
//!
//! - **Maintained.** `serde_yml` has been archived. `noyalib` is
//!   actively developed; security advisories and YAML 1.2
//!   corrections flow into it.
//! - **Safe.** `noyalib` forbids `unsafe` code. The original
//!   `serde_yml` shipped FFI bindings to the C `libyaml` parser via
//!   `libyml`; this shim removes that dependency entirely.
//! - **Faster.** `noyalib` outpaces `serde_yaml_ng` (the most active
//!   `serde_yaml` fork) by 39–64 % on representative workloads.
//! - **YAML 1.2 spec-compliant.** Passes 406/406 cases in the
//!   official YAML 1.2 test suite.
//! - **No archived advisory chain.** The shim depends on `noyalib`,
//!   not on `serde_yaml` 0.9 or `libyml` — your downstream
//!   `cargo audit` / `cargo deny` runs no longer flag the
//!   unmaintained chain.
//!
//! ## Recommended: switch directly to `noyalib`
//!
//! ```toml
//! # Cargo.toml
//! - serde_yml = "0.0"
//! + noyalib = { version = "0.0.5", features = ["compat-serde-yaml"] }
//! ```
//!
//! ```rust,ignore
//! - use serde_yml::{from_str, to_string, Value};
//! + use noyalib::compat::serde_yaml::{from_str, to_string, Value};
//! ```
//!
//! See [`MIGRATION.md`](https://github.com/sebastienrousseau/serde_yml/blob/master/MIGRATION.md)
//! for the full mapping table.
//!
//! ## Stop-gap: keep using `serde_yml = "0.0.14"`
//!
//! Existing call sites compile unchanged against this shim. Every
//! item below is marked `#[deprecated]`, so the compiler will point
//! at the spots that need updating during your migration.
//!
//! ## Removed in 0.0.14
//!
//! The deep internal modules that previous versions exposed —
//! `serde_yml::libyml`, `serde_yml::loader`, `serde_yml::modules`,
//! `serde_yml::de::{Event, Progress}`, `serde_yml::ser::SerializerConfig`,
//! `serde_yml::value::Index`, `DocumentAnchor`, `State` — are
//! **gone** in this release. They were implementation details of
//! the C-FFI parser that no longer exists. Migrate to `noyalib`
//! directly; it offers equivalent (and safer) functionality. See
//! `MIGRATION.md` for the equivalence table.

#![deprecated(
    since = "0.0.14",
    note = "serde_yml is unmaintained. Migrate to `noyalib` (https://crates.io/crates/noyalib). See MIGRATION.md."
)]
#![doc(html_root_url = "https://docs.rs/serde_yml/0.0.14")]

// ── Top-level re-exports — name-for-name with serde_yml 0.0.13 ─────────

#[doc(inline)]
pub use noyalib::compat::serde_yaml::{
    from_reader, from_slice, from_str, from_value, to_string, to_value, to_writer, Deserializer,
    Error, Location, Mapping, Number, Result, Sequence, Serializer, Tag, TaggedValue, Value,
};

// ── Sub-modules — keep path-form imports working ───────────────────────

/// YAML value types. Re-exported from [`noyalib::compat::serde_yaml::value`].
pub mod value {
    pub use noyalib::compat::serde_yaml::value::{Mapping, Number, Sequence, Tag, TaggedValue, Value};
}

/// YAML mapping type. Re-exported from [`noyalib::compat::serde_yaml::mapping`].
pub mod mapping {
    pub use noyalib::compat::serde_yaml::mapping::Mapping;
}

/// Serde `#[serde(with = "...")]` helpers. Re-exported from
/// [`noyalib::compat::serde_yaml::with`].
pub mod with {
    pub use noyalib::compat::serde_yaml::with::{
        nested_singleton_map, singleton_map, singleton_map_optional, singleton_map_recursive,
        singleton_map_with,
    };
}
