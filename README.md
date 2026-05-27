<h1 align="center">⚠️ Serde YML — Deprecated</h1>

<p align="center">
  <strong>This crate is unmaintained. Please migrate to
  <a href="https://crates.io/crates/noyalib"><code>noyalib</code></a>.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/serde_yml"><img src="https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge&color=red&label=deprecated&logo=rust" alt="Crates.io (deprecated)" /></a>
  <a href="https://crates.io/crates/noyalib"><img src="https://img.shields.io/crates/v/noyalib.svg?style=for-the-badge&color=66c2a5&label=use%20noyalib&logo=rust" alt="Use noyalib" /></a>
  <a href="https://docs.rs/noyalib"><img src="https://img.shields.io/badge/docs.rs-noyalib-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="noyalib docs" /></a>
</p>

---

## What changed in `0.0.13`

`serde_yml 0.0.13` is a **thin compatibility shim**. Every public
item now forwards to [`noyalib`], a pure-Rust YAML library with
`#![forbid(unsafe_code)]` enforced workspace-wide. The previous C-FFI
parser (`libyml`) has been **removed entirely** from this crate's
dependency tree.

Existing call sites compile unchanged, but every `use serde_yml::*`
emits a deprecation warning pointing at the migration guide.

## Recommended: switch directly to `noyalib`

```diff
  # Cargo.toml
- serde_yml = "0.0"
+ noyalib = { version = "0.0.5", features = ["compat-serde-yaml"] }
```

```diff
  // anywhere in your codebase
- use serde_yml::{from_str, to_string, Value};
+ use noyalib::compat::serde_yaml::{from_str, to_string, Value};
```

The `noyalib::compat::serde_yaml` module is name-for-name compatible
with the `serde_yml` / `serde_yaml` 0.9 surface, so the migration is
typically a `Cargo.toml` edit plus a search-and-replace on imports.

See [`MIGRATION.md`](./MIGRATION.md) for the full mapping table,
including the small list of types that were removed and what to use
instead.

## Stop-gap: keep `serde_yml = "0.0.13"`

If you cannot migrate right now, depending on `serde_yml = "0.0.13"`
keeps your code compiling. You get noyalib's safe parser
transparently, and the deprecation warnings show every call site
that needs to move.

```toml
[dependencies]
serde_yml = "0.0.13"
```

## Why migrate?

- **Maintained.** `serde_yml` is archived. `noyalib` is actively
  developed; YAML 1.2 corrections and security advisories flow into
  it.
- **Safe.** noyalib forbids `unsafe` code. The previous `serde_yml`
  releases linked the C `libyaml` parser via `libyml` and inherited
  all of its FFI surface.
- **Faster.** noyalib outpaces `serde_yaml_ng` (the most active
  `serde_yaml` fork) by 39 – 64 % on representative workloads.
- **Spec-compliant.** Passes 406/406 cases in the official YAML 1.2
  test suite.
- **No archived advisory chain.** The shim depends only on
  `noyalib`. Downstream `cargo audit` and `cargo deny` runs stop
  flagging the `serde_yaml` 0.9 / `libyml` advisories.

## MSRV

`serde_yml 0.0.13` requires Rust **1.85.0** (noyalib's MSRV).
Users on older toolchains should pin `serde_yml = "=0.0.12"` and
plan a migration window.

## License

Dual-licensed under [MIT](./LICENSE-MIT) or
[Apache-2.0](./LICENSE-APACHE).

[`noyalib`]: https://crates.io/crates/noyalib
