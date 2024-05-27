#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_yml::with::nested_singleton_map;
    use serde_yml::{
        nested_singleton_map_deserialize,
        nested_singleton_map_serialize,
    };

    // Define the inner enum with different variants
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum InnerEnum {
        Variant1,
        Variant2(String),
        Variant3 { field1: i32, field2: bool },
    }

    // Define the outer enum that contains the inner enum as a field
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum OuterEnum {
        Variant1(InnerEnum),
        Variant2 { inner: InnerEnum },
    }

    // Define a struct that contains the outer enum as a field
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct NestedEnumStruct {
        #[serde(with = "nested_singleton_map")]
        field: OuterEnum,
    }

    // Test for serializing a nested singleton map
    #[test]
    fn test_nested_singleton_map_serialize_variant1() {
        // Example 1: OuterEnum::Variant1(InnerEnum::Variant1)
        let input = NestedEnumStruct {
            field: OuterEnum::Variant1(InnerEnum::Variant1),
        };
        let mut writer = Vec::new();
        nested_singleton_map_serialize!(&input, &mut writer).unwrap();
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 1:\n{}", yaml);
    }

    // Test for deserializing a nested singleton map
    #[test]
    fn test_nested_singleton_map_deserialize_variant1() {
        let input = NestedEnumStruct {
            field: OuterEnum::Variant1(InnerEnum::Variant1),
        };
        let mut writer = Vec::new();
        serde_yml::to_writer(&mut writer, &input)
            .expect("Failed to serialize");
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 1:\n{}", yaml);

        // Deserialize from the YAML string
        let output: NestedEnumStruct =
            nested_singleton_map_deserialize!(&yaml);

        println!(
            "\n✅ Deserialized YAML for Example 1:\n{:#?}",
            output
        );
        assert_eq!(input, output);
    }

    // Test for serializing a nested singleton map with InnerEnum::Variant2
    #[test]
    fn test_nested_singleton_map_serialize_variant2() {
        // Example 2: OuterEnum::Variant1(InnerEnum::Variant2)
        let input = NestedEnumStruct {
            field: OuterEnum::Variant1(InnerEnum::Variant2(
                "test".to_string(),
            )),
        };
        let mut writer = Vec::new();
        nested_singleton_map_serialize!(&input, &mut writer).unwrap();
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 2:\n{}", yaml);
    }

    // Test for deserializing a nested singleton map with InnerEnum::Variant2
    #[test]
    fn test_nested_singleton_map_deserialize_variant2() {
        let input = NestedEnumStruct {
            field: OuterEnum::Variant1(InnerEnum::Variant2(
                "test".to_string(),
            )),
        };
        let mut writer = Vec::new();
        serde_yml::to_writer(&mut writer, &input)
            .expect("Failed to serialize");
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 2:\n{}", yaml);

        // Deserialize from the YAML string
        let output: NestedEnumStruct =
            nested_singleton_map_deserialize!(&yaml);

        println!(
            "\n✅ Deserialized YAML for Example 2:\n{:#?}",
            output
        );
        assert_eq!(input, output);
    }

    // Test for serializing a nested singleton map with InnerEnum::Variant3
    #[test]
    fn test_nested_singleton_map_serialize_variant3() {
        // Example 3: OuterEnum::Variant1(InnerEnum::Variant3)
        let input = NestedEnumStruct {
            field: OuterEnum::Variant1(InnerEnum::Variant3 {
                field1: 42,
                field2: true,
            }),
        };
        let mut writer = Vec::new();
        nested_singleton_map_serialize!(&input, &mut writer).unwrap();
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 3:\n{}", yaml);
    }

    // Test for deserializing a nested singleton map with InnerEnum::Variant3
    #[test]
    fn test_nested_singleton_map_deserialize_variant3() {
        let input = NestedEnumStruct {
            field: OuterEnum::Variant1(InnerEnum::Variant3 {
                field1: 42,
                field2: true,
            }),
        };
        let mut writer = Vec::new();
        serde_yml::to_writer(&mut writer, &input)
            .expect("Failed to serialize");
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 3:\n{}", yaml);

        // Deserialize from the YAML string
        let output: NestedEnumStruct =
            nested_singleton_map_deserialize!(&yaml);

        println!(
            "\n✅ Deserialized YAML for Example 3:\n{:#?}",
            output
        );
        assert_eq!(input, output);
    }

    // Test for serializing a nested singleton map with OuterEnum::Variant2 containing InnerEnum::Variant1
    #[test]
    fn test_nested_singleton_map_serialize_outer_variant2_inner_variant1(
    ) {
        // Example 4: OuterEnum::Variant2 { inner: InnerEnum::Variant1 }
        let input = NestedEnumStruct {
            field: OuterEnum::Variant2 {
                inner: InnerEnum::Variant1,
            },
        };
        let mut writer = Vec::new();
        nested_singleton_map_serialize!(&input, &mut writer).unwrap();
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 4:\n{}", yaml);
    }

    // Test for deserializing a nested singleton map with OuterEnum::Variant2 containing InnerEnum::Variant1
    #[test]
    fn test_nested_singleton_map_deserialize_outer_variant2_inner_variant1(
    ) {
        let input = NestedEnumStruct {
            field: OuterEnum::Variant2 {
                inner: InnerEnum::Variant1,
            },
        };
        let mut writer = Vec::new();
        serde_yml::to_writer(&mut writer, &input)
            .expect("Failed to serialize");
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 4:\n{}", yaml);

        // Deserialize from the YAML string
        let output: NestedEnumStruct =
            nested_singleton_map_deserialize!(&yaml);

        println!(
            "\n✅ Deserialized YAML for Example 4:\n{:#?}",
            output
        );
        assert_eq!(input, output);
    }

    // Test for serializing a nested singleton map with OuterEnum::Variant2 containing InnerEnum::Variant2
    #[test]
    fn test_nested_singleton_map_serialize_outer_variant2_inner_variant2(
    ) {
        // Example 5: OuterEnum::Variant2 { inner: InnerEnum::Variant2 }
        let input = NestedEnumStruct {
            field: OuterEnum::Variant2 {
                inner: InnerEnum::Variant2("test".to_string()),
            },
        };
        let mut writer = Vec::new();
        nested_singleton_map_serialize!(&input, &mut writer).unwrap();
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 5:\n{}", yaml);
    }

    // Test for deserializing a nested singleton map with OuterEnum::Variant2 containing InnerEnum::Variant2
    #[test]
    fn test_nested_singleton_map_deserialize_outer_variant2_inner_variant2(
    ) {
        let input = NestedEnumStruct {
            field: OuterEnum::Variant2 {
                inner: InnerEnum::Variant2("test".to_string()),
            },
        };
        let mut writer = Vec::new();
        serde_yml::to_writer(&mut writer, &input)
            .expect("Failed to serialize");
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 5:\n{}", yaml);

        // Deserialize from the YAML string
        let output: NestedEnumStruct =
            nested_singleton_map_deserialize!(&yaml);

        println!(
            "\n✅ Deserialized YAML for Example 5:\n{:#?}",
            output
        );
        assert_eq!(input, output);
    }

    // Test for serializing a nested singleton map with OuterEnum::Variant2 containing InnerEnum::Variant3
    #[test]
    fn test_nested_singleton_map_serialize_outer_variant2_inner_variant3(
    ) {
        // Example 6: OuterEnum::Variant2 { inner: InnerEnum::Variant3 }
        let input = NestedEnumStruct {
            field: OuterEnum::Variant2 {
                inner: InnerEnum::Variant3 {
                    field1: 42,
                    field2: true,
                },
            },
        };
        let mut writer = Vec::new();
        nested_singleton_map_serialize!(&input, &mut writer).unwrap();
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 6:\n{}", yaml);
    }

    // Test for deserializing a nested singleton map with OuterEnum::Variant2 containing InnerEnum::Variant3
    #[test]
    fn test_nested_singleton_map_deserialize_outer_variant2_inner_variant3(
    ) {
        let input = NestedEnumStruct {
            field: OuterEnum::Variant2 {
                inner: InnerEnum::Variant3 {
                    field1: 42,
                    field2: true,
                },
            },
        };
        let mut writer = Vec::new();
        serde_yml::to_writer(&mut writer, &input)
            .expect("Failed to serialize");
        let yaml = String::from_utf8(writer)
            .expect("Failed to create string from Vec<u8>");

        println!("\n✅ Serialized YAML for Example 6:\n{}", yaml);

        // Deserialize from the YAML string
        let output: NestedEnumStruct =
            nested_singleton_map_deserialize!(&yaml);

        println!(
            "\n✅ Deserialized YAML for Example 6:\n{:#?}",
            output
        );
        assert_eq!(input, output);
    }
}
