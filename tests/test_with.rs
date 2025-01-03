#![allow(missing_docs)]
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_yml::with::*;

    // Define the enum MyEnum
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum MyEnum {
        Unit,
        Newtype(usize),
        Tuple(usize, usize),
        Struct { value: usize },
    }

    // Test serialization and deserialization using nested_singleton_map
    #[test]
    fn test_nested_singleton_map() {
        // Define enum InnerEnum and OuterEnum for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum InnerEnum {
            Variant1,
            Variant2(String),
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum OuterEnum {
            Variant1(InnerEnum),
            Variant2 { inner: InnerEnum },
        }

        // Define struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "nested_singleton_map")]
            field: OuterEnum,
        }

        // Test serialization and deserialization for OuterEnum::Variant1(InnerEnum::Variant1)
        let test_struct = TestStruct {
            field: OuterEnum::Variant1(InnerEnum::Variant1),
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field:\n  Variant1: Variant1\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);

        // Test serialization and deserialization for OuterEnum::Variant2 { inner: InnerEnum::Variant2("value".to_string()) }
        let test_struct = TestStruct {
            field: OuterEnum::Variant2 {
                inner: InnerEnum::Variant2("value".to_string()),
            },
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(
            yaml,
            "field:\n  Variant2:\n    inner:\n      Variant2: value\n"
        );
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test serialization and deserialization using singleton_map_optional
    #[test]
    fn test_singleton_map_optional() {
        // Define struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "singleton_map_optional")]
            field: Option<MyEnum>,
        }

        // Test serialization and deserialization for Some(MyEnum::Unit) and None
        let test_struct = TestStruct {
            field: Some(MyEnum::Unit),
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field: Unit\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);

        let test_struct = TestStruct { field: None };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field: null\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test serialization and deserialization using singleton_map_with
    #[test]
    fn test_singleton_map_with() {
        // Define struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "singleton_map_with")]
            field: MyEnum,
        }

        // Test serialization and deserialization for MyEnum::Unit
        let test_struct = TestStruct {
            field: MyEnum::Unit,
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field: Unit\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test nested_singleton_map serialization
    #[test]
    fn test_nested_singleton_map_serialization() {
        // Define enum InnerEnum and OuterEnum for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum InnerEnum {
            Variant1,
            Variant2(String),
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum OuterEnum {
            Variant1(InnerEnum),
            Variant2 { inner: InnerEnum },
        }

        // Test serialization for OuterEnum::Variant1(InnerEnum::Variant1)
        let value = OuterEnum::Variant1(InnerEnum::Variant1);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        nested_singleton_map::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant1: Variant1\n");

        // Test serialization for OuterEnum::Variant2 { inner: InnerEnum::Variant2("value".to_string()) }
        let value = OuterEnum::Variant2 {
            inner: InnerEnum::Variant2("value".to_string()),
        };
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        nested_singleton_map::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant2:\n  inner:\n    Variant2: value\n");
    }

    // Test nested_singleton_map deserialization
    #[test]
    fn test_nested_singleton_map_deserialization() {
        // Define enum InnerEnum and OuterEnum for deserialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum InnerEnum {
            Variant1,
            Variant2(String),
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum OuterEnum {
            Variant1(InnerEnum),
            Variant2 { inner: InnerEnum },
        }

        // Test deserialization for OuterEnum::Variant1(InnerEnum::Variant1)
        let yaml = "Variant1: Variant1\n";
        let deserialized: OuterEnum =
            nested_singleton_map::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(
            deserialized,
            OuterEnum::Variant1(InnerEnum::Variant1)
        );

        // Test deserialization for OuterEnum::Variant2 { inner: InnerEnum::Variant2("value".to_string()) }
        let yaml = "Variant2:\n  inner:\n    Variant2: value\n";
        let deserialized: OuterEnum =
            nested_singleton_map::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(
            deserialized,
            OuterEnum::Variant2 {
                inner: InnerEnum::Variant2("value".to_string())
            }
        );
    }

    // Test serialization and deserialization using singleton_map_recursive
    #[test]
    fn test_singleton_map_recursive() {
        // Define enum NestedEnum and struct TestStruct for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum NestedEnum {
            Variant(MyEnum),
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            #[serde(with = "singleton_map_recursive")]
            field: NestedEnum,
        }

        // Test serialization and deserialization for NestedEnum::Variant(MyEnum::Unit)
        let test_struct = TestStruct {
            field: NestedEnum::Variant(MyEnum::Unit),
        };
        let yaml = serde_yml::to_string(&test_struct).unwrap();
        assert_eq!(yaml, "field:\n  Variant: Unit\n");
        let deserialized: TestStruct =
            serde_yml::from_str(&yaml).unwrap();
        assert_eq!(test_struct, deserialized);
    }

    // Test top-level singleton_map_recursive serialization and deserialization
    #[test]
    fn test_singleton_map_recursive_top_level() {
        // Test serialization and deserialization for MyEnum::Unit
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");

        let deserialized: MyEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }

    // Test singleton_map serialization
    #[test]
    fn test_singleton_map_serialization() {
        // Test serialization for each variant of MyEnum
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");

        let value = MyEnum::Newtype(42);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Newtype: 42\n");

        let value = MyEnum::Tuple(1, 2);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Tuple:\n- 1\n- 2\n");

        let value = MyEnum::Struct { value: 42 };
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Struct:\n  value: 42\n");
    }

    // Test singleton_map deserialization
    #[test]
    fn test_singleton_map_deserialization() {
        // Test deserialization for each variant of MyEnum
        let yaml = "Unit\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Unit);

        let yaml = "Newtype: 42\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Newtype(42));

        let yaml = "Tuple:\n- 1\n- 2\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Tuple(1, 2));

        let yaml = "Struct:\n  value: 42\n";
        let deserialized: MyEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Struct { value: 42 });
    }

    // Test singleton_map_optional serialization
    #[test]
    fn test_singleton_map_optional_serialization() {
        // Test serialization for Some(MyEnum::Unit) and None
        let value = Some(MyEnum::Unit);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_optional::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");

        let value: Option<MyEnum> = None;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_optional::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "null\n");
    }

    // Test singleton_map_optional deserialization
    #[test]
    fn test_singleton_map_optional_deserialization() {
        // Test deserialization for Some(MyEnum::Unit) and None
        let yaml = "Unit\n";
        let deserialized: Option<MyEnum> =
            singleton_map_optional::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, Some(MyEnum::Unit));

        let yaml = "null\n";
        let deserialized: Option<MyEnum> =
            singleton_map_optional::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, None);
    }

    // Test singleton_map_with serialization
    #[test]
    fn test_singleton_map_with_serialization() {
        // Test serialization for MyEnum::Unit
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_with::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");
    }

    // Test singleton_map_with deserialization
    #[test]
    fn test_singleton_map_with_deserialization() {
        // Test deserialization for MyEnum::Unit
        let yaml = "Unit\n";
        let deserialized: MyEnum = singleton_map_with::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        )
        .unwrap();
        assert_eq!(deserialized, MyEnum::Unit);
    }

    // Test singleton_map_recursive serialization
    #[test]
    fn test_singleton_map_recursive_serialization() {
        // Define enum NestedEnum for serialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum NestedEnum {
            Variant(MyEnum),
        }

        // Test serialization for NestedEnum::Variant(MyEnum::Unit)
        let value = NestedEnum::Variant(MyEnum::Unit);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant: Unit\n");
    }

    // Test singleton_map_recursive deserialization
    #[test]
    fn test_singleton_map_recursive_deserialization() {
        // Define enum NestedEnum for deserialization
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum NestedEnum {
            Variant(MyEnum),
        }

        // Test deserialization for NestedEnum::Variant(MyEnum::Unit)
        let yaml = "Variant: Unit\n";
        let deserialized: NestedEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, NestedEnum::Variant(MyEnum::Unit));
    }

    // Test top-level singleton_map_recursive serialization
    #[test]
    fn test_singleton_map_recursive_top_level_serialization() {
        // Test serialization for MyEnum::Unit
        let value = MyEnum::Unit;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Unit\n");
    }

    // Test top-level singleton_map_recursive deserialization
    #[test]
    fn test_singleton_map_recursive_top_level_deserialization() {
        // Test deserialization for MyEnum::Unit
        let yaml = "Unit\n";
        let deserialized: MyEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            )
            .unwrap();
        assert_eq!(deserialized, MyEnum::Unit);
    }
    // Tests for error handling
    #[test]
    fn test_singleton_map_deserialization_error() {
        // Test deserialization error for invalid YAML input
        let yaml = "InvalidYAML";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_singleton_map_missing_field_error() {
        // Test deserialization error for missing field
        let yaml = "MissingField: 42";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(result.is_err());
    }

    // Tests for edge cases
    #[test]
    fn test_empty_enum() {
        // Define an enum with a single variant
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum SingleVariantEnum {
            Variant,
        }

        // Test serialization and deserialization of the single-variant enum
        let value = SingleVariantEnum::Variant;
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant\n");

        let deserialized: SingleVariantEnum =
            singleton_map::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }
    #[test]
    fn test_generic_enum() {
        // Define an enum with generic type parameters
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum GenericEnum<T> {
            Variant(T),
        }

        // Test serialization and deserialization of the generic enum
        let value = GenericEnum::Variant(42);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();
        assert_eq!(yaml, "Variant: 42\n");

        let deserialized: GenericEnum<i32> =
            singleton_map::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }
    #[test]
    fn test_singleton_map_unknown_variant_error() {
        // Attempt to deserialize a map with an unknown variant.
        let yaml = "NotARealVariant: 123\n";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
            result.is_err(),
            "Deserializing unknown variant should fail"
        );
    }

    #[test]
    fn test_singleton_map_multiple_keys_error() {
        // Attempt to deserialize a map with multiple entries; only one is allowed.
        let yaml = "Newtype: 42\nAnotherVariant: 99\n";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
            result.is_err(),
            "Deserializing multiple keys should fail"
        );
    }

    #[test]
    fn test_singleton_map_zero_key_map_error() {
        // Attempt to deserialize an empty map, which can't match a valid variant.
        let yaml = "{}\n";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
            result.is_err(),
            "Deserializing an empty map should fail"
        );
    }

    #[test]
    fn test_singleton_map_non_map_for_variant_error() {
        // Attempt to deserialize a sequence or other structure in place of a map.
        let yaml = "- 1\n- 2\n";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
        result.is_err(),
        "Deserializing a non-map structure for an enum variant should fail"
    );
    }

    #[test]
    fn test_singleton_map_optional_multiple_keys_error() {
        // For `Option<MyEnum>`, a map with multiple entries is also invalid.
        let yaml = "Unit: 0\nNewtype: 1\n";
        let result: Result<Option<MyEnum>, _> =
            singleton_map_optional::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            );
        assert!(
        result.is_err(),
        "Deserializing multiple keys into Option<MyEnum> should fail"
    );
    }

    #[test]
    fn test_singleton_map_with_unknown_variant_error() {
        // `singleton_map_with` just delegates to `singleton_map`, so unknown variant fails.
        let yaml = "UnknownStuff: 10\n";
        let result: Result<MyEnum, _> = singleton_map_with::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
            result.is_err(),
            "Unknown variant must fail for singleton_map_with"
        );
    }

    #[test]
    fn test_nested_singleton_map_multiple_keys_error() {
        // For nested enums, multiple keys at the outer level is not valid
        // in the `nested_singleton_map` approach.
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum InnerEnum {
            A,
            B(String),
        }
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum OuterEnum {
            Variant1(InnerEnum),
            Variant2 { x: InnerEnum },
        }

        let yaml = "Variant1: A\nVariant2:\n  x: B: Yikes!\n";
        let result: Result<OuterEnum, _> =
            nested_singleton_map::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            );
        assert!(
        result.is_err(),
        "Deserializing multiple keys in nested_singleton_map should fail"
    );
    }

    #[test]
    fn test_singleton_map_recursive_extra_map_key_error() {
        // Attempt to break recursion with an extra key in the nested map.
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Nested {
            A(MyEnum),
        }

        // This includes a valid variant plus an unexpected extra key.
        let yaml = "A:\n  Newtype: 42\n  ExtraKey: 55\n";
        let result: Result<Nested, _> =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            );
        assert!(
            result.is_err(),
            "Extra key inside nested map must cause an error"
        );
    }

    #[test]
    fn test_custom_deserializer_compatibility() {
        // Verifies that these modules still work with a manually constructed Deserializer.
        use serde_yml::Deserializer as YamlDeserializer;

        // We'll reuse `MyEnum` for brevity.
        let yaml = "Newtype: 64\n";
        let de = YamlDeserializer::from_str(yaml);
        let result: MyEnum = singleton_map::deserialize(de).unwrap();
        assert_eq!(result, MyEnum::Newtype(64));
    }

    #[test]
    fn test_empty_map_in_option_variant() {
        // Specifically test the case where an Option<T> is expected, but we get an empty map.
        // That should fail for `singleton_map_optional`.
        let yaml = "{}\n";
        let result: Result<Option<MyEnum>, _> =
            singleton_map_optional::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            );
        assert!(
            result.is_err(),
            "Deserializing an empty map into Option<MyEnum> must fail"
        );
    }

    #[test]
    fn test_nested_singleton_map_conflicting_structure() {
        // A deeper-level structure conflict in nested_singleton_map.
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum InnerEnum {
            A(i32),
            B,
        }
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum OuterEnum {
            OuterVariant { x: InnerEnum },
        }

        let yaml = r#"OuterVariant:
  x:
    B: 123
"#;
        // Here, variant B is a unit variant, but we've provided "123" as if it's a newtype.
        let result: Result<OuterEnum, _> =
            nested_singleton_map::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            );
        assert!(
            result.is_err(),
            "Providing sub-fields to a unit variant should fail"
        );
    }

    #[test]
    fn test_singleton_map_null_value_error() {
        // Test deserializing null value for non-Option enum
        let yaml = "Newtype: null\n";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
            result.is_err(),
            "Null value should fail for non-Option enum"
        );
    }

    #[test]
    fn test_singleton_map_wrong_variant_data_type() {
        // Test deserializing wrong data type for variant
        let yaml = "Newtype: \"not a number\"\n";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
            result.is_err(),
            "Wrong data type should fail deserialization"
        );
    }

    #[test]
    fn test_singleton_map_recursive_cyclic_reference() {
        // Define recursive enum structure
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum RecursiveEnum {
            Leaf(i32),
            Node(Box<RecursiveEnum>),
        }

        let value =
            RecursiveEnum::Node(Box::new(RecursiveEnum::Leaf(42)));
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();

        let deserialized: RecursiveEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_singleton_map_with_invalid_utf8() {
        // Test handling of invalid UTF-8 in strings
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum StringEnum {
            Text(String),
        }

        let yaml = "Text: \x7F\n"; // Valid UTF-8
        let result: Result<StringEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(
            result.is_err(),
            "Invalid UTF-8 should fail deserialization"
        );
    }

    #[test]
    fn test_singleton_map_optional_empty_string() {
        // Test handling of empty string in Option
        let yaml = "\"\"\n";
        let result: Result<Option<MyEnum>, _> =
            singleton_map_optional::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            );
        assert!(
            result.is_err(),
            "Empty string should fail for Option<MyEnum>"
        );
    }

    #[test]
    fn test_nested_singleton_map_missing_inner_field() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct InnerStruct {
            required: String,
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum ComplexEnum {
            Variant { data: InnerStruct },
        }

        let yaml = "Variant:\n  data: {}\n";
        let result: Result<ComplexEnum, _> =
            nested_singleton_map::deserialize(
                serde_yml::Deserializer::from_str(yaml),
            );
        assert!(
            result.is_err(),
            "Missing required inner field should fail"
        );
    }

    #[test]
    fn test_singleton_map_with_large_numbers() {
        // Test handling of numbers at the boundaries
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum NumberEnum {
            Big(i64),
            Small(i64),
        }

        // Test serialization and deserialization of max i64
        let value = NumberEnum::Big(i64::MAX);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();

        let deserialized: NumberEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(&yaml),
        )
        .unwrap();
        assert_eq!(deserialized, NumberEnum::Big(i64::MAX));

        // Test serialization and deserialization of min i64
        let value = NumberEnum::Small(i64::MIN);
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();

        let deserialized: NumberEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(&yaml),
        )
        .unwrap();
        assert_eq!(deserialized, NumberEnum::Small(i64::MIN));

        // Test error case: number overflow
        let yaml = format!("Big: {}\n", u64::MAX);
        let result: Result<NumberEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(&yaml),
        );
        assert!(result.is_err(), "Number overflow should fail");
    }

    #[test]
    fn test_singleton_map_recursive_deep_nesting() {
        // Test very deep nesting of enums
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum DeepEnum {
            Next(Option<Box<DeepEnum>>),
            End,
        }

        // Create a deeply nested structure
        let mut value = DeepEnum::End;
        for _ in 0..100 {
            value = DeepEnum::Next(Some(Box::new(value)));
        }

        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();

        let deserialized: DeepEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_singleton_map_optional_complex_none() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum ComplexEnum {
            Variant { field: Option<MyEnum> },
        }

        // Test None handling in nested optional fields
        let value = ComplexEnum::Variant { field: None };
        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map::serialize(&value, &mut serializer).unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();

        let deserialized: ComplexEnum = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(&yaml),
        )
        .unwrap();
        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_unit_variant_with_data() {
        // Test attempting to provide data to a unit variant
        let yaml = "Unit: 42\n";
        let result: Result<MyEnum, _> = singleton_map::deserialize(
            serde_yml::Deserializer::from_str(yaml),
        );
        assert!(result.is_err(), "Data for unit variant should fail");
    }

    #[test]
    fn test_singleton_map_recursive_mixed_variants() {
        // Test mixing different variant types in recursive structures
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum MixedEnum {
            Unit,
            Newtype(Box<MixedEnum>),
            Struct { next: Option<Box<MixedEnum>> },
        }

        let value = MixedEnum::Newtype(Box::new(MixedEnum::Struct {
            next: Some(Box::new(MixedEnum::Unit)),
        }));

        let mut serializer = serde_yml::Serializer::new(Vec::new());
        singleton_map_recursive::serialize(&value, &mut serializer)
            .unwrap();
        let yaml = String::from_utf8(serializer.into_inner().unwrap())
            .unwrap();

        let deserialized: MixedEnum =
            singleton_map_recursive::deserialize(
                serde_yml::Deserializer::from_str(&yaml),
            )
            .unwrap();
        assert_eq!(value, deserialized);
    }
}
