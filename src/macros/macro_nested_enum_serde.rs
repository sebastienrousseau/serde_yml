#[macro_export]
/// A macro that serializes a nested singleton map to a YAML format.
///
/// This macro uses the `serde_yml` crate to serialize the input value to the provided writer.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use serde_yml::with::nested_singleton_map;
/// use serde_yml::nested_singleton_map_serialize;
/// use serde_yml::nested_singleton_map_deserialize;
///
/// // Define the inner enum with different variants
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum InnerEnum {
///     Variant1,
///     Variant2(String),
///     Variant3 {
///         field1: i32,
///         field2: bool,
///     },
/// }
///
/// // Define the outer enum that contains the inner enum as a field
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum OuterEnum {
///     Variant1(InnerEnum),
///     Variant2 {
///         inner: InnerEnum,
///     },
/// }
///
/// // Define a struct that contains the outer enum as a field
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct NestedEnumStruct {
///     #[serde(with = "nested_singleton_map")]
///     field: OuterEnum,
/// }
///
/// // Example 1: OuterEnum::Variant1(InnerEnum::Variant1)
/// let input1 = NestedEnumStruct {
///     field: OuterEnum::Variant1(InnerEnum::Variant1),
/// };
/// let mut writer = Vec::new();
/// nested_singleton_map_serialize!(&input1, &mut writer).unwrap();
/// println!("\n✅ Serialized YAML for Example 1:\n{}", String::from_utf8(writer).unwrap());
/// ```
macro_rules! nested_singleton_map_serialize {
    ($value:expr, $writer:expr) => {
        // Use `serde_yml::with::nested_singleton_map::serialize` to serialize the input value.
        // The serializer writes the output to the provided writer.
        serde_yml::with::nested_singleton_map::serialize($value, &mut serde_yml::Serializer::new($writer))
    };
}

#[macro_export]
/// A macro that deserializes a nested singleton map from a YAML format.
///
/// This macro uses the `serde_yml` crate to deserialize the input value from a YAML string.
/// It directly calls `serde_yml::from_str` to perform the deserialization and uses `expect`
/// to handle any potential deserialization errors by panicking with a provided message.
///
/// # Example
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use serde_yml::nested_singleton_map_deserialize;
/// use serde_yml;
///
/// // Define your enums and structs as usual
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum InnerEnum {
///     Variant1,
///     Variant2(String),
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum OuterEnum {
///     Variant1(InnerEnum),
///     Variant2 {
///         inner: InnerEnum,
///     },
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Example {
///     #[serde(with = "serde_yml::with::nested_singleton_map")]
///     field: OuterEnum,
/// }
///
/// // Create a YAML string to deserialize
/// let yaml = r#"
///     field:
///       Variant2:
///         inner:
///           Variant2: value
/// "#;
///
/// // Use the macro to deserialize the YAML string
/// let example: Example = nested_singleton_map_deserialize!(&yaml);
///
/// // Verify the deserialization result
/// let expected = Example {
///     field: OuterEnum::Variant2 {
///         inner: InnerEnum::Variant2("value".to_string()),
///     },
/// };
/// assert_eq!(example, expected);
/// ```
macro_rules! nested_singleton_map_deserialize {
    ($yaml:expr) => {{
        // Use `serde_yml::from_str` to deserialize the YAML string.
        // The `expect` method is used to handle any errors that occur during deserialization.
        // If deserialization fails, it will panic with the message "Failed to deserialize".
        serde_yml::from_str($yaml).expect("Failed to deserialize")
    }};
}
