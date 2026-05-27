# Migrating off `serde_yml`

`serde_yml` is unmaintained. The `0.0.14` release is a thin
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

## Path B — Stay on `serde_yml = "0.0.14"` (stop-gap)

If you cannot migrate right now, the shim keeps your code compiling.
You get noyalib's safe parser transparently, and the compiler emits
a deprecation warning at every call site so you can budget the work.

```toml
[dependencies]
serde_yml = "0.0.14"
```

No code changes required.

---

## Public-surface mapping

The common surface is preserved name-for-name:

| `serde_yml` (≤ 0.0.13)              | `serde_yml` 0.0.14 shim             | Direct `noyalib` equivalent                       |
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

## Removed in 0.0.14

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

`serde_yml 0.0.14` requires **Rust 1.85.0** (noyalib's MSRV); the
previous releases required 1.56. Users who cannot move past 1.56
should pin `serde_yml = "=0.0.13"` and plan a migration window.
