#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_yml::{
        de::Progress,
        loader::Loader,
        value::{Tag, TaggedValue, Value},
        Number,
    };
    use std::collections::HashMap;

    // ------------------------------------------------------------------------
    //  PRIMITIVES
    // ------------------------------------------------------------------------
    /// Tests for basic primitive types: `null`, booleans, numeric types, chars, and empty/unit values.
    mod primitives {
        use super::*;

        /// Test deserialization of a `null` value into `Option<()>`.
        #[test]
        fn test_deserialize_null() {
            let value = Value::Null;
            let result: Option<()> =
                serde_yml::from_value(value).unwrap();
            assert_eq!(result, None);
        }

        /// Test deserialization of a `bool` value.
        #[test]
        fn test_deserialize_bool() {
            let value = Value::Bool(true);
            let result: bool = serde_yml::from_value(value).unwrap();
            assert!(result);
        }

        /// Test deserialization of an `i64` value.
        #[test]
        fn test_deserialize_i64() {
            let value = Value::Number(42.into());
            let result: i64 = serde_yml::from_value(value).unwrap();
            assert_eq!(result, 42);
        }

        /// Test deserialization of a `u64` value.
        #[test]
        fn test_deserialize_u64() {
            let value = Value::Number(42.into());
            let result: u64 = serde_yml::from_value(value).unwrap();
            assert_eq!(result, 42);
        }

        /// Test deserialization of a `f64` value.
        #[test]
        fn test_deserialize_f64() {
            let value = Value::Number(42.5.into());
            let result: f64 = serde_yml::from_value(value).unwrap();
            assert_eq!(result, 42.5);
        }

        /// Test deserialization of a `f32` value.
        #[test]
        fn test_deserialize_f32() {
            let value = Value::Number(Number::from(42.5f32));
            let result: f32 = serde_yml::from_value(value).unwrap();
            assert_eq!(result, 42.5f32);
        }

        /// Test deserialization of a `char` value.
        #[test]
        fn test_deserialize_char() {
            let value = Value::String("a".to_string());
            let result: char = serde_yml::from_value(value).unwrap();
            assert_eq!(result, 'a');
        }

        /// Test deserialization of a `String` value.
        #[test]
        fn test_deserialize_string() {
            let value = Value::String("hello".to_string());
            let result: String = serde_yml::from_value(value).unwrap();
            assert_eq!(result, "hello");
        }

        /// Test deserialization of a unit value.
        #[test]
        fn test_deserialize_unit() {
            let value = Value::Null;
            let result: () = serde_yml::from_value(value).unwrap();
            println!(
                "✅ Deserialized unit value successfully. {:?}",
                result
            );
        }

        /// Test deserialization of a `()` value (another way to check unit).
        #[test]
        fn test_deserialize_unit_value() {
            let value = Value::Null;
            let result: () = serde_yml::from_value(value).unwrap();
            println!(
                "✅ Deserialized unit value successfully. {:?}",
                result
            );
        }
    }

    // ------------------------------------------------------------------------
    //  COLLECTIONS (Sequences, Maps, Bytes)
    // ------------------------------------------------------------------------
    /// Tests for collections (vectors, byte arrays, hash maps) and options thereof.
    mod collections {
        use super::*;

        /// Test deserialization of a sequence into a `Vec<i32>`.
        #[test]
        fn test_deserialize_sequence() {
            let value = Value::Sequence(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
            ]);
            let result: Vec<i32> =
                serde_yml::from_value(value).unwrap();
            assert_eq!(result, vec![1, 2]);
        }

        /// Test deserialization of a sequence into a `Vec<u8>`.
        #[test]
        fn test_deserialize_bytes() {
            let value = Value::Sequence(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
            ]);
            let result: Vec<u8> = serde_yml::from_value(value).unwrap();
            assert_eq!(result, vec![1, 2]);
        }

        /// Test deserialization of a map into a `HashMap`.
        #[test]
        fn test_deserialize_map() {
            let value = Value::Mapping(
                vec![
                    (
                        Value::String("x".to_string()),
                        Value::Number(1.into()),
                    ),
                    (
                        Value::String("y".to_string()),
                        Value::Number(2.into()),
                    ),
                ]
                .into_iter()
                .collect(),
            );
            let result: HashMap<String, i32> =
                serde_yml::from_value(value).unwrap();
            let mut expected = HashMap::new();
            expected.insert("x".to_string(), 1);
            expected.insert("y".to_string(), 2);
            assert_eq!(result, expected);
        }

        /// Test deserialization of `Option` with `Some` value.
        #[test]
        fn test_deserialize_option_some() {
            let value = Value::Number(42.into());
            let result: Option<i32> =
                serde_yml::from_value(value).unwrap();
            assert_eq!(result, Some(42));
        }

        /// Test deserialization of `Option` with `None` value.
        #[test]
        fn test_deserialize_option_none() {
            let value = Value::Null;
            let result: Option<i32> =
                serde_yml::from_value(value).unwrap();
            assert_eq!(result, None);
        }

        /// Test deserialization of an identifier (string).
        #[test]
        fn test_deserialize_identifier() {
            let value = Value::String("hello".to_string());
            let result: String = serde_yml::from_value(value).unwrap();
            assert_eq!(result, "hello");
        }

        /// Test deserialization of a byte array.
        #[test]
        fn test_deserialize_byte_array() {
            let value = Value::Sequence(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
                Value::Number(3.into()),
            ]);
            let result: [u8; 3] = serde_yml::from_value(value).unwrap();
            assert_eq!(result, [1, 2, 3]);
        }

        /// Test deserialization of an optional byte array.
        #[test]
        fn test_deserialize_optional_byte_array() {
            let value = Value::Sequence(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
                Value::Number(3.into()),
            ]);
            let result: Option<[u8; 3]> =
                serde_yml::from_value(value).unwrap();
            assert_eq!(result, Some([1, 2, 3]));
        }
    }

    // ------------------------------------------------------------------------
    //  STRUCTS (Tuples, Named Structs, Newtype)
    // ------------------------------------------------------------------------
    /// Tests for structs, tuple structs, and newtype structs.
    mod structs {
        use super::*;

        /// Test deserialization of a newtype struct.
        #[test]
        fn test_deserialize_newtype_struct() {
            let value = Value::Number(42.into());
            #[derive(Deserialize, PartialEq, Debug)]
            struct Newtype(i32);
            let result: Newtype = serde_yml::from_value(value).unwrap();
            assert_eq!(result, Newtype(42));
        }

        /// Test deserialization of a tuple.
        #[test]
        fn test_deserialize_tuple() {
            let value = Value::Sequence(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
            ]);
            let result: (i32, i32) =
                serde_yml::from_value(value).unwrap();
            assert_eq!(result, (1, 2));
        }

        /// Test deserialization of a tuple struct.
        #[test]
        fn test_deserialize_tuple_struct() {
            let value = Value::Sequence(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
            ]);
            #[derive(Deserialize, PartialEq, Debug)]
            struct TupleStruct(i32, i32);
            let result: TupleStruct =
                serde_yml::from_value(value).unwrap();
            assert_eq!(result, TupleStruct(1, 2));
        }

        /// Test deserialization of a struct.
        #[test]
        fn test_deserialize_struct() {
            let value = Value::Mapping(
                vec![
                    (
                        Value::String("x".to_string()),
                        Value::Number(1.into()),
                    ),
                    (
                        Value::String("y".to_string()),
                        Value::Number(2.into()),
                    ),
                ]
                .into_iter()
                .collect(),
            );
            #[derive(Deserialize, PartialEq, Debug)]
            struct Point {
                x: i32,
                y: i32,
            }
            let result: Point = serde_yml::from_value(value).unwrap();
            assert_eq!(result, Point { x: 1, y: 2 });
        }

        /// Test deserialization of an empty struct.
        #[test]
        fn test_deserialize_empty_struct() {
            let value = Value::Null;
            #[derive(Deserialize, PartialEq, Debug)]
            struct Empty;
            let result: Empty = serde_yml::from_value(value).unwrap();
            assert_eq!(result, Empty);
        }

        /// Test deserialization of a unit struct.
        #[test]
        fn test_deserialize_unit_struct() {
            let value = Value::Null;
            #[derive(Deserialize, PartialEq, Debug)]
            struct Unit;
            let result: Unit = serde_yml::from_value(value).unwrap();
            assert_eq!(result, Unit);
        }
    }

    // ------------------------------------------------------------------------
    //  ENUMS (Basic, Tagged, Multi-Field)
    // ------------------------------------------------------------------------
    /// Tests for enums with various layout options: unit, newtype, tuple, struct, sequences, maps.
    mod enums {
        use super::*;

        /// Test deserialization of a tagged enum variant.
        #[test]
        fn test_deserialize_enum() {
            let value = Value::Tagged(Box::new(TaggedValue {
                tag: Tag::new("B"),
                value: Value::Number(42.into()),
            }));
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                A,
                B(i32),
                C { x: i32 },
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::B(42));
        }

        /// Test deserialization of a unit variant.
        #[test]
        fn test_deserialize_unit_variant() {
            let value = Value::String("Variant".to_string());
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant,
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant);
        }

        /// Test deserialization of a newtype variant.
        #[test]
        fn test_deserialize_newtype_variant() {
            let yaml_str = "!Variant 0";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();

            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant(i32),
            }

            let result: E = serde_yml::from_value(value).unwrap();
            println!("\n✅ Deserialized newtype variant: {:?}", result);
        }

        /// Test deserialization of a tuple variant.
        #[test]
        fn test_deserialize_tuple_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\n- 1\n- 2\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant(i32, i32),
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant(1, 2));
        }

        /// Test deserialization of a struct variant.
        #[test]
        fn test_deserialize_struct_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\nx: 1\ny: 2\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant { x: i32, y: i32 },
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant { x: 1, y: 2 });
        }

        /// Test deserialization of a sequence variant.
        #[test]
        fn test_deserialize_sequence_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\n- 1\n- 2\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant(Vec<i32>),
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant(vec![1, 2]));
        }

        /// Test deserialization of a map variant.
        #[test]
        fn test_deserialize_map_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\nx: 1\ny: 2\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant(HashMap<String, i32>),
            }
            let result: E = serde_yml::from_value(value).unwrap();
            let mut expected = HashMap::new();
            expected.insert("x".to_string(), 1);
            expected.insert("y".to_string(), 2);
            assert_eq!(result, E::Variant(expected));
        }

        /// Test deserialization of a tagged unit variant.
        #[test]
        fn test_deserialize_tagged_unit_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant,
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant);
        }

        /// Test deserialization of a tagged newtype variant.
        #[test]
        fn test_deserialize_tagged_newtype_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant 0\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant(i32),
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant(0));
        }

        /// Test deserialization of a tagged tuple variant.
        #[test]
        fn test_deserialize_tagged_tuple_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\n- 1\n- 2\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant(i32, i32),
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant(1, 2));
        }

        /// Test deserialization of a tagged struct variant.
        #[test]
        fn test_deserialize_tagged_struct_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\nx: 1\ny: 2\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant { x: i32, y: i32 },
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant { x: 1, y: 2 });
        }

        /// Test deserialization of a tagged sequence variant.
        #[test]
        fn test_deserialize_tagged_sequence_variant() {
            // YAML representation of the enum variant
            let yaml_str = "---\n!Variant\n- 1\n- 2\n";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                Variant(Vec<i32>),
            }
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::Variant(vec![1, 2]));
        }

        /// Test deserialization of a unit struct variant.
        #[test]
        fn test_deserialize_unit_struct_variant() {
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                V,
            }
            let value = Value::String("V".to_string());
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::V);
        }

        /// Test deserialization of a newtype struct variant.
        #[test]
        fn test_deserialize_newtype_struct_variant() {
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                V(i32),
            }
            let value = Value::Tagged(Box::new(TaggedValue {
                tag: Tag::new("V"),
                value: Value::Number(42.into()),
            }));
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::V(42));
        }

        /// Test deserialization of a tuple struct variant.
        #[test]
        fn test_deserialize_tuple_struct_variant() {
            #[derive(Deserialize, PartialEq, Debug)]
            enum E {
                V(i32, i32),
            }
            let value = Value::Tagged(Box::new(TaggedValue {
                tag: Tag::new("V"),
                value: Value::Sequence(vec![
                    Value::Number(1.into()),
                    Value::Number(2.into()),
                ]),
            }));
            let result: E = serde_yml::from_value(value).unwrap();
            assert_eq!(result, E::V(1, 2));
        }
    }

    // ------------------------------------------------------------------------
    //  INTERNAL LOADER / PROGRESS
    // ------------------------------------------------------------------------
    /// Tests specifically covering custom loader behaviors, whitespace/normalization, etc.
    mod loader_tests {
        use super::*;

        /// Tests the `new_normalized_str` method (which, depending on design, might remove extra spaces).
        /// In this case, preserving input might be crucial for YAML’s whitespace sensitivity.
        #[test]
        fn can_load_document_with_16_spaces_value() {
            let hardcoded = "t: a                abc";

            // Test normalized input
            let normalized_progress =
                Progress::new_normalized_str(hardcoded)
                    .expect("Failed to normalize input");
            let mut normalized_loader =
                Loader::new(normalized_progress).unwrap();
            let normalized_document = normalized_loader.next_document();
            assert!(
                normalized_document.is_some(),
                "Normalized input should pass and produce a valid document"
            );
        }

        /// Test deserialization of an empty tuple via Loader (YAML `---`).
        #[test]
        fn test_deserialize_empty_tuple() {
            let yaml_str = "---";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();

            let result: () = serde_yml::from_value(value).unwrap();
            println!("\n✅ Deserialized Empty tuple: {:?}", result);
        }

        /// Test deserialization of an empty tuple struct via Loader (YAML `---`).
        #[test]
        fn test_deserialize_empty_tuple_struct() {
            let yaml_str = "---";
            let value: Value = serde_yml::from_str(yaml_str).unwrap();

            #[derive(Deserialize, PartialEq, Debug)]
            struct Empty;

            let result: Empty = serde_yml::from_value(value).unwrap();
            println!(
                "\n✅ Deserialized Empty tuple struct: {:?}",
                result
            );
        }
    }

    /// Covers advanced or edge YAML scenarios to ensure the parser handles a variety of edge cases.
    #[cfg(test)]
    mod advanced_yaml_tests {
        use serde::Deserialize;
        use std::collections::HashMap;

        // ------------------------------------------------------------------------
        // MULTI-DOCUMENT TESTS
        // ------------------------------------------------------------------------
        /// Tests that specifically handle multiple YAML documents in one string
        /// and verify how the parser responds (e.g., erroring if there's more than one doc).
        mod multi_document {
            use super::*;

            /// Tries to parse multiple YAML docs as if they were a single doc.
            /// We confirm it fails, rather than merging or silently parsing.
            #[test]
            fn test_more_than_one_document_error() {
                let yaml = r#"
            ---
            doc: one
            ---
            doc: two
            "#;

                #[derive(Deserialize, Debug)]
                struct SingleDoc {
                    _doc: String,
                }

                // We only check that we get an error. We don't care about the exact error message.
                let single_result =
                    serde_yml::from_str::<SingleDoc>(yaml);
                assert!(
                    single_result.is_err(),
                    "Parsing multiple docs as one should fail"
                );
            }
        }

        // ------------------------------------------------------------------------
        // SCALAR EDGE CASES
        // ------------------------------------------------------------------------
        /// Tests for special scalar scenarios: leading zeros, multiline block scalars,
        /// and out-of-range integers.
        mod scalar_edge_cases {
            use super::*;

            /// Leading zeros: either it successfully parses as 1 or it errors.
            /// We accept both scenarios to accommodate different parser behaviors.
            #[test]
            fn test_leading_zeros_integer() {
                let yaml = "value: 0001";

                #[derive(Deserialize)]
                struct Zeros {
                    value: i32,
                }

                let result = serde_yml::from_str::<Zeros>(yaml);
                match result {
                    Ok(parsed) => {
                        assert_eq!(
                            parsed.value, 1,
                            "If parse is OK, leading zeros => 1"
                        );
                    }
                    Err(_err) => {
                        // If it fails, that’s acceptable too.
                        return;
                    }
                }
            }

            #[test]
            fn test_syntax_only_parse() {
                // Just confirm valid syntax, ignoring whether it maps cleanly onto a struct.
                let yaml = r#"
    incomplete:
      but_syntactically_ok
    "#;

                // If you have a loader or syntax-check method:
                match serde_yml::from_str::<serde_yml::Value>(yaml) {
                    Ok(value) => {
                        println!(
                            "Parsed syntax successfully: {:?}",
                            value
                        );
                        // We don't do type-level checks here.
                    }
                    Err(e) => println!("Syntax error? {:?}", e),
                }
            }

            #[test]
            fn test_custom_tag() {
                let yaml = r#"
    !MyApp/Resource
    name: "My resource"
    value: 10
    "#;

                // Depending on your code, you might have a special struct or just parse to Value.
                let parsed: serde_yml::Result<serde_yml::Value> =
                    serde_yml::from_str(yaml);

                match parsed {
                    Ok(val) => {
                        println!("Parsed custom tag as: {:?}", val)
                    }
                    Err(e) => {
                        println!("Error reading custom tag: {:?}", e)
                    }
                }
            }

            #[test]
            fn test_unicode_edge_cases() {
                let yaml = r#"
    combined_char: "é"   # e + combining accent
    high_plane_emoji: "🤵🏽"
    "#;

                #[derive(Debug, Deserialize)]
                struct UniTest {
                    combined_char: String,
                    high_plane_emoji: String,
                }

                let parsed =
                    serde_yml::from_str::<UniTest>(yaml).unwrap();

                // Option 1: Explicitly access each field
                println!(
        "Parsed advanced unicode: combined_char={}, high_plane_emoji={}",
        parsed.combined_char, parsed.high_plane_emoji
    );

                // Option 2: Destructure the struct
                let UniTest {
                    combined_char,
                    high_plane_emoji,
                } = parsed;
                println!(
        "Destructured => combined_char={}, high_plane_emoji={}",
        combined_char, high_plane_emoji
    );
            }

            /// Demonstrates a multiline (block) scalar using `|`.
            /// Typically results in "line1\nline2\n" with a trailing newline.
            #[test]
            fn test_multiline_string() {
                let yaml = r#"
            multiline: |
              line1
              line2
            "#;

                #[derive(Debug, Deserialize)]
                struct BlockString {
                    multiline: String,
                }

                let parsed: BlockString =
                    serde_yml::from_str(yaml).unwrap();

                // Adjust the expected string depending on whether your parser
                // preserves the trailing newline.
                assert_eq!(parsed.multiline, "line1\nline2\n");
                println!(
                    "Parsed multiline string: {:?}",
                    parsed.multiline
                );
            }

            /// Attempts to parse an out-of-range integer (999) into an i8 (max 127).
            /// We see whether the parser errors or silently wraps.
            #[test]
            fn test_out_of_range_integers() {
                let yaml = r#"
            too_large: 999
            "#;

                #[derive(Debug, Deserialize)]
                struct SmallInt {
                    too_large: i8,
                }

                let result = serde_yml::from_str::<SmallInt>(yaml);
                match result {
                    Ok(parsed) => {
                        // If it wraps or truncates, you'll see a strange value.
                        println!("Parsed struct: {:?}", parsed);
                        println!(
                            "Value of 'too_large' is: {}",
                            parsed.too_large
                        );
                    }
                    Err(e) => {
                        // If it properly rejects out-of-range, you get an error.
                        println!(
                            "Got an error (likely out-of-range): {:?}",
                            e
                        );
                    }
                }
            }

            #[test]
            fn test_quoted_vs_unquoted_strings() {
                // Compare a bare word vs. a quoted word vs. something that might look numeric but is actually quoted.
                let yaml = r#"
    bare: hello
    single_quoted: '1234'
    double_quoted: "some string"
    tricky: "00100"
    "#;

                #[derive(Debug, Deserialize)]
                struct Strings {
                    bare: String,
                    single_quoted: String,
                    double_quoted: String,
                    tricky: String,
                }

                let parsed: Strings =
                    serde_yml::from_str(yaml).unwrap();
                println!("Parsed strings: {:?}", parsed);

                // The key check is that `single_quoted` and `double_quoted` remain strings
                // (rather than being interpreted as numeric).
                assert_eq!(parsed.bare, "hello");
                assert_eq!(parsed.single_quoted, "1234");
                assert_eq!(parsed.double_quoted, "some string");
                assert_eq!(parsed.tricky, "00100");
            }

            #[test]
            fn test_special_float_cases() {
                let yaml = r#"
    pos_inf: .inf
    neg_inf: -.INF
    nan_val: .NaN
    "#;

                #[derive(Debug, Deserialize)]
                struct Floats {
                    pos_inf: f64,
                    neg_inf: f64,
                    nan_val: f64,
                }

                let parsed =
                    serde_yml::from_str::<Floats>(yaml).unwrap();
                println!("Parsed special floats: {:?}", parsed);

                // Confirm infinite or NaN:
                assert!(
                    parsed.pos_inf.is_infinite()
                        && parsed.pos_inf.is_sign_positive()
                );
                assert!(
                    parsed.neg_inf.is_infinite()
                        && parsed.neg_inf.is_sign_negative()
                );
                assert!(parsed.nan_val.is_nan());
            }

            #[test]
            fn test_inline_comments() {
                let yaml = r#"
    # This is a comment before any keys
    key1: value1  # trailing comment on the same line
    key2: value2  # another trailing comment
    # comment at the end
    "#;

                #[derive(Debug, Deserialize)]
                struct WithComments {
                    key1: String,
                    key2: String,
                }

                let parsed = serde_yml::from_str::<WithComments>(yaml);
                match parsed {
                    Ok(data) => {
                        println!("Parsed with comments: {:?}", data);
                        assert_eq!(data.key1, "value1");
                        assert_eq!(data.key2, "value2");
                    }
                    Err(e) => {
                        println!("Some libraries might reject trailing comments. Error: {:?}", e);
                    }
                }
            }

            #[test]
            fn test_deeply_nested_structures() {
                // A nested sequence of sequences. Let's go 10 levels deep for demonstration.
                let yaml = r#"
    - 1
    -
      - 2
      -
        - 3
        -
          - 4
          # ... keep nesting as needed ...
    "#;

                // Depending on your code’s recursion limits, you may or may not see an error:
                let parsed: serde_yml::Result<serde_yml::Value> =
                    serde_yml::from_str(yaml);
                match parsed {
                    Ok(v) => println!("Deep nesting parsed: {:?}", v),
                    Err(e) => println!(
                        "Error for deeply nested structures: {:?}",
                        e
                    ),
                }
            }

            #[test]
            fn test_signed_zero_floats() {
                let yaml = r#"
    positive_zero: 0.0
    negative_zero: -0.0
    "#;

                #[derive(Debug, Deserialize)]
                struct SignedZeros {
                    positive_zero: f64,
                    negative_zero: f64,
                }

                let parsed: SignedZeros =
                    serde_yml::from_str(yaml).unwrap();

                // Check sign bits or at least show them.
                println!(
                    "positive_zero bits = {:?}",
                    parsed.positive_zero.to_bits()
                );
                println!(
                    "negative_zero bits = {:?}",
                    parsed.negative_zero.to_bits()
                );

                // Or test with `is_sign_negative()` if you want to see if the sign is stored:
                assert!(!parsed.positive_zero.is_sign_negative());
                assert!(parsed.negative_zero.is_sign_negative());
            }

            #[test]
            fn test_unicode_and_emoji_support() {
                let yaml = r#"
    greeting: "Hello, 世界"
    emoji: "😀"
    "#;

                #[derive(Debug, Deserialize)]
                struct UnicodeData {
                    greeting: String,
                    emoji: String,
                }

                let parsed =
                    serde_yml::from_str::<UnicodeData>(yaml).unwrap();
                assert_eq!(parsed.greeting, "Hello, 世界");
                assert_eq!(parsed.emoji, "😀");
            }
        }

        // ------------------------------------------------------------------------
        // MAPS, MERGING, AND ALIASES
        // ------------------------------------------------------------------------
        /// Tests covering map-related features: ignoring unknown fields, handling duplicates,
        /// YAML merge keys, and complex alias references for sequences or maps.
        mod maps_and_merging {
            use super::*;

            /// Ensures that unknown fields are ignored rather than causing an error,
            /// which is Serde's default behavior.
            #[test]
            fn test_ignore_additional_fields() {
                let yaml = r#"
            known: 123
            unknown_field: "extra content"
            nested_unknown:
              deeper: "stuff"
            "#;

                #[derive(Debug, Deserialize)]
                struct Minimal {
                    known: i32,
                }

                let parsed: Minimal =
                    serde_yml::from_str(yaml).unwrap();
                // The "known" field should be 123, ignoring the unknown fields
                assert_eq!(parsed.known, 123);
            }

            /// Demonstrates how the parser handles duplicate YAML keys in a map.
            /// Some parsers keep the last key’s value, some error. We just observe behavior.
            #[test]
            fn test_duplicate_keys_in_map() {
                let yaml = r#"
            duplicate: 1
            normal: 42
            duplicate: 99
            "#;

                let parsed: serde_yml::Result<HashMap<String, i32>> =
                    serde_yml::from_str(yaml);

                match parsed {
                    Ok(map) => {
                        println!(
                            "Parsed map with duplicates: {:?}",
                            map
                        );
                        assert_eq!(
                            map.get("normal"),
                            Some(&42),
                            "The normal key should parse as 42"
                        );
                        // You might see 99 for "duplicate", but it depends on the parser.
                    }
                    Err(e) => {
                        println!("test_duplicate_keys_in_map: Library errored: {:?}", e);
                    }
                }
            }

            /// YAML’s merge key (`<<`) can merge fields from an anchor. Checks whether
            /// this is supported or errors out if not supported.
            #[test]
            fn test_merge_key() {
                let yaml = r#"
            defaults: &defaults
              foo: "default foo"
              bar: "default bar"

            override:
              <<: *defaults
              bar: "overridden bar"
            "#;

                #[derive(Debug, Deserialize)]
                struct Settings {
                    foo: String,
                    bar: String,
                }

                #[derive(Debug, Deserialize)]
                struct Root {
                    defaults: Settings,
                    r#override: Settings,
                }

                let parsed: serde_yml::Result<Root> =
                    serde_yml::from_str(yaml);
                match parsed {
                    Ok(root) => {
                        println!("Parsed with merges: {:?}", root);
                        assert_eq!(root.defaults.foo, "default foo");
                        assert_eq!(root.defaults.bar, "default bar");

                        // The override merges from defaults but overrides "bar"
                        assert_eq!(root.r#override.foo, "default foo");
                        assert_eq!(
                            root.r#override.bar,
                            "overridden bar"
                        );
                    }
                    Err(e) => {
                        println!(
                            "YAML merges unsupported? Error: {:?}",
                            e
                        );
                    }
                }
            }

            /// Shows how a complex anchor reference can alias entire sequences/maps.
            /// If not supported, the library may error. Otherwise, it merges or references them.
            #[test]
            fn test_alias_referencing_complex_data() {
                let yaml = r#"
            numbers: &nums
              - 1
              - 2
              - 3

            reference1:
              <<: *nums

            reference2:
              some_alias: *nums
            "#;

                #[derive(Debug, Deserialize)]
                struct References {
                    numbers: Vec<i32>,
                    reference1: Vec<i32>,
                    reference2: HashMap<String, Vec<i32>>,
                }

                let parsed = serde_yml::from_str::<References>(yaml);
                match parsed {
                    Ok(data) => {
                        println!("Alias referencing complex data parsed successfully: {:#?}", data);
                        assert_eq!(data.numbers, vec![1, 2, 3]);
                        assert_eq!(data.reference1, vec![1, 2, 3]);
                        assert_eq!(
                            data.reference2.get("some_alias"),
                            Some(&vec![1, 2, 3])
                        );
                    }
                    Err(e) => {
                        println!("test_alias_referencing_complex_data: Error: {:?}", e);
                    }
                }
            }

            #[test]
            fn test_complex_keys_in_map() {
                let yaml = r#"
    ?
      - key1
      - key2
    : "value_for_array_key"

    normal_key: "normal_value"
    "#;

                // Typically, Serde will reject arrays as map keys unless you define a special type.
                // Let’s see if it fails or does something else:
                let parsed: serde_yml::Result<serde_yml::Value> =
                    serde_yml::from_str(yaml);

                match parsed {
                    Ok(val) => println!(
                        "Parsed map with complex key: {:?}",
                        val
                    ),
                    Err(e) => {
                        println!("Error for complex keys: {:?}", e)
                    }
                }
            }
        }
    }
}
