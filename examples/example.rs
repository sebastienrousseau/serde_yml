//! # serde_yml examples (deprecation shim)
//!
//! `serde_yml` is deprecated; the examples below exercise the public
//! surface that the 0.0.13 shim continues to provide. The original
//! example suite included demonstrations of the C-FFI parser
//! (`libyml`), the low-level `loader::Loader`, and the
//! `modules::path::Path` error helper — all three were removed
//! because they have no Rust-only equivalent in `noyalib`. See
//! `MIGRATION.md` for the noyalib equivalents.
//!
//! Run with: `cargo run --example example`.

#![allow(deprecated)]

mod serializer;
mod value;
mod with;

fn main() {
    serializer::main();
    value::main();
    with::main();
}
