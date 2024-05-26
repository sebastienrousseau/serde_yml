#[cfg(test)]
mod tests {
    use serde::{ser::Serializer as _, Serialize};
    use serde_yml::{
        libyml::emitter::{Scalar, ScalarStyle},
        Serializer, State,
    };
    use std::{collections::BTreeMap, fmt::Write};

    // Test cases for scalar serialization
    #[test]
    fn test_scalar_serialization() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let scalar_value = Scalar {
            tag: None,
            value: "test value",
            style: ScalarStyle::Plain,
        };

        // Act
        serializer.emit_scalar(scalar_value).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "test value\n",
            "Serialized scalar value doesn't match expected output"
        );
    }

    // Test cases for sequence start serialization
    #[test]
    fn test_sequence_start_serialization() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        serializer.emit_sequence_start().unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "",
            "Serialized sequence start doesn't match expected output"
        );
    }

    // Test cases for mapping start serialization
    #[test]
    fn test_mapping_start_serialization() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        serializer.emit_mapping_start().unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "",
            "Serialized mapping start doesn't match expected output"
        );
    }

    // Test cases for flushing mapping start
    #[test]
    fn test_flush_mapping_start() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.state = State::CheckForTag;

        // Act
        serializer.flush_mapping_start().unwrap();

        // Assert
        assert_eq!(
            buffer,
            b"", // Since state was `CheckForTag`, no output should be generated
            "Flush mapping start output doesn't match expected output"
        );
    }

    // Test cases for taking tag with found tag state
    #[test]
    fn test_take_tag_with_found_tag_state() {
        // Arrange
        let mut serializer = Serializer::<Vec<u8>>::new(Vec::new());
        serializer.state = State::FoundTag("test tag".to_owned());

        // Act
        let tag = serializer.take_tag();

        // Assert
        assert_eq!(
            tag,
            Some("!test tag".to_owned()), // Found tag should be prefixed with '!'
            "Tag extraction output doesn't match expected output"
        );
    }

    // Test cases for taking tag with no state
    #[test]
    fn test_take_tag_with_no_state() {
        // Arrange
        let mut serializer = Serializer::<Vec<u8>>::new(Vec::new());

        // Act
        let tag = serializer.take_tag();

        // Assert
        assert_eq!(
            tag,
            None, // Since there was no specific state, tag extraction should return None
            "Tag extraction output doesn't match expected output"
        );
    }

    // Test cases for converting into inner
    #[test]
    fn test_into_inner() {
        // Arrange
        let mut buffer = Vec::new();
        let buffer_clone = buffer.clone();
        let serializer = Serializer::new(&mut buffer);

        // Act
        let result = serializer.into_inner().unwrap();

        // Assert
        assert_eq!(&*result, &buffer_clone);
    }

    // Test cases for serializing boolean values
    #[test]
    fn test_serialize_bool() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        serializer.serialize_bool(true).unwrap();
        serializer.serialize_bool(false).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "true\n--- false\n",
            "Serialized boolean values don't match expected output"
        );
    }

    // Test cases for serializing i64 values
    #[test]
    fn test_serialize_i64() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        serializer.serialize_i64(42).unwrap();
        serializer.serialize_i64(-100).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "42\n--- -100\n",
            "Serialized i64 values don't match expected output"
        );
    }

    // Test cases for serializing f64 values
    #[test]
    fn test_serialize_f64() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        serializer.serialize_f64(std::f64::consts::PI).unwrap();
        serializer.serialize_f64(f64::INFINITY).unwrap();
        serializer.serialize_f64(f64::NEG_INFINITY).unwrap();
        serializer.serialize_f64(f64::NAN).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "3.141592653589793\n--- .inf\n--- -.inf\n--- .nan\n",
            "Serialized f64 values don't match expected output"
        );
    }

    // Test cases for serializing char values
    #[test]
    fn test_serialize_char() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        serializer.serialize_char('a').unwrap();
        serializer.serialize_char('💻').unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "'a'\n--- '💻'\n",
            "Serialized char values don't match expected output"
        );
    }

    // Test cases for serializing Option values
    #[test]
    fn test_serialize_option() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;

        // Act
        some_value.serialize(&mut serializer).unwrap();
        none_value.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "42\n--- null\n",
            "Serialized Option values don't match expected output"
        );
    }

    // Test cases for serializing enum values
    #[test]
    fn test_serialize_enum() {
        // Arrange
        #[derive(Serialize)]
        enum MyEnum {
            A,
            B(i32),
            C { x: i32, y: i32 },
        }

        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        MyEnum::A.serialize(&mut serializer).unwrap();
        MyEnum::B(42).serialize(&mut serializer).unwrap();
        MyEnum::C { x: 1, y: 2 }.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "A\n--- !B 42\n--- !C\nx: 1\n'y': 2\n",
            "Serialized enum values don't match expected output"
        );
    }

    // Test cases for serializing sequences
    #[test]
    fn test_serialize_sequence() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let sequence = vec!["42", "hello", "true"];

        // Act
        sequence.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "- '42'\n- hello\n- 'true'\n",
            "Serialized sequence doesn't match expected output"
        );
    }

    // Test cases for serializing maps
    #[test]
    fn test_serialize_map() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let mut map = BTreeMap::new();
        map.insert("name", "John");
        map.insert("age", "30");

        // Act
        map.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "age: '30'\nname: John\n",
            "Serialized map doesn't match expected output"
        );
    }

    // Test cases for serializing nested structs
    #[test]
    fn test_serialize_nested_struct() {
        // Arrange
        #[derive(Serialize)]
        struct Person {
            name: String,
            age: u32,
            address: Address,
        }

        #[derive(Serialize)]
        struct Address {
            street: String,
            city: String,
        }

        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        let person = Person {
            name: "Alice".to_string(),
            age: 25,
            address: Address {
                street: "123 Main St".to_string(),
                city: "Anytown".to_string(),
            },
        };
        person.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "name: Alice\nage: 25\naddress:\n  street: '123 Main St'\n  city: Anytown\n",
            "Serialized nested struct doesn't match expected output"
        );
    }

    // Test cases for serializing optional fields
    #[test]
    fn test_serialize_optional_fields() {
        // Arrange
        #[derive(Serialize)]
        struct User {
            name: String,
            email: Option<String>,
            age: Option<u32>,
        }

        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        let user = User {
            name: "Bob".to_string(),
            email: Some("bob@example.com".to_string()),
            age: None,
        };
        user.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "name: Bob\nemail: bob@example.com\nage: null\n",
            "Serialized optional fields don't match expected output"
        );
    }

    // Test cases for serializing tagged value
    #[test]
    fn test_serialize_tagged_value() {
        // Arrange
        #[derive(Serialize)]
        struct TaggedValue {
            #[serde(rename = "!tag")]
            value: String,
        }

        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        let tagged_value = TaggedValue {
            value: "example".to_string(),
        };
        tagged_value.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "'!tag': example\n",
            "Serialized tagged value doesn't match expected output"
        );
    }

    // Test cases for serializing large data
    #[test]
    fn test_serialize_large_data() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let large_sequence: Vec<_> = (0..1000).collect();

        // Act
        large_sequence.serialize(&mut serializer).unwrap();

        // Assert
        let mut expected_output = String::new(); // Create an empty String
        for i in &large_sequence {
            // Append to the String directly
            writeln!(&mut expected_output, "- {}", i).unwrap();
        }

        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            expected_output,
            "Serialized large data doesn't match expected output"
        );
    }

    // Test cases for serializing nested sequences
    #[test]
    fn test_serialize_nested_sequences() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let nested_sequences = vec![
            vec!["a", "b", "c"],
            vec!["d", "e", "f"],
            vec!["g", "h", "i"],
        ];

        // Act
        nested_sequences.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "- - a\n  - b\n  - c\n- - d\n  - e\n  - f\n- - g\n  - h\n  - i\n",
            "Serialized nested sequences don't match expected output"
        );
    }

    // Test cases for serializing nested maps
    #[test]
    fn test_serialize_nested_maps() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let mut nested_maps = BTreeMap::new();
        let mut inner_map1 = BTreeMap::new();
        inner_map1.insert("key1", "value1");
        inner_map1.insert("key2", "value2");
        let mut inner_map2 = BTreeMap::new();
        inner_map2.insert("key3", "value3");
        inner_map2.insert("key4", "value4");
        nested_maps.insert("map1", inner_map1);
        nested_maps.insert("map2", inner_map2);

        // Act
        nested_maps.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "map1:\n  key1: value1\n  key2: value2\nmap2:\n  key3: value3\n  key4: value4\n",
            "Serialized nested maps don't match expected output"
        );
    }

    // Test cases for serializing mixed data types
    #[test]
    fn test_serialize_mixed_data_types() {
        // Arrange
        #[derive(Serialize)]
        struct MixedData {
            name: String,
            age: u32,
            active: bool,
            scores: Vec<i32>,
            metadata: BTreeMap<String, String>,
        }

        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        let mixed_data = MixedData {
            name: "Alice".to_string(),
            age: 30,
            active: true,
            scores: vec![80, 90, 95],
            metadata: {
                let mut map = BTreeMap::new();
                map.insert("key1".to_string(), "value1".to_string());
                map.insert("key2".to_string(), "value2".to_string());
                map
            },
        };
        mixed_data.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "name: Alice\nage: 30\nactive: true\nscores:\n- 80\n- 90\n- 95\nmetadata:\n  key1: value1\n  key2: value2\n",
            "Serialized mixed data types don't match expected output"
        );
    }

    // Test cases for serializing empty sequence and map
    #[test]
    fn test_serialize_empty_sequence_and_map() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let empty_sequence: Vec<i32> = Vec::new();
        let empty_map: BTreeMap<String, i32> = BTreeMap::new();

        // Act
        empty_sequence.serialize(&mut serializer).unwrap();
        empty_map.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "[]\n--- {}\n",
            "Serialized empty sequence and map don't match expected output"
        );
    }

    // Test cases for serializing special characters
    #[test]
    fn test_serialize_special_characters() {
        // Arrange
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        let special_string = "\"'\\n\t";

        // Act
        special_string.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "\"\\\"'\\\\n\\t\"\n",
            "Serialized special characters don't match expected output"
        );
    }

    // Test cases for serializing with custom serializer
    #[test]
    fn test_serialize_custom_serializer() {
        // Arrange
        use serde::ser::SerializeMap;

        #[derive(Serialize)]
        struct CustomStruct {
            #[serde(serialize_with = "custom_serialize")]
            value: String,
        }

        fn custom_serialize<S>(
            value: &String,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_entry(
                "custom_value",
                &format!("<<{}>>", value),
            )?;
            map.end()
        }

        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);

        // Act
        let custom_struct = CustomStruct {
            value: "example".to_string(),
        };
        custom_struct.serialize(&mut serializer).unwrap();

        // Assert
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "value:\n  custom_value: <<example>>\n",
            "Serialized custom serializer doesn't match expected output"
        );
    }
}
