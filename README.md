<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://kura.pro/serde_yml/images/logos/serde_yml.svg" alt="serde_yml logo" width="128" />
</p>

<h1 align="center">serde_yml</h1>

<p align="center">
  Deprecated YAML library for Rust — the <code>0.0.13</code> release
  is a thin compatibility shim that forwards every call to
  <a href="https://crates.io/crates/noyalib"><code>noyalib</code></a>,
  a pure-Rust YAML 1.2 parser with zero <code>unsafe</code> code.
</p>

<p align="center">
  <a href="https://crates.io/crates/serde_yml"><img src="https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge&color=red&label=deprecated&logo=rust" alt="Crates.io (deprecated)" /></a>
  <a href="https://crates.io/crates/noyalib"><img src="https://img.shields.io/crates/v/noyalib.svg?style=for-the-badge&color=fc8d62&label=use%20noyalib&logo=rust" alt="Use noyalib" /></a>
  <a href="https://docs.rs/serde_yml"><img src="https://img.shields.io/badge/docs.rs-serde__yml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://docs.rs/noyalib"><img src="https://img.shields.io/badge/docs.rs-noyalib-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="noyalib docs" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — recommended (`noyalib`) vs. stop-gap (`serde_yml 0.0.13`)
- [Quick Start](#quick-start) — shim usage in ten lines

**Deprecation reference**

- [What changed in 0.0.13](#what-changed-in-0013) — the shim, in one paragraph
- [One-minute migration to noyalib](#one-minute-migration-to-noyalib) — name-for-name mapping
- [Why migrate?](#why-migrate) — design rationale
- [What still works in 0.0.13](#what-still-works-in-0013) — surviving tests and examples
- [What was removed in 0.0.13](#what-was-removed-in-0013) — the C-FFI surface
- [Behavioural notes](#behavioural-notes) — two intentional deltas worth knowing

**Operational**

- [MSRV](#msrv) — Rust 1.85.0 floor
- [Documentation](#documentation) — migration guide, noyalib docs, license bundle
- [License](#license)

---

## Install

### Recommended — depend on `noyalib` directly

```toml
[dependencies]
noyalib = { version = "0.0.5", features = ["compat-serde-yaml"] }
```

```diff
- use serde_yml::{from_str, to_string, Value};
+ use noyalib::compat::serde_yaml::{from_str, to_string, Value};
```

The `compat-serde-yaml` feature exposes a name-for-name surface
mirroring `serde_yml` / `serde_yaml` 0.9. Every type is
noyalib-native — no transitive dep on the archived upstream, so
downstream `cargo audit` / `cargo deny` runs stop flagging the
unmaintained chain.

### Stop-gap — keep `serde_yml = "0.0.13"`

```toml
[dependencies]
serde_yml = "0.0.13"
```

Existing call sites compile unchanged. The compiler emits a
deprecation warning at every `use serde_yml::*` import pointing at
the migration guide. The shim ships only `noyalib` as a runtime
dependency; the previous C-FFI parser (`libyml`) has been removed
from the dependency graph entirely.

---

## Quick Start

```rust
#![allow(deprecated)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    name: String,
    port: u16,
}

fn main() -> serde_yml::Result<()> {
    let yaml = "name: myapp\nport: 8080\n";
    let cfg: Config = serde_yml::from_str(yaml)?;
    let back = serde_yml::to_string(&cfg)?;
    let round: Config = serde_yml::from_str(&back)?;
    assert_eq!(cfg, round);
    Ok(())
}
```

Run the bundled examples with `cargo run --example example` (the
aggregator) or `cargo run --example migration` (the standalone
migration demo).

---

## What changed in 0.0.13

`serde_yml 0.0.13` is a thin compatibility shim. Every public item
re-exports from [`noyalib::compat::serde_yaml`]. The runtime
dependency list dropped from six crates (`libyml`, `indexmap`,
`itoa`, `ryu`, `memchr`, `serde`) to two (`noyalib`, `serde`), and
the C-FFI parser is no longer in the graph. Existing call sites
compile unchanged — the compiler emits a `#[deprecated]` warning
at each one so you can budget the migration.

[`noyalib::compat::serde_yaml`]: https://docs.rs/noyalib/latest/noyalib/compat/serde_yaml/

---

## One-minute migration to noyalib

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

| `serde_yml` (≤ 0.0.12) | `serde_yml` 0.0.13 shim | Direct `noyalib` |
| --- | --- | --- |
| `serde_yml::from_str` | unchanged | `noyalib::compat::serde_yaml::from_str` |
| `serde_yml::from_slice` | unchanged | `noyalib::compat::serde_yaml::from_slice` |
| `serde_yml::from_reader` | unchanged | `noyalib::compat::serde_yaml::from_reader` |
| `serde_yml::from_value` | unchanged | `noyalib::compat::serde_yaml::from_value` |
| `serde_yml::to_string` | unchanged | `noyalib::compat::serde_yaml::to_string` |
| `serde_yml::to_writer` | unchanged | `noyalib::compat::serde_yaml::to_writer` |
| `serde_yml::to_value` | unchanged | `noyalib::compat::serde_yaml::to_value` |
| `serde_yml::Value` | unchanged | `noyalib::compat::serde_yaml::Value` |
| `serde_yml::Mapping` | unchanged | `noyalib::compat::serde_yaml::Mapping` |
| `serde_yml::Number` | unchanged | `noyalib::compat::serde_yaml::Number` |
| `serde_yml::Sequence` | unchanged | `noyalib::compat::serde_yaml::Sequence` |
| `serde_yml::Error` / `Location` | unchanged | `noyalib::compat::serde_yaml::{Error, Location}` |
| `serde_yml::Deserializer` | unchanged | `noyalib::compat::serde_yaml::Deserializer` |
| `serde_yml::Serializer` | unchanged | `noyalib::compat::serde_yaml::Serializer` |
| `serde_yml::value::*` | unchanged | `noyalib::compat::serde_yaml::value::*` |
| `serde_yml::mapping::*` | unchanged | `noyalib::compat::serde_yaml::mapping::*` |
| `serde_yml::with::singleton_map*` | unchanged | `noyalib::compat::serde_yaml::with::*` |

Full table including the removed internal modules — and what to
reach for in noyalib instead — lives at
[`MIGRATION.md`](./MIGRATION.md).

---

## Why migrate?

- **Maintained.** `serde_yml` is archived. `noyalib` is actively
  developed; YAML 1.2 corrections and security advisories flow
  into it on every release.
- **Safe.** noyalib enforces `#![forbid(unsafe_code)]` across the
  entire workspace — no FFI, no raw-pointer dereferences, no
  `unsafe` blocks in the parser, scanner, formatter, or CST. The
  previous `serde_yml` releases linked the C `libyaml` parser via
  `libyml`.
- **Faster.** noyalib's deserialiser outpaces `serde_yaml_ng`
  (the most active `serde_yaml` fork) by 39–64 % on representative
  workloads; the streaming path adds another 22 % on top for large
  documents.
- **Spec-compliant.** Passes 387/387 attempted cases in the
  official YAML 1.2 test suite under strict comparison.
- **No archived advisory chain.** The shim depends only on
  `noyalib`. Downstream `cargo audit` and `cargo deny` runs stop
  flagging the `serde_yaml` 0.9 / `libyml` advisory chain.

See [noyalib's design rationale](https://github.com/sebastienrousseau/noyalib/blob/main/README.md#why-this-approach)
for the longer write-up.

---

## What still works in 0.0.13

The shim is wire-compatible with typical user code. Verified by
running `cargo test --all-targets` + `cargo run --example example`
+ `cargo run --example migration`:

| Surface | Status |
| :--- | :--- |
| `tests/shim.rs` — typed round-trips, sub-module path imports, `Error::location()` | **9 / 9 pass** |
| `examples/example.rs` — aggregator running 17 sub-modules from `serializer/`, `value/`, `with/` | **exits 0** |
| `examples/migration.rs` — standalone migration demo | **exits 0** |

The full per-file inventory of retained / patched / removed
tests and examples is in [`MIGRATION.md` § "Test and example
coverage in 0.0.13"](./MIGRATION.md#test-and-example-coverage-in-0013).

---

## What was removed in 0.0.13

The deep internal modules that previous versions exposed leaked
implementation details of the C-FFI parser. They have **no
Rust-only equivalent in `noyalib`** and are removed in the shim:

| Removed | What it was | Where it went |
| :--- | :--- | :--- |
| `serde_yml::libyml::*` | Raw FFI bindings to C `libyaml` | n/a — `noyalib` is pure Rust, exposes no FFI |
| `serde_yml::loader::Loader` | Low-level YAML event loader | `noyalib::load_all_as::<T>` or `noyalib::de::Deserializer` |
| `serde_yml::de::{Event, Progress}` | Event enum + input cursor for `Loader` | Covered by noyalib's streaming API |
| `serde_yml::de::DocumentAnchor` | Anchor-resolution helper | `noyalib` resolves anchors transparently |
| `serde_yml::ser::{SerializerConfig, State}` | C-emitter configuration + state | `noyalib::ser::Config` |
| `serde_yml::modules::path::Path` | Error-path builder | `noyalib::Error::location()` / `Error::path()` |
| `serde_yml::value::Index` | Sealed trait for `Value` indexing | `noyalib::Value` implements `Index<&str>` / `Index<usize>` natively |

The full table is in [`MIGRATION.md`](./MIGRATION.md#removed-in-0013).

---

## Behavioural notes

Two behaviours in `noyalib` are intentionally safer than the
original `serde_yml` defaults. They flow through the shim:

1. **Custom-tag scalars surface as `Value::Tagged`** rather than
   being silently coerced to the inner string. Exhaustive matches
   on the previous six-variant `Value` enum need either a
   `Value::Tagged(_)` arm or a call to `Value::untag()` /
   `Value::untag_ref()` before the match.

2. **YAML 1.2 strict booleans by default.** `country: NO` stays
   `"NO"` (the YAML 1.2 fix to the "Norway problem") instead of
   becoming `false`. Opt back into YAML 1.1 resolver semantics via
   `noyalib::ParserConfig::version(noyalib::YamlVersion::V1_1)`
   if your pipeline depended on the legacy boolean recognition.

Both are covered in detail by noyalib's
[migration guide](https://github.com/sebastienrousseau/noyalib/blob/main/doc/MIGRATION-FROM-SERDE-YAML.md)
along with the per-case opt-outs.

---

## MSRV

`serde_yml 0.0.13` requires **Rust 1.85.0** (noyalib's MSRV); the
previous releases required 1.56. Users who cannot move past 1.56
should pin `serde_yml = "=0.0.12"` and plan a migration window.

---

## Documentation

| Document | Covers |
| --- | --- |
| [`MIGRATION.md`](./MIGRATION.md) | Find/replace tables, full removed-surface mapping, test/example coverage triage, MSRV note |
| [noyalib README](https://github.com/sebastienrousseau/noyalib/blob/main/README.md) | The replacement crate — features, benchmarks, ecosystem comparison |
| [noyalib migration guides](https://github.com/sebastienrousseau/noyalib/blob/main/doc/MIGRATION.md) | Per-crate migration guides (`serde_yaml`, `serde_yml`, `serde-yaml-ng`, `serde-norway`, and others) |
| [docs.rs/serde_yml](https://docs.rs/serde_yml) | API reference for the shim — every item carries the `#[deprecated]` banner |
| [docs.rs/noyalib](https://docs.rs/noyalib) | API reference for the replacement crate |

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#contents">Back to Top</a></p>
