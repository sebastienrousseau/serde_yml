//! Examples covering the `Value` surface that the shim still
//! supports. `index_examples` was removed because it exercised the
//! `serde_yml::value::Index` sealed trait, which is no longer
//! exposed by the shim — `noyalib::Value` implements `Index<&str>`
//! and `Index<usize>` natively; see `MIGRATION.md`.

pub(crate) mod de_examples;

pub(crate) fn main() {
    de_examples::main();
}
