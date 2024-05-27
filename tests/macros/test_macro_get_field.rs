/// Tests for the `macro_get_field` macro.
mod tests {
    use serde_yml::macro_get_field;
    use std::env;
    use std::error::Error;
    use std::path::Path;

    /// Test reading a JSON file and retrieving a field value.
    #[test]
    fn test_macro_get_field_success() -> Result<(), Box<dyn Error>> {
        // Define the generated function name.
        macro_get_field!(get_field, serde_json::from_reader);

        // Define the path to the JSON file.
        let file_path = Some("tests/data/test.json");

        // Define the field names to retrieve.
        let field_name = "name";
        let field_age = "age";
        let field_city = "city";

        // Retrieve the field values.
        let field_value_name = get_field(file_path, field_name)?;
        let field_value_age = get_field(file_path, field_age)?;
        let field_value_city = get_field(file_path, field_city)?;

        // Check if the field values are correct.
        assert_eq!(field_value_name, "John Doe");
        assert_eq!(field_value_age, "30");
        assert_eq!(field_value_city, "New York");

        Ok(())
    }

    /// Test retrieving a non-existent field from a JSON file.
    #[test]
    fn test_macro_get_field_non_existent_field(
    ) -> Result<(), Box<dyn Error>> {
        // Define the generated function name.
        macro_get_field!(get_field, serde_json::from_reader);

        // Define the path to the JSON file.
        let file_path = Some("tests/data/test.json");

        // Define a non-existent field name.
        let field_name = "non_existent_field";

        // Attempt to retrieve the non-existent field value.
        let result = get_field(file_path, field_name);

        // Check if the expected error is returned.
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Field '{}' not found", field_name)
        );

        Ok(())
    }

    /// Test retrieving a field value from a non-existent JSON file.
    #[test]
    fn test_macro_get_field_non_existent_file(
    ) -> Result<(), Box<dyn Error>> {
        // Define the generated function name.
        macro_get_field!(get_field, serde_json::from_reader);

        // Define the path to a non-existent JSON file.
        let file_path = Some("tests/data/non_existent.json");

        // Define a field name to retrieve.
        let field_name = "name";

        // Attempt to retrieve the field value from the non-existent file.
        let result = get_field(file_path, field_name);

        // Check if the expected error is returned.
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No such file or directory"));

        Ok(())
    }
}
