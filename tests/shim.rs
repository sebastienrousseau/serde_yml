// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Smoke tests for the `serde_yml 0.0.14` deprecation shim. The
// public surface here is a thin re-export of
// `noyalib::compat::serde_yaml`; these tests just verify the
// re-exports resolve and round-trip on representative shapes so a
// downstream `cargo update -p serde_yml` does not break compilation
// or runtime behaviour for typical call sites.

#![allow(deprecated)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    name: String,
    port: u16,
}

#[test]
fn from_str_round_trips_typed() {
    let yaml = "name: serde_yml\nport: 8080\n";
    let cfg: Config = serde_yml::from_str(yaml).unwrap();
    assert_eq!(
        cfg,
        Config {
            name: "serde_yml".into(),
            port: 8080,
        }
    );
}

#[test]
fn to_string_then_from_str_round_trips() {
    let cfg = Config {
        name: "noyalib".into(),
        port: 9090,
    };
    let s = serde_yml::to_string(&cfg).unwrap();
    let back: Config = serde_yml::from_str(&s).unwrap();
    assert_eq!(cfg, back);
}

#[test]
fn from_slice_typed() {
    let bytes = b"name: shim\nport: 1234\n";
    let cfg: Config = serde_yml::from_slice(bytes).unwrap();
    assert_eq!(cfg.port, 1234);
}

#[test]
fn from_reader_typed() {
    let bytes: &[u8] = b"name: reader\nport: 80\n";
    let cfg: Config = serde_yml::from_reader(bytes).unwrap();
    assert_eq!(cfg.name, "reader");
}

#[test]
fn from_value_and_to_value_roundtrip() {
    let cfg = Config {
        name: "value".into(),
        port: 7,
    };
    let v: serde_yml::Value = serde_yml::to_value(&cfg).unwrap();
    let back: Config = serde_yml::from_value(v).unwrap();
    assert_eq!(cfg, back);
}

#[test]
fn value_submodule_path_resolves() {
    use serde_yml::value::{Mapping, Number, Value};
    let mut m = Mapping::new();
    let _ = m.insert("k", Value::Number(Number::Integer(1)));
    assert_eq!(m.len(), 1);
}

#[test]
fn mapping_submodule_path_resolves() {
    let m: serde_yml::mapping::Mapping = serde_yml::mapping::Mapping::new();
    assert!(m.is_empty());
}

#[test]
fn with_submodule_path_resolves() {
    // Compile-time check: the `#[serde(with = "serde_yml::with::singleton_map")]`
    // path still resolves through the shim.
    #[allow(unused_imports)]
    use serde_yml::with::{
        nested_singleton_map, singleton_map, singleton_map_optional, singleton_map_recursive,
        singleton_map_with,
    };
}

#[test]
fn error_carries_location() {
    let err = serde_yml::from_str::<serde_yml::Value>("a: [unclosed").unwrap_err();
    let loc = err.location().expect("parse error must carry a location");
    assert!(loc.line() >= 1);
    assert!(loc.column() >= 1);
    let _: usize = loc.index();
}
