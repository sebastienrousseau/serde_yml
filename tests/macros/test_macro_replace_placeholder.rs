mod tests {

    use serde_yml::macro_replace_placeholder;

    #[derive(Default)]
    struct Params {
        field1: Option<&'static str>,
        field2: Option<&'static str>,
    }

    #[test]
    fn test_macro_replace_placeholder() {
        let params = Params {
            field1: Some("value1"),
            field2: Some("value2"),
        };

        // Test replacing both fields
        let line = macro_replace_placeholder!(
            "Field 1: {field1}, Field 2: {field2}",
            &params,
            field1,
            field2
        );
        assert_eq!(line, "Field 1: value1, Field 2: value2");
    }

    #[test]
    fn test_macro_replace_placeholder_no_fields() {
        let params = Params {
            field1: None,
            field2: None,
        };

        // Test replacing both fields
        let line = macro_replace_placeholder!(
            "Field 1: {field1}, Field 2: {field2}",
            &params,
            field1,
            field2
        );
        assert_eq!(line, "Field 1: , Field 2: ");
    }
    #[test]
    fn test_macro_replace_placeholder_single_field() {
        let params = Params {
            field1: Some("value1"),
            field2: Some("value2"),
        };

        // Test replacing only one field
        let line = macro_replace_placeholder!(
            "Field 1: {field1}, Field 2: {field2}",
            &params,
            field1
        );
        assert_eq!(line, "Field 1: value1, Field 2: {field2}");
    }

    #[test]
    fn test_macro_replace_placeholder_empty_line() {
        let params = Params {
            field1: Some("value1"),
            field2: Some("value2"),
        };

        // Test replacing placeholders in an empty line
        let line =
            macro_replace_placeholder!("", &params, field1, field2);
        assert_eq!(line, "");
    }
}
