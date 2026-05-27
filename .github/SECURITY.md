# Security policy

## Status: `serde_yml` is deprecated

`serde_yml` is **unmaintained**. The `0.0.13` release is a thin
compatibility shim that forwards every call to a maintained
backend (`noyalib`) so existing call sites keep compiling. See
[`MIGRATION.md`](../MIGRATION.md) for the recommended migration
paths.

## Known advisory — fixed in 0.0.13

[**RUSTSEC-2025-0068**](https://rustsec.org/advisories/RUSTSEC-2025-0068.html)
(also [GHSA-hhw4-xg65-fp2x](https://github.com/advisories/GHSA-hhw4-xg65-fp2x))
flagged all `serde_yml ≤ 0.0.12` as unsound — the
`serde_yml::ser::Serializer.emitter` field could cause a
segmentation fault via the C-FFI `libyaml` parser the original
crate linked against.

**Upgrading to `serde_yml = "0.0.13"` removes the vulnerable
surface entirely.** The C-FFI dependency is gone, `Serializer` is
now a pure-Rust unit struct with no `emitter` field, and the
backend enforces `#![forbid(unsafe_code)]` workspace-wide.
Verification:

```bash
cargo update -p serde_yml --precise 0.0.13
cargo audit          # RUSTSEC-2025-0068 cleared for serde_yml
cargo tree -p serde_yml | grep libyml   # (no output)
```

## Supported versions

| Version | Status |
| :--- | :--- |
| `0.0.13` | Deprecation shim — backed by a maintained, pure-Rust parser; safe to use as a stop-gap |
| `≤ 0.0.12` | **End-of-life.** Pinning these keeps RUSTSEC-2025-0068 in your audit feed. Migrate. |

## Reporting a vulnerability

Because the crate is unmaintained, please **file new
vulnerability reports against the backend crate**, not against
`serde_yml`:

- For findings in the YAML parser / serializer (the actual code
  path): [github.com/sebastienrousseau/noyalib/security/advisories/new](https://github.com/sebastienrousseau/noyalib/security/advisories/new)
- For findings in the alternative crates listed in
  [`MIGRATION.md`](../MIGRATION.md): use that crate's own
  disclosure channel.

If a finding is **specific to the `serde_yml` shim layer itself**
(the re-export glue in `src/lib.rs`, not the underlying parser),
open a private security advisory against this repository:
<https://github.com/sebastienrousseau/serde_yml/security/advisories/new>.

When reporting, include:

- Type of issue (e.g. buffer overflow, soundness, SQL injection,
  cross-site scripting)
- Full paths of source file(s) related to the manifestation of
  the issue
- The location of the affected source code (tag/branch/commit or
  direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact, including how an attacker might exploit the issue

This information helps triage your report quickly.
