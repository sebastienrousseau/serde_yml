// SPDX-License-Identifier: MIT OR Apache-2.0
//
// `serde_yml` is deprecated. This example shows the two valid paths
// in 0.0.13:
//
//  1. (Recommended) Switch directly to `noyalib`:
//       cargo remove serde_yml
//       cargo add noyalib --features compat-serde-yaml
//       sed -i 's/serde_yml/noyalib::compat::serde_yaml/g' src/**/*.rs
//
//  2. (Stop-gap) Keep `serde_yml = "0.0.13"`; every call below
//     forwards to noyalib through the shim. The compiler will emit
//     a deprecation warning for each `serde_yml::*` usage,
//     marking the call sites for migration.
//
// Run with: `cargo run --example migration`

#![allow(deprecated)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    title: String,
    port: u16,
}

fn main() -> serde_yml::Result<()> {
    let yaml = "title: MyApp\nport: 8080\n";
    let cfg: Config = serde_yml::from_str(yaml)?;
    println!("parsed: {cfg:?}");

    let out = serde_yml::to_string(&cfg)?;
    println!("emitted:\n{out}");
    Ok(())
}
