//! Basic usage of the YAML serializer via the deprecation shim.
//!
//! The original example used `Serializer::new(writer)` and called
//! `Person::serialize(&mut serializer)`. The noyalib backend that
//! powers the 0.0.13 shim does not expose a streaming-serializer
//! constructor at the same shape, so this example uses the
//! equivalent `to_writer` entry point instead.

use serde::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

pub(crate) fn main() {
    println!("\n❯ Executing examples/serializer/basic.rs");

    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        city: "New York".to_string(),
    };

    serde_yml::to_writer(std::io::stdout(), &person).unwrap();

    println!("\n✅ Person serialized to YAML.");
}
