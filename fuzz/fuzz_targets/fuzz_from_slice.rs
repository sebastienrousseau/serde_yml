#![no_main]
#![allow(deprecated)]

// `serde_yml` 0.0.13 is a deprecation shim — the actual parsing is
// performed by `noyalib`. Fuzz findings against this target are
// effectively findings against the `noyalib` parser; file them at
// <https://github.com/sebastienrousseau/noyalib/issues> rather than
// here. This target is kept so that `serde_yml` users running their
// own fuzz harness against the shim continue to work.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() <= 10240 {
        _ = serde_yml::from_slice::<serde_yml::Value>(data);
    }
});
