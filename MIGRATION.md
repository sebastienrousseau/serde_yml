# Migrating off `serde_yml`

`serde_yml` is unmaintained. The `0.0.13` release is a thin
compatibility shim so existing call sites keep working while you
migrate to a maintained alternative.

> ## ⚠️ Security: RUSTSEC-2025-0068 is fixed in 0.0.13
>
> [RUSTSEC-2025-0068](https://rustsec.org/advisories/RUSTSEC-2025-0068.html)
> flagged all `serde_yml ≤ 0.0.12` as unsound — the
> `serde_yml::ser::Serializer.emitter` field could trigger a
> segmentation fault via the C-FFI `libyaml` parser the original
> crate linked against. **Upgrading to `serde_yml = "0.0.13"`
> removes the vulnerable surface entirely** — the C-FFI dependency
> is gone, `Serializer` is now a pure-Rust unit struct with no
> `emitter` field, and the backend (`noyalib`) enforces
> `#![forbid(unsafe_code)]` workspace-wide. Pinning
> `serde_yml = "=0.0.12"` keeps the advisory in your audit feed;
> pinning `serde_yml = "0.0.13"` (or migrating directly to a
> maintained alternative) clears it.

The shim itself depends on
[`noyalib`](https://crates.io/crates/noyalib)'s `compat-serde-yaml`
feature for its implementation — that's an implementation detail,
not a recommendation that you must migrate to `noyalib`
specifically. Three crates are realistic destinations; pick the one
that fits.

| Destination | Migration shape | When it's the right choice |
| :--- | :--- | :--- |
| **[`noyalib`](https://crates.io/crates/noyalib)** | Drop-in via `compat-serde-yaml` feature | Codebases that want a `serde_yml`-shaped API on a modern, pure-Rust, `#![forbid(unsafe_code)]` backend with zero call-site changes |
| **[`serde-saphyr`](https://crates.io/crates/serde-saphyr)** | Path rename for typed code; **no `Value` DOM** | Typed-deserialise workloads (`from_str::<MyStruct>`) — the 95 % case. Doesn't fit codebases that hold dynamic `Value` trees in flight |
| **[`yaml-rust2`](https://crates.io/crates/yaml-rust2)** | Not serde-integrated — lower-level parser API | Users who were on the low-level `serde_yml::libyml` / `loader` surface (removed in this shim) and want to keep working at that level |

The rest of this document describes each migration path, the
public-surface mapping, the modules that are gone in this shim,
and the behavioural deltas to know about.

---

## Path A — Stay on `serde_yml = "0.0.13"` (stop-gap)

If you cannot migrate right now, depending on the shim keeps your
code compiling. The compiler emits a deprecation warning at every
`use serde_yml::*` import so you can budget the work.

```toml
[dependencies]
serde_yml = "0.0.13"
```

No code changes required. The previous C-FFI parser (`libyml`) and
`serde_yaml` 0.9 are no longer in your dependency graph, so
downstream `cargo audit` / `cargo deny` runs stop flagging the
unmaintained chain immediately.

---

## Path B — Migrate to `noyalib`

```diff
-[dependencies]
-serde_yml = "0.0"
+[dependencies]
+noyalib = { version = "0.0.5", features = ["compat-serde-yaml"] }
```

```diff
-use serde_yml::{from_str, to_string, Value};
+use noyalib::compat::serde_yaml::{from_str, to_string, Value};
```

Or rename at the import site for a one-line diff:

```rust
use noyalib::compat::serde_yaml as serde_yml;
```

That is the entire migration for the typical `from_str` /
`to_string` / `Value` / `Mapping` / `with::singleton_map` codebase.
The public-surface mapping table is in
[§ Public-surface mapping](#public-surface-mapping) below.

---

## Path C — Migrate to `serde-saphyr` (typed-only)

```diff
-[dependencies]
-serde_yml = "0.0"
+[dependencies]
+serde-saphyr = "0.0"
```

```diff
-use serde_yml::from_str;
+use serde_saphyr::from_str;
 let cfg: MyConfig = from_str(yaml)?;
```

`serde-saphyr` is a modern serde-integrated wrapper around the
`saphyr` parser. It covers the typed-deserialise surface — the
95 % case — but it deliberately does not ship a `Value` DOM. If
your code holds `serde_yml::Value` in flight (mapping inspection,
dynamic navigation, ad-hoc JSON-bridge work), the two options
are:

- **Restructure to typed-only deserialisation.** Define a struct
  that mirrors the YAML shape and deserialise into it directly.
  This is usually the cleanest end-state.
- **Pick `noyalib` instead** — it ships a `serde_yml`-compatible
  `Value` type via the `compat-serde-yaml` feature.

| Surface | `serde-saphyr` mapping |
| :--- | :--- |
| `serde_yml::from_str::<T>` | `serde_saphyr::from_str::<T>` |
| `serde_yml::from_slice::<T>` | `serde_saphyr::from_slice::<T>` |
| `serde_yml::from_reader::<R, T>` | `serde_saphyr::from_reader::<R, T>` |
| `serde_yml::to_string` | `serde_saphyr::to_string` |
| `serde_yml::to_writer` | `serde_saphyr::to_writer` |
| `serde_yml::Value`, `Mapping`, `Sequence`, `Number` | **no equivalent** — restructure to typed-only or pick `noyalib` |
| `serde_yml::value::*` / `mapping::*` | same — no equivalent |
| `serde_yml::with::singleton_map*` | check `serde-saphyr` docs for enum-representation helpers; the `singleton_map` family is `serde_yaml`-specific |

---

## Path D — Migrate to `yaml-rust2` (low-level)

```diff
-[dependencies]
-serde_yml = "0.0"
+[dependencies]
+yaml-rust2 = "0.9"
```

```diff
-use serde_yml::from_str;
-let v: serde_yml::Value = serde_yml::from_str(yaml)?;
+use yaml_rust2::YamlLoader;
+let docs = YamlLoader::load_from_str(yaml)?;
+let v = &docs[0];
```

`yaml-rust2` is a pure-Rust YAML parser — the active continuation
of the original `yaml-rust` crate. It returns a `Yaml` enum (its
own AST), **not a `serde::Deserialize` value**. Migrating to it
means bringing your own typed-conversion code or hand-writing the
read paths. This is the right choice when you actually want the
parser primitives — custom loaders, lint tools, format-preserving
editors — i.e. exactly the workloads that the now-removed
`serde_yml::libyml` / `serde_yml::loader` surface served.

For typed `from_str::<T>` flows, prefer `noyalib` or
`serde-saphyr`.

---

## Public-surface mapping

The common surface is preserved name-for-name through the
`serde_yml 0.0.13` shim, and maps directly to `noyalib` for users
taking Path B:

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

For `serde-saphyr` and `yaml-rust2`, see Path C and Path D above —
those crates do not aim for `serde_yaml` surface compatibility.

---

## Removed in 0.0.13

The deep internal modules that previous versions exposed leaked
implementation details of the C-FFI parser. They are **removed**
in the shim. If your code depended on any of these, the right
replacement depends on which destination you chose:

| Removed from `serde_yml`                 | What it was                          | Where it goes |
| ---------------------------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `serde_yml::libyml::*`                   | Raw FFI bindings to C `libyaml`      | `yaml-rust2` for low-level parsing; otherwise none — the pure-Rust replacements don't expose FFI |
| `serde_yml::loader::Loader`              | Low-level YAML event loader          | `yaml-rust2::YamlLoader`; `noyalib::load_all_as::<T>` for typed |
| `serde_yml::de::Event`                   | Re-export of the libyml event enum   | `yaml-rust2`'s parser API; `noyalib`'s streaming `Deserializer` for typed flows |
| `serde_yml::de::Progress`                | Input cursor for `Loader`            | Not needed — `noyalib` / `serde-saphyr` handle cursoring internally |
| `serde_yml::de::DocumentAnchor`          | Anchor-resolution helper             | `noyalib` and `serde-saphyr` resolve anchors transparently |
| `serde_yml::ser::SerializerConfig`       | C-emitter configuration              | `noyalib::ser::Config` |
| `serde_yml::ser::State`                  | C-emitter state machine handle       | None — modern serializers don't expose internal state |
| `serde_yml::modules::path::Path`         | Error-path builder                   | `noyalib::Error::location()` / `Error::path()`; `serde-saphyr::Error` provides similar info |
| `serde_yml::number::*`                   | Number-parsing helpers               | `noyalib::Number` (same API) |
| `serde_yml::value::Index`                | Sealed trait for `Value` indexing    | `noyalib::Value` implements `std::ops::Index<&str>` / `<usize>` natively |

This repository is archived — direct migration questions to the
destination crate's issue tracker.

---

## Behavioural notes

The shim is backed by `noyalib`'s parser, which is intentionally
safer than the original `serde_yml` defaults. Two behaviours flow
through that you may need to handle:

1. **Custom-tag scalars surface as `Value::Tagged`** rather than
   being silently coerced to the inner string. Code that
   exhaustively matched the previous six-variant `Value` enum
   needs either a `Value::Tagged(_)` arm or a call to
   `Value::untag()` / `Value::untag_ref()` before the match.

2. **YAML 1.2 strict booleans by default.** `country: NO` stays
   `"NO"` (the YAML 1.2 fix to the "Norway problem") instead of
   becoming `false`. The legacy boolean recognition was a YAML 1.1
   resolver behaviour.

Migrations to `serde-saphyr` or `yaml-rust2` will encounter the
same YAML 1.2 strictness (it is the spec-correct behaviour); only
`noyalib` exposes an explicit opt-back-in via
`ParserConfig::version(YamlVersion::V1_1)`.

---

## MSRV change

`serde_yml 0.0.13` requires **Rust 1.85.0** (matching the
backend's MSRV); the previous releases required 1.56. Users who
cannot move past 1.56 should pin `serde_yml = "=0.0.12"` and plan
a migration window.

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
| `examples/migration.rs` | Standalone shim demo |
| `examples/example.rs` | Aggregator that runs the 17 sub-modules below |
| `examples/serializer/{basic, collections, complex_nested, custom_serialization, enums, error_handling, optional_and_default, structs}.rs` | `basic.rs` was patched to use `to_writer` instead of `Serializer::new(stdout)` (the streaming-serializer constructor is not exposed by the shim) |
| `examples/value/de_examples.rs` | Patched: the `!Variant 0` → `E::Variant(0)` sub-case was removed (custom tags are preserved verbatim, including the leading `!`, so the legacy auto-coercion no longer applies — see "Behavioural notes" above) |
| `examples/with/{singleton_map, singleton_map_recursive, singleton_map_optional, singleton_map_enum_variants, singleton_map_recursive_deep_nesting, singleton_map_recursive_optional, singleton_map_recursive_serialize_deserialize, singleton_map_recursive_with, nested_singleton_map}.rs` | Unchanged from the original — `with::singleton_map*` is fully re-exported |

### Tests removed (legacy implementation-detail coverage)

| File | Why |
| :--- | :--- |
| `tests/test_de.rs` | `Deserializer::from_str(s)` constructor — the shim's `Deserializer::new` takes a `&Value` |
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
| `examples/value/index_examples.rs` | `value::Index` sealed trait — modern `Value` types implement `Index<&str>` / `Index<usize>` natively |
| `examples/with/singleton_map_with{,_custom_serialize,_custom_serialize_deserialize}.rs` (3 files) | The shim's `singleton_map_with` exposes `serialize_with` / `deserialize_with` (with explicit transform fn), not bare `serialize` / `deserialize` aliases |

If you depended on any of these, pick the destination crate from
the table at the top of this document — its public surface
offers the equivalent functionality.
