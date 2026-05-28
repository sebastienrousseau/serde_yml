<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/serde_yml/v1/logos/serde_yml.svg" alt="serde_yml logo" width="128" />
</p>

<h1 align="center">serde_yml</h1>

<p align="center">
  Deprecated YAML library for Rust. The <code>0.0.13</code> release
  is a thin compatibility shim so existing call sites keep working
  while you migrate to a maintained alternative of your choice.
</p>

<p align="center">
  <a href="https://crates.io/crates/serde_yml"><img src="https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge&color=red&label=deprecated&logo=rust" alt="Crates.io (deprecated)" /></a>
  <a href="https://docs.rs/serde_yml"><img src="https://img.shields.io/badge/docs.rs-serde__yml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="./MIGRATION.md"><img src="https://img.shields.io/badge/migration-guide-66c2a5?style=for-the-badge" alt="Migration guide" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — stop-gap shim usage
- [Security: RUSTSEC-2025-0068 fixed in 0.0.13](#security-rustsec-2025-0068-fixed-in-0013)
- [Quick Start](#quick-start) — shim usage in ten lines

**Choosing a replacement**

- [Maintained alternatives](#maintained-alternatives) — picking a destination crate
- [One-minute migration paths](#one-minute-migration-paths) — diff snippets per destination

**Deprecation reference**

- [What changed in 0.0.13](#what-changed-in-0013) — the shim, in one paragraph
- [What still works in 0.0.13](#what-still-works-in-0013) — surviving tests and examples
- [What was removed in 0.0.13](#what-was-removed-in-0013) — the C-FFI surface
- [Behavioural notes](#behavioural-notes) — two intentional deltas worth knowing

**Operational**

- [MSRV](#msrv) — Rust 1.85.0 floor
- [Documentation](#documentation) — migration guide, alternative-crate docs
- [License](#license)

---

## Install

`serde_yml = "0.0.13"` is a stop-gap so an in-flight migration
doesn't block your release. Existing call sites compile unchanged;
the compiler emits a deprecation warning at each `use serde_yml::*`
import pointing at the migration guide.

```toml
[dependencies]
serde_yml = "0.0.13"
```

The shim itself depends on `noyalib`'s `compat-serde-yaml` feature
for its implementation. The previous C-FFI parser (`libyml`) and
`serde_yaml` 0.9 are no longer in the dependency graph. Whether
your eventual destination is `noyalib` or one of the other
[maintained alternatives](#maintained-alternatives) is your call.

---

## Security: RUSTSEC-2025-0068 fixed in 0.0.13

[**RUSTSEC-2025-0068**](https://rustsec.org/advisories/RUSTSEC-2025-0068.html)
(also [GHSA-hhw4-xg65-fp2x](https://github.com/advisories/GHSA-hhw4-xg65-fp2x))
flagged **all `serde_yml` versions ≤ 0.0.12** as unsound — the
`serde_yml::ser::Serializer.emitter` field could cause a
segmentation fault. The advisory is informational severity but
real: it stems from the C-FFI `libyaml` parser the original crate
linked against.

**Upgrading to `serde_yml = "0.0.13"` removes the vulnerable
surface entirely:**

- The C-FFI `libyml` dependency is **gone** from the graph.
- `serde_yml::ser::Serializer` is now a re-export of a pure-Rust
  unit struct (`pub struct Serializer;`) with **no `emitter`
  field**. Any code that referenced `.emitter` won't compile —
  which is exactly the desired outcome.
- The pure-Rust backend (`noyalib`) enforces
  `#![forbid(unsafe_code)]` across the whole workspace, so the
  underlying unsoundness class cannot recur through the shim.

Verification:

```bash
$ cargo update -p serde_yml --precise 0.0.13
$ cargo tree -p serde_yml | grep libyml
# (no output — libyml is not in the dependency graph)
```

The same structural fix flows through to any
[maintained alternative](#maintained-alternatives) you eventually
migrate to.

### `cargo audit` will still warn — here's why and how to handle it

The RustSec advisory database has chosen **not** to mark `0.0.13` as
patched. The decision was made on the basis that `serde_yml` is
unmaintained regardless of which version you pick — a position that
applies to the *crate* as a whole rather than to a specific *code
path*. Practical consequence: `cargo audit` and `cargo deny` will
emit RUSTSEC-2025-0068 against `serde_yml = "0.0.13"` even though
the unsound surface no longer exists in this release.

This repo's own CI suppresses the warning via [`.cargo/audit.toml`](./.cargo/audit.toml)
and [`deny.toml`](./deny.toml). If you're a downstream user who
wants the same suppression in your own project, copy the snippet
below — and feel free to remove it whenever you migrate fully.

```toml
# .cargo/audit.toml — at the workspace root
[advisories]
# RUSTSEC-2025-0068 affects serde_yml's C-FFI surface (`libyml`).
# The 0.0.13 deprecation shim removes that surface entirely; verify
# locally with `cargo tree -p serde_yml | grep libyml` (no output).
# The advisory database tracks the crate's unmaintained status, not
# code presence — so we ignore the advisory ourselves and document
# the structural fix here.
ignore = ["RUSTSEC-2025-0068"]
```

For `cargo deny`, add the same `RUSTSEC-2025-0068` ID to your
`deny.toml`'s `[advisories] ignore` list with the same rationale.

The cleanest long-term path is still to migrate off `serde_yml`
entirely — the maintained alternatives are listed above.

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
aggregator) or `cargo run --example migration` (a single-file demo).

---

## Maintained alternatives

Three crates are realistic destinations for a `serde_yml` user.
None is being prescribed — pick the one that fits the codebase.

| Crate | Latest | Migration shape | Best fit |
| :--- | :--- | :--- | :--- |
| **[`noyalib`](https://crates.io/crates/noyalib)** | 0.0 | Drop-in via `features = ["compat-serde-yaml"]` — zero call-site changes for typical users | Codebases that want a `serde_yml`-shaped API on a modern, safe, pure-Rust backend |
| **[`serde-saphyr`](https://crates.io/crates/serde-saphyr)** | 0.0 | Path rename for typed code; **no `Value` DOM** | Typed-deserialise workloads (`from_str::<MyStruct>`) — the 95 % case. Doesn't fit codebases that hold dynamic `Value` trees in flight |
| **[`yaml-rust2`](https://crates.io/crates/yaml-rust2)** | 0.9 | Not serde-integrated — lower-level parser API | Users who were on the low-level `serde_yml::libyml` / `loader` surface (removed in this shim) and want to keep working at that level |

### Decision guide

- **You used `serde_yml::from_str` / `to_string` / `Value` /
  `Mapping` / `with::singleton_map*`** — pick **`noyalib`**. The
  `compat-serde-yaml` feature was built for this exact migration:
  one-line `Cargo.toml` change, one search-replace on imports,
  done.
- **You only ever called `from_str::<MyStruct>` and never touched
  the `Value` type** — pick **`serde-saphyr`**. Smaller surface
  than `noyalib`, modern parser.
- **You were using `serde_yml::libyml::*` or
  `serde_yml::loader::Loader`** — pick **`yaml-rust2`**. Those
  surfaces are gone from this shim regardless; `yaml-rust2` is the
  natural Rust-native equivalent for low-level YAML processing.

---

## One-minute migration paths

Side-by-side diff snippets for each destination. The full
function-mapping tables are in [`MIGRATION.md`](./MIGRATION.md).

### → `noyalib`

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

### → `serde-saphyr` (typed-only)

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

If your code holds a `serde_yml::Value` in flight, you cannot move
it directly to `serde-saphyr` — there is no equivalent `Value`
type. The two viable options are: (a) restructure to typed-only
deserialisation, or (b) pick `noyalib` instead.

### → `yaml-rust2` (low-level)

```diff
-[dependencies]
-serde_yml = "0.0"
+[dependencies]
+yaml-rust2 = "0.9"
```

```diff
-use serde_yml::{from_str, Value};
-let v: Value = serde_yml::from_str(yaml)?;
+use yaml_rust2::YamlLoader;
+let docs = YamlLoader::load_from_str(yaml)?;
+let v = &docs[0];
```

`yaml-rust2` returns a `Yaml` enum (its own AST), not a
`serde::Deserialize` value — bring your own typed-conversion code
or hand-write the read paths. This is the right choice when you
actually want the parser primitives.

---

## What changed in 0.0.13

`serde_yml 0.0.13` is a thin compatibility shim. The runtime
dependency list dropped from six crates (`libyml`, `indexmap`,
`itoa`, `ryu`, `memchr`, `serde`) to two (`noyalib`, `serde`) —
the C-FFI parser is no longer in the graph. Existing call sites
compile unchanged; the compiler emits a `#[deprecated]` warning at
each one so you can budget the migration.

The shim being backed by `noyalib` internally is an
implementation detail, not a recommendation to use `noyalib`
specifically. The
[Maintained alternatives](#maintained-alternatives) section above
covers the choice.

---

## What still works in 0.0.13

The shim is wire-compatible with typical user code. Verified by
`cargo test --all-targets` + `cargo run --example example` +
`cargo run --example migration`:

| Surface | Status |
| :--- | :--- |
| `tests/shim.rs` — typed round-trips, sub-module path imports, `Error::location()` | **9 / 9 pass** |
| `examples/example.rs` — aggregator running 17 sub-modules from `serializer/`, `value/`, `with/` | **exits 0** |
| `examples/migration.rs` — standalone shim demo | **exits 0** |

The full per-file inventory of retained / patched / removed
tests and examples is in [`MIGRATION.md` § "Test and example
coverage in 0.0.13"](./MIGRATION.md#test-and-example-coverage-in-0013).

---

## What was removed in 0.0.13

The deep internal modules that previous versions exposed leaked
implementation details of the C-FFI parser. They are **removed**
in this shim. The right replacement depends on which alternative
you picked:

| Removed from `serde_yml` | What it was | Where it goes |
| :--- | :--- | :--- |
| `serde_yml::libyml::*` | Raw FFI bindings to C `libyaml` | `yaml-rust2` for low-level parsing; otherwise no equivalent (the pure-Rust replacements don't expose FFI) |
| `serde_yml::loader::Loader` | Low-level YAML event loader | `yaml-rust2::YamlLoader`; `noyalib::load_all_as::<T>` for typed |
| `serde_yml::de::{Event, Progress}` | Event enum + input cursor | Covered by `yaml-rust2`'s parser API or `noyalib`'s streaming `Deserializer` |
| `serde_yml::de::DocumentAnchor` | Anchor-resolution helper | `noyalib` and `serde-saphyr` resolve anchors transparently |
| `serde_yml::ser::{SerializerConfig, State}` | C-emitter configuration | `noyalib::ser::Config` |
| `serde_yml::modules::path::Path` | Error-path builder | `noyalib::Error::location()` / `Error::path()` |
| `serde_yml::value::Index` | Sealed trait for `Value` indexing | `noyalib::Value` implements `Index<&str>` / `Index<usize>` natively |

The full table is in [`MIGRATION.md`](./MIGRATION.md#removed-in-0013).

---

## Behavioural notes

The shim is backed by `noyalib`'s parser, which is intentionally
safer than the original `serde_yml` defaults. Two behaviours flow
through that you may need to handle:

1. **Custom-tag scalars surface as `Value::Tagged`** rather than
   being silently coerced to the inner string. Exhaustive matches
   on the previous six-variant `Value` enum need either a
   `Value::Tagged(_)` arm or a call to `Value::untag()` /
   `Value::untag_ref()` before the match.

2. **YAML 1.2 strict booleans by default.** `country: NO` stays
   `"NO"` (the YAML 1.2 fix to the "Norway problem") instead of
   becoming `false`. The legacy boolean recognition was a YAML 1.1
   resolver behaviour.

Migrations to `serde-saphyr` or `yaml-rust2` will encounter the
same YAML 1.2 strictness (it is the spec-correct behaviour); only
`noyalib` exposes an explicit opt-back-in via
`ParserConfig::version(YamlVersion::V1_1)`.

---

## MSRV

`serde_yml 0.0.13` requires **Rust 1.85.0** (matching the
backend's MSRV). The previous releases required 1.56. Users who
cannot move past 1.56 should pin `serde_yml = "=0.0.12"` and plan
a migration window.

---

## Documentation

| Document | Covers |
| --- | --- |
| [`MIGRATION.md`](./MIGRATION.md) | Find/replace tables per destination, full removed-surface mapping, test/example coverage triage, MSRV note |
| [`noyalib`](https://docs.rs/noyalib) — [GitHub](https://github.com/sebastienrousseau/noyalib) | Drop-in destination via `compat-serde-yaml` feature |
| [`serde-saphyr`](https://docs.rs/serde-saphyr) | Typed-deserialise destination |
| [`yaml-rust2`](https://docs.rs/yaml-rust2) | Low-level parser destination |
| [docs.rs/serde_yml](https://docs.rs/serde_yml) | API reference for this shim — every item carries the `#[deprecated]` banner |

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#contents">Back to Top</a></p>
