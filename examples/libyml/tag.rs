//! Examples for the `Tag` struct and its methods in the `tag` module.
//!
//! This file demonstrates the creation, usage, and comparison of `Tag` instances,
//! as well as the usage of its various methods.

use serde_yml::libyml::tag::Tag;
use serde_yml::libyml::tag::TagFormatError;

pub(crate) fn main() {
    // Print a message to indicate the file being executed.
    println!("\n❯ Executing examples/libyml/tag.rs");

    // Example: Creating a new Tag instance
    let tag_null = Tag::new(Tag::NULL);
    println!(
        "\n✅ Created a new Tag instance for NULL: {:?}",
        tag_null
    );

    // Example: Creating a Tag instance for a custom tag
    let custom_tag = Tag::new("tag:example.org,2024:custom");
    println!(
        "\n✅ Created a new Tag instance for custom tag: {:?}",
        custom_tag
    );

    // Example: Checking if a Tag starts with a prefix
    match custom_tag.starts_with("tag:example.org") {
        Ok(true) => {
            println!("\n✅ The tag starts with the given prefix.")
        }
        Ok(false) => println!(
            "\n✅ The tag does not start with the given prefix."
        ),
        Err(TagFormatError) => {
            println!("\n✅ Error: The prefix is longer than the tag.")
        }
    }

    // Example: Comparing a Tag with a &str
    let comparison_str = "tag:example.org,2024:custom";
    if custom_tag == comparison_str {
        println!("\n✅ The tag is equal to the given string slice.");
    } else {
        println!(
            "\n✅ The tag is not equal to the given string slice."
        );
    }

    // Example: Using Deref to access the underlying byte slice
    let tag_bytes: &[u8] = &custom_tag;
    println!(
        "\n✅ The underlying byte slice of the tag: {:?}",
        tag_bytes
    );

    // Example: Using the Debug implementation
    println!(
        "\n✅ Debug representation of the custom tag: {:?}",
        custom_tag
    );

    // Example: Using Tag constants
    let tag_bool = Tag::new(Tag::BOOL);
    println!(
        "\n✅ Created a new Tag instance for BOOL: {:?}",
        tag_bool
    );
    let tag_int = Tag::new(Tag::INT);
    println!("\n✅ Created a new Tag instance for INT: {:?}", tag_int);
    let tag_float = Tag::new(Tag::FLOAT);
    println!(
        "\n✅ Created a new Tag instance for FLOAT: {:?}",
        tag_float
    );
}
