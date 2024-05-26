//! This is the main function for the build script.
//!
//! Currently, it only instructs Cargo to re-run this build script if `build.rs` is changed.
fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");
}
