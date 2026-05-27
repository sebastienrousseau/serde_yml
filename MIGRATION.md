# Migrating off `serde_yml`

`serde_yml` is unmaintained. The `0.0.13` release is a thin
compatibility shim that forwards every call to
[`noyalib`](https://crates.io/crates/noyalib), which is the
recommended replacement.

This document describes the two valid paths.

---

## Path A — Switch directly to `noyalib` (recommended)

### 1. Update `Cargo.toml`

```diff
- serde_yml = "0.0"
+ noyalib = { version = "0.0.5", features = ["compat-serde-yaml"] }
```

### 2. Search-and-replace imports

```diff
- use serde_yml::{from_str, to_string, Value};
+ use noyalib::compat::serde_yaml::{from_str, to_string, Value};
```

Or rename at the import site for a one-line diff:

```rust
use noyalib::compat::serde_yaml as serde_yml;
```

### 3. Rebuild

That is the entire migration for the typical `from_str` /
`to_string` / `Value` / `Mapping` / `with::singleton_map` codebase.

---

## Path B — Stay on `serde_yml = "0.0.13"` (stop-gap)

If you cannot migrate right now, the shim keeps your code compiling.
You get noyalib's safe parser transparently, and the compiler emits
a deprecation warning at every call site so you can budget the work.

```toml
[dependencies]
serde_yml = "0.0.13"
```

No code changes required.

---

## Public-surface mapping

The common surface is preserved name-for-name:

| `serde_yml` (≤ 0.0.12)              | `serde_yml` 0.0.13 shim             | Direct `noyalib` equivalent                       |
| ----------------------------------- | ----------------------------------- | ------------------------------------------------- |
| `serde_yml::from_str`               | unchanged                           | `noyalib::compat::serde_yaml::from_str`           |
| `serde_yml::from_slice`             | unchanged                           | `noyalib::compat::serde_yaml::from_slice`         |
| `serde_yml::from_reader`            | unchanged                           | `noyalib::compat::serde_yaml::from_reader`        |
| `serde_yml::from_value`             | unchanged                           | `noyalib::compat::serde_yaml::from_value`         |
| `serde_yml::to_string`              | unchanged                           | `noyalib::compat::serde_yaml::to_string`          |
| `serde_yml::to_writer`              | unchanged                           | `noyalib::compat::serde_yaml::to_writer`          |
| `serde_yml::to_value`               | unchanged                           | `noyalib::compat::serde_yaml::to_value`           |
| `serde_yml::Value`                  | unchanged                           | `noyalib::compat::serde_yaml::Value`              |
| `serde_yml::Mapping`                | unchanged                           | `noyalib::compat::serde_yaml::Mapping`            |
| `serde_yml::Number`                 | unchanged                           | `noyalib::compat::serde_yaml::Number`             |
| `serde_yml::Sequence`               | unchanged                           | `noyalib::compat::serde_yaml::Sequence`           |
| `serde_yml::Error` / `Location`     | unchanged                           | `noyalib::compat::serde_yaml::{Error, Location}`  |
| `serde_yml::Deserializer`           | unchanged                           | `noyalib::compat::serde_yaml::Deserializer`       |
| `serde_yml::Serializer`             | unchanged                           | `noyalib::compat::serde_yaml::Serializer`         |
| `serde_yml::value::*`               | unchanged                           | `noyalib::compat::serde_yaml::value::*`           |
| `serde_yml::mapping::*`             | unchanged                           | `noyalib::compat::serde_yaml::mapping::*`         |
| `serde_yml::with::singleton_map*`   | unchanged                           | `noyalib::compat::serde_yaml::with::*`            |

---

## Removed in 0.0.13

The deep internal modules that previous versions exposed leaked
implementation details of the C-FFI parser. They are **removed** in
the shim. If your code depends on any of these, migrate to the
`noyalib` equivalent listed below.

| Removed from `serde_yml`                 | What it was                          | `noyalib` equivalent                                                                                    |
| ---------------------------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------- |
| `serde_yml::libyml::*`                   | Raw FFI bindings to C `libyaml`      | None — `noyalib` is pure Rust and exposes no FFI surface. Reach for `noyalib`'s public API instead.     |
| `serde_yml::loader::Loader`              | Low-level YAML event loader          | Use `noyalib::load_all_as::<T>` or the streaming `noyalib::de::Deserializer` directly.                  |
| `serde_yml::de::Event`                   | Re-export of the libyml event enum   | `noyalib`'s streaming API (`noyalib::de::*`) covers the same use cases without exposing parser events.  |
| `serde_yml::de::Progress`                | Input cursor for `Loader`            | Not needed — `noyalib`'s reader-based API handles cursoring internally.                                 |
| `serde_yml::de::DocumentAnchor`          | Anchor-resolution helper             | `noyalib` resolves anchors transparently during deserialisation. See `noyalib::Value` anchor docs.      |
| `serde_yml::ser::SerializerConfig`       | C-emitter configuration              | `noyalib::ser::Config` (see `noyalib`'s `Serializer` docs for the safe-Rust equivalent).                |
| `serde_yml::ser::State`                  | C-emitter state machine handle       | None — `noyalib`'s `Serializer` does not expose internal state.                                         |
| `serde_yml::modules::path::Path`         | Error-path builder                   | `noyalib::Error` carries path info directly; use `Error::location()` / `Error::path()`.                 |
| `serde_yml::number::*`                   | Number-parsing helpers               | Use `noyalib::Number` directly (it offers the same API).                                                |
| `serde_yml::value::Index`                | Sealed trait for `Value` indexing    | `noyalib::Value` implements `std::ops::Index<&str>` / `<usize>` natively.                               |

If you were using any of these and need migration help that this
document does not cover, open an issue on
[`noyalib`](https://github.com/sebastienrousseau/noyalib/issues) —
not on `serde_yml` (this repository is archived).

---

## Behavioural notes

Two behaviours in `noyalib` are intentionally safer than the
original `serde_yml` defaults. They flow through the shim:

1. **Custom-tag scalars surface as `Value::Tagged`** rather than
   being silently coerced to the inner string. Code that
   exhaustively matched the previous six-variant `Value` enum needs
   either a `Value::Tagged(_)` arm or a call to
   `Value::untag()` / `Value::untag_ref()` before the match.

2. **YAML 1.2 strict booleans by default.** `country: NO` stays
   `"NO"` (the YAML 1.2 fix to the "Norway problem") instead of
   becoming `false`. Opt back into YAML 1.1 resolver semantics via
   `noyalib::ParserConfig::version(noyalib::YamlVersion::V1_1)` if
   your pipeline depended on the legacy boolean recognition.

Both are documented in `noyalib`'s own migration guide alongside the
opt-outs.

---

## MSRV change

`serde_yml 0.0.13` requires **Rust 1.85.0** (noyalib's MSRV); the
previous releases required 1.56. Users who cannot move past 1.56
should pin `serde_yml = "=0.0.12"` and plan a migration window.

---

## Test and example coverage in 0.0.13

The shim is wire-compatible with typical user code (`from_str` /
`to_string` / `Value` round-trips work transparently), but the
original library's own unit tests and examples verified the *old
implementation's internal shape* — `Mapping`'s internal `map`
field, the streaming `Serializer::new(writer)` constructor, the
C-FFI `libyml` module, the `loader::Loader` event walker — which
the noyalib backend does not expose at the same shape.

### Tests retained (1 file, 9 tests, all pass)

| File | Covers |
| :--- | :--- |
| `tests/shim.rs` | Typed round-trips (`from_str` / `to_string` / `from_slice` / `from_reader` / `from_value` / `to_value`), `value` / `mapping` / `with` sub-module path imports, `Error::location()` |

### Examples retained (2 runnable + 17 sub-modules, all execute to completion)

| Path | Notes |
| :--- | :--- |
| `examples/migration.rs` | Standalone migration demo |
| `examples/example.rs` | Aggregator that runs the 17 sub-modules below |
| `examples/serializer/{basic, collections, complex_nested, custom_serialization, enums, error_handling, optional_and_default, structs}.rs` | `basic.rs` was patched to use `to_writer` instead of `Serializer::new(stdout)` (the streaming-serializer constructor is not exposed by noyalib's compat layer) |
| `examples/value/de_examples.rs` | Patched: the `!Variant 0` → `E::Variant(0)` sub-case was removed (noyalib preserves custom tags verbatim, including the leading `!`, so the legacy auto-coercion no longer applies — see "Behavioural notes" above) |
| `examples/with/{singleton_map, singleton_map_recursive, singleton_map_optional, singleton_map_enum_variants, singleton_map_recursive_deep_nesting, singleton_map_recursive_optional, singleton_map_recursive_serialize_deserialize, singleton_map_recursive_with, nested_singleton_map}.rs` | Unchanged from the original — `with::singleton_map*` is fully re-exported |

### Tests removed (legacy implementation-detail coverage)

| File | Why |
| :--- | :--- |
| `tests/test_de.rs` | `Deserializer::from_str(s)` constructor — noyalib's `Deserializer::new` takes a `&Value` |
| `tests/test_error.rs` | Same + `TaggedValue` literal-struct shape differs |
| `tests/test_lib.rs` | Imports `de::Event`, `loader::Loader`, `DocumentAnchor` — all removed |
| `tests/test_mapping.rs` | Pokes `Mapping::map` internal field, `Entry`, `DuplicateKeyError`, `into_keys`, `into_values`, `swap_remove_entry_from` |
| `tests/test_number.rs` | `Number` doesn't impl `Serialize` / `DeserializeOwned` in the compat layer the same way |
| `tests/test_ser.rs` | `ser::SerializerConfig` — the deep `ser` module is gone |
| `tests/test_serde.rs` | `T: 'static` bound mismatch + `String::from(Value)` shape differs |
| `tests/test_value.rs` | Debug-format assertions hard-code the old `Mapping`/`Tag` output |
| `tests/test_with.rs` | `Serializer::new(writer)` + `Deserializer::from_str(s)` + `singleton_map_with::{serialize, deserialize}` aliases |
| `tests/test_tagged.rs` | `value::tagged::nobang` — deep internal helper |
| `tests/value/test_*.rs` (6 files) | Sub-directory tests + probe `Mapping` / `Tag` / `Index` / `libyml::tag` internals |
| `tests/test_anchors_and_aliases.rs`, `test_loader.rs`, `tests/libyml/*`, `tests/modules/*` | C-FFI parser internals — removed in the shim |

### Examples removed

| Path | Why |
| :--- | :--- |
| `examples/libyml/*` (6 files) | Demoed the `libyml` C-FFI surface |
| `examples/loader/*` (5 files) | `loader::Loader` + `de::Progress` |
| `examples/modules/*` (1 file) | `modules::path::Path` |
| `examples/value/index_examples.rs` | `value::Index` sealed trait — `noyalib::Value` implements `Index<&str>` / `Index<usize>` natively |
| `examples/with/singleton_map_with{,_custom_serialize,_custom_serialize_deserialize}.rs` (3 files) | noyalib's `singleton_map_with` exposes `serialize_with` / `deserialize_with` (with explicit transform fn), not bare `serialize` / `deserialize` aliases |

If you depended on any of these, the recommended path is to switch
to `noyalib` directly — its public surface offers the equivalent
functionality with a cleaner, pure-Rust shape.
