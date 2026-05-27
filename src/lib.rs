// SPDX-License-Identifier: MIT OR Apache-2.0

//! # ⚠️ `serde_yml` is deprecated — migrate to a maintained alternative
//!
//! This crate is **unmaintained**. The `0.0.13` release is a thin
//! compatibility shim so existing call sites keep working while you
//! plan a migration. See [`MIGRATION.md`](https://github.com/sebastienrousseau/serde_yml/blob/master/MIGRATION.md)
//! for the full guide.
//!
//! ## Maintained alternatives
//!
//! - **[`noyalib`](https://crates.io/crates/noyalib)** — pure-Rust,
//!   `#![forbid(unsafe_code)]`, drop-in via the `compat-serde-yaml`
//!   feature (zero call-site changes for typical users).
//! - **[`serde-saphyr`](https://crates.io/crates/serde-saphyr)** —
//!   modern parser, serde-integrated typed deserialisation. **No
//!   `Value` DOM** — fits codebases that only call
//!   `from_str::<MyStruct>`.
//! - **[`yaml-rust2`](https://crates.io/crates/yaml-rust2)** —
//!   pure-Rust parser primitives, no `serde` wrapper. Fits users
//!   who were using the low-level `serde_yml::libyml` / `loader`
//!   surface (removed in this shim).
//!
//! `MIGRATION.md` carries the per-crate mapping tables.
//!
//! ## Why the shim is backed by `noyalib`
//!
//! The shim itself depends on `noyalib`'s `compat-serde-yaml`
//! feature for its implementation. This is an implementation
//! detail of the shim, not a recommendation that you must use
//! `noyalib`. Two things follow:
//!
//! - **No archived advisory chain.** The previous C-FFI parser
//!   (`libyml`) and `serde_yaml` 0.9 are gone from the dependency
//!   graph entirely; downstream `cargo audit` / `cargo deny` runs
//!   stop flagging the unmaintained chain.
//! - **Safer defaults flow through.** `#![forbid(unsafe_code)]`,
//!   YAML 1.2 strict booleans (the "Norway problem" fix), custom
//!   tags preserved as `Value::Tagged` rather than coerced.
//!
//! If you want to evaluate `noyalib` directly, the
//! `compat-serde-yaml` feature exposes the same surface this shim
//! re-exports. If you'd rather pick a different alternative,
//! `MIGRATION.md` covers `serde-saphyr` and `yaml-rust2`.
//!
//! ## Stop-gap: keep using `serde_yml = "0.0.13"`
//!
//! Existing call sites compile unchanged against this shim. Every
//! item below is marked `#[deprecated]`, so the compiler will point
//! at the spots that need updating during your migration.
//!
//! ## Removed in 0.0.13 (vs. 0.0.12)
//!
//! The deep internal modules that previous versions exposed —
//! `serde_yml::libyml`, `serde_yml::loader`, `serde_yml::modules`,
//! `serde_yml::de::{Event, Progress}`, `serde_yml::ser::SerializerConfig`,
//! `serde_yml::value::Index`, `DocumentAnchor`, `State` — are
//! **gone** in this release. They were implementation details of
//! the C-FFI parser that no longer exists. See `MIGRATION.md` for
//! the equivalence table per alternative.

#![deprecated(
    since = "0.0.13",
    note = "serde_yml is unmaintained. Migrate to a maintained alternative (noyalib, serde-saphyr, or yaml-rust2). See MIGRATION.md."
)]
#![doc(html_root_url = "https://docs.rs/serde_yml/0.0.13")]

// ── Top-level re-exports — name-for-name with serde_yml 0.0.13 ─────────

#[doc(inline)]
pub use noyalib::compat::serde_yaml::{
    from_reader, from_slice, from_str, from_value, to_string, to_value,
    to_writer, Deserializer, Error, Location, Mapping, Number, Result,
    Sequence, Serializer, Tag, TaggedValue, Value,
};

// ── Sub-modules — keep path-form imports working ───────────────────────

/// YAML value types. Re-exported from [`noyalib::compat::serde_yaml::value`].
pub mod value {
    pub use noyalib::compat::serde_yaml::value::{
        Mapping, Number, Sequence, Tag, TaggedValue, Value,
    };
}

/// YAML mapping type. Re-exported from [`noyalib::compat::serde_yaml::mapping`].
pub mod mapping {
    pub use noyalib::compat::serde_yaml::mapping::Mapping;
}

/// Serde `#[serde(with = "...")]` helpers. Re-exported from
/// [`noyalib::compat::serde_yaml::with`].
pub mod with {
    pub use noyalib::compat::serde_yaml::with::{
        nested_singleton_map, singleton_map, singleton_map_optional,
        singleton_map_recursive, singleton_map_with,
    };
}
