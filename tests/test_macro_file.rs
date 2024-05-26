#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_yml::generate_file;
    use std::fs;

    #[derive(Serialize, Debug)]
    struct MyData {
        key: String,
        value: String,
        nested: NestedData,
        items: Vec<Item>,
    }

    #[derive(Serialize, Debug)]
    struct NestedData {
        id: u32,
        description: String,
    }

    #[derive(Serialize, Debug)]
    struct Item {
        name: String,
        quantity: u32,
        price: f64,
    }

    fn create_test_data() -> MyData {
        MyData {
            key: "example".to_string(),
            value: "Hello, World!".to_string(),
            nested: NestedData {
                id: 1,
                description: "This is a nested structure".to_string(),
            },
            items: vec![
                Item {
                    name: "Item1".to_string(),
                    quantity: 10,
                    price: 99.99,
                },
                Item {
                    name: "Item2".to_string(),
                    quantity: 5,
                    price: 9.99,
                },
            ],
        }
    }

    #[test]
    fn test_generate_yaml_file() {
        let value = create_test_data();
        generate_file!("yaml", &value, |content| {
            fs::write("test_output.yaml", content)
        });
        assert!(fs::metadata("test_output.yaml").is_ok());
        fs::remove_file("test_output.yaml").unwrap();
    }

    #[test]
    fn test_generate_json_file() {
        let value = create_test_data();
        generate_file!("json", &value, |content| {
            fs::write("test_output.json", content)
        });
        assert!(fs::metadata("test_output.json").is_ok());
        fs::remove_file("test_output.json").unwrap();
    }

    #[test]
    fn test_generate_txt_file() {
        let value = create_test_data();
        generate_file!("txt", &value, |content| {
            fs::write("test_output.txt", content)
        });
        assert!(fs::metadata("test_output.txt").is_ok());
        fs::remove_file("test_output.txt").unwrap();
    }

    #[test]
    fn test_unsupported_file_type() {
        let value = create_test_data();
        generate_file!("unsupported", &value, |_content| {
            Ok::<(), String>(())
        });
    }

    #[test]
    fn test_custom_serializer_failure() {
        #[derive(Debug)]
        struct NonSerializable;

        let value = NonSerializable;

        let custom_serializer =
            |_value: &NonSerializable| -> Result<String, String> {
                Err("Intentional serialization failure".to_string())
            };

        generate_file!(
            "json",
            &value,
            |_content| { Ok::<(), String>(()) },
            custom_serializer
        );
    }
}
