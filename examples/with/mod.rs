//! Examples for the `with::singleton_map*` enum-representation
//! helpers. The `singleton_map_with` family was removed: noyalib's
//! compat layer exposes `serialize_with` / `deserialize_with`
//! taking an explicit transform function, not the bare
//! `serialize` / `deserialize` aliases the original
//! `serde_yml::with::singleton_map_with` provided. Use
//! `noyalib::compat::serde_yaml::with::singleton_map_with` directly
//! if you need this surface.

pub(crate) mod nested_singleton_map;
pub(crate) mod singleton_map;
pub(crate) mod singleton_map_enum_variants;
pub(crate) mod singleton_map_optional;
pub(crate) mod singleton_map_recursive;
pub(crate) mod singleton_map_recursive_deep_nesting;
pub(crate) mod singleton_map_recursive_optional;
pub(crate) mod singleton_map_recursive_serialize_deserialize;
pub(crate) mod singleton_map_recursive_with;

pub(crate) fn main() {
    singleton_map::main();
    singleton_map_recursive::main();
    singleton_map_enum_variants::main();
    singleton_map_recursive_deep_nesting::main();
    singleton_map_recursive_serialize_deserialize::main();
    singleton_map_optional::main();
    singleton_map_recursive_optional::main();
    singleton_map_recursive_with::main();
    nested_singleton_map::main();
}
