#![allow(missing_docs)]
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_yml::{
        from_reader, from_slice, from_str, from_value, to_string,
        to_value, Mapping, Number, Result, Sequence, Value, VERSION,
    };
    use std::io::Cursor;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Point {
        x: f64,
        y: f64,
    }

    /// Verifies that the `VERSION` string is not empty.
    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_version() {
        assert!(!VERSION.is_empty(), "VERSION should not be empty");
    }

    /// Ensures that the `VERSION` string appears to follow a semver-like format with a dot.
    #[test]
    fn test_version_format() {
        assert!(
            VERSION.contains('.'),
            "VERSION should use a dotted semver format"
        );
        assert!(
            VERSION.split('.').count() >= 2,
            "VERSION should have at least major.minor components"
        );
    }

    /// Verifies that the `VERSION` string matches the crate version in Cargo.toml.
    #[test]
    fn test_version_consistency() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }

    /// Test serialization of a Point struct to YAML string
    #[test]
    fn test_serialization() -> Result<()> {
        let point = Point { x: 1.0, y: 2.0 };
        let yaml = to_string(&point)?;
        assert_eq!(yaml, "x: 1.0\n'y': 2.0\n");
        Ok(())
    }

    /// Test deserialization of a YAML string to a Point struct
    #[test]
    fn test_deserialization() -> Result<()> {
        let yaml = "x: 1.0\ny: 2.0\n";
        let point: Point = from_str(yaml)?;
        assert_eq!(point, Point { x: 1.0, y: 2.0 });
        Ok(())
    }

    /// Test deserialization from a reader (Cursor in this case)
    #[test]
    fn test_from_reader() -> Result<()> {
        let yaml = "x: 1.0\ny: 2.0\n";
        let mut cursor = Cursor::new(yaml);
        let point: Point = from_reader(&mut cursor)?;
        assert_eq!(point, Point { x: 1.0, y: 2.0 });
        Ok(())
    }

    /// Test deserialization from a byte slice
    #[test]
    fn test_from_slice() -> Result<()> {
        let yaml = b"x: 1.0\ny: 2.0\n";
        let point: Point = from_slice(yaml)?;
        assert_eq!(point, Point { x: 1.0, y: 2.0 });
        Ok(())
    }

    /// Test Mapping functionality
    #[test]
    fn test_mapping() {
        let mut map = Mapping::new();
        map.insert(
            Value::String("key".to_string()),
            Value::Number(Number::from(42)),
        );
        assert_eq!(map.get("key").and_then(Value::as_i64), Some(42));
    }

    /// Test Sequence functionality
    #[test]
    fn test_sequence() {
        let seq = Sequence::from(vec![
            Value::Number(Number::from(1)),
            Value::Number(Number::from(2)),
        ]);
        assert_eq!(seq.len(), 2);
        assert_eq!(seq[0].as_i64(), Some(1));
    }

    /// Test Value to and from conversion
    #[test]
    fn test_value_conversion() {
        let point = Point { x: 1.0, y: 2.0 };
        let value =
            to_value(&point).expect("Failed to convert to Value");
        let deserialized: Point =
            from_value(value).expect("Failed to convert from Value");
        assert_eq!(point, deserialized);
    }

    /// Test deserialization error handling
    #[test]
    fn test_deserialization_error() {
        let yaml = "invalid_yaml: [unterminated_sequence";
        let result: Result<Point> = from_str(yaml);

        assert!(result.is_err(), "Expected deserialization to fail");

        if let Err(err) = result {
            // Print the error message to understand its structure
            eprintln!("Deserialization error: {:?}", err);
        }
    }

    /// Test custom Mapping insertion behavior
    #[test]
    fn test_custom_mapping_insertion() {
        let mut map = Mapping::new();
        map.insert(
            Value::String("nested".to_string()),
            Value::Mapping(Mapping::new()),
        );
        assert!(map
            .get("nested")
            .and_then(Value::as_mapping)
            .is_some());
    }

    /// Test handling of empty Sequence
    #[test]
    fn test_empty_sequence() {
        let seq = Sequence::new();
        assert!(seq.is_empty());
    }

    /// Test handling of empty Mapping
    #[test]
    fn test_empty_mapping() {
        let map = Mapping::new();
        assert!(map.is_empty());
    }

    /// Test handling of nested serialization
    #[test]
    fn test_nested_serialization() -> Result<()> {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Nested {
            outer: Point,
            inner: Option<Point>,
        }
        let nested = Nested {
            outer: Point { x: 1.0, y: 2.0 },
            inner: Some(Point { x: 3.0, y: 4.0 }),
        };
        let yaml = to_string(&nested)?;
        let deserialized: Nested = from_str(&yaml)?;
        assert_eq!(nested, deserialized);
        Ok(())
    }

    /// Test handling of null values
    #[test]
    fn test_null_values() -> Result<()> {
        let yaml = "key: null\n";
        let value: Mapping = from_str(yaml)?;
        assert!(value.get("key").map(Value::is_null).unwrap_or(false));
        Ok(())
    }

    /// Test handling of invalid data type mismatch
    #[test]
    fn test_invalid_data_type_mismatch() {
        let yaml = "key: [1, 2, 3]\n";
        let result: Result<Point> = from_str(yaml); // Attempt to deserialize into `Point`

        assert!(
            result.is_err(),
            "Expected deserialization to fail due to type mismatch"
        );
    }

    /// Test handling of large numbers
    #[test]
    fn test_large_number() -> Result<()> {
        let yaml = "big: 9223372036854775807\n";
        let value: Mapping = from_str(yaml)?;
        assert_eq!(
            value.get("big").and_then(Value::as_i64),
            Some(9223372036854775807)
        );
        Ok(())
    }

    /// Test handling of float precision
    #[test]
    #[allow(clippy::approx_constant)]
    fn test_float_precision() -> Result<()> {
        let yaml = "float: 3.141592653589793\n";
        let value: Mapping = from_str(yaml)?;
        assert_eq!(
            value.get("float").and_then(Value::as_f64),
            Some(3.141592653589793)
        );
        Ok(())
    }
}
