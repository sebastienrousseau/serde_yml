#[macro_export]
/// A macro that generates a file based on a provided value, a generator function, and an optional custom serializer.
///
/// The macro takes four parameters: `$file_type:expr`, `$value:expr`, `$generator:expr`, and an optional `$serializer:expr`.
///
/// - `$file_type:expr`: A string literal representing the type of the file to be generated (e.g., "yaml", "json", "txt").
/// - `$value:expr`: A reference to a value.
/// - `$generator:expr`: A closure that takes a string slice (the serialized content) and generates the file.
/// - `$serializer:expr`: An optional custom serializer function that takes a reference to the value and returns a `Result<String, String>`.
///
/// The macro attempts to generate the file using the provided `$generator` function. If an error occurs during the generation process, it prints an error message to the standard error stream.
///
/// # Examples
///
/// ## YAML Example
/// ```rust
/// use std::fs;
/// use serde::Serialize;
/// use serde_yml::to_string as to_yaml_string;
/// use serde_yml::generate_file;
///
/// #[derive(Serialize, Debug)]
/// struct MyData {
///     key: String,
///     value: String,
///     nested: NestedData,
///     items: Vec<Item>,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct NestedData {
///     id: u32,
///     description: String,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct Item {
///     name: String,
///     quantity: u32,
///     price: f64,
/// }
///
/// let value = MyData {
///     key: "example".to_string(),
///     value: "Hello, Serde YML!".to_string(),
///     nested: NestedData {
///         id: 1,
///         description: "This is a nested structure".to_string(),
///     },
///     items: vec![
///         Item {
///             name: "Item1".to_string(),
///             quantity: 10,
///             price: 99.99,
///         },
///         Item {
///             name: "Item2".to_string(),
///             quantity: 5,
///             price: 9.99,
///         },
///     ],
/// };
///
/// generate_file!("yaml", &value, |content| {
///     fs::write("output.yaml", content)
/// });
/// fs::remove_file("output.yaml").unwrap();
/// ```
///
/// ## JSON Example
/// ```rust
/// use std::fs;
/// use serde::Serialize;
/// use serde_json::to_string as to_json_string;
/// use serde_yml::generate_file;
///
/// #[derive(Serialize, Debug)]
/// struct MyData {
///     key: String,
///     value: String,
///     nested: NestedData,
///     items: Vec<Item>,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct NestedData {
///     id: u32,
///     description: String,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct Item {
///     name: String,
///     quantity: u32,
///     price: f64,
/// }
///
/// let value = MyData {
///     key: "example".to_string(),
///     value: "Hello, Serde JSON!".to_string(),
///     nested: NestedData {
///         id: 1,
///         description: "This is a nested structure".to_string(),
///     },
///     items: vec![
///         Item {
///             name: "Item1".to_string(),
///             quantity: 10,
///             price: 99.99,
///         },
///         Item {
///             name: "Item2".to_string(),
///             quantity: 5,
///             price: 9.99,
///         },
///     ],
/// };
///
/// generate_file!("json", &value, |content| {
///     fs::write("output.json", content)
/// });
/// fs::remove_file("output.json").unwrap();
/// ```
///
/// ## TXT Example
/// ```rust
/// use std::fs;
/// use serde::Serialize;
/// use serde_yml::generate_file;
///
/// #[derive(Serialize, Debug)]
/// struct MyData {
///     key: String,
///     value: String,
///     nested: NestedData,
///     items: Vec<Item>,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct NestedData {
///     id: u32,
///     description: String,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct Item {
///     name: String,
///     quantity: u32,
///     price: f64,
/// }
///
/// let value = MyData {
///     key: "example".to_string(),
///     value: "Hello, Serde TXT!".to_string(),
///     nested: NestedData {
///         id: 1,
///         description: "This is a nested structure".to_string(),
///     },
///     items: vec![
///         Item {
///             name: "Item1".to_string(),
///             quantity: 10,
///             price: 99.99,
///         },
///         Item {
///             name: "Item2".to_string(),
///             quantity: 5,
///             price: 9.99,
///         },
///     ],
/// };
///
/// generate_file!("txt", &value, |content| {
///     let txt_string = format!("{:?}", content);
///     fs::write("output.txt", txt_string)
/// });
/// fs::remove_file("output.txt").unwrap();
/// ```
///
macro_rules! generate_file {
    ($file_type:expr, $value:expr, $generator:expr, $serializer:expr) => {
        let result = $serializer($value);
        if let Ok(content) = result {
            if let Err(err) = $generator(&content) {
                eprintln!(
                    "Error generating {} file: {}",
                    $file_type, err
                );
            }
        } else {
            eprintln!(
                "Error serializing value to {}: {}",
                $file_type,
                result.unwrap_err()
            );
        }
    };
    ($file_type:expr, $value:expr, $generator:expr) => {
        generate_file!($file_type, $value, $generator, |value| {
            match $file_type {
                "yaml" => serde_yml::to_string(value)
                    .map_err(|e| e.to_string()),
                "json" => serde_json::to_string(value)
                    .map_err(|e| e.to_string()),
                "txt" => Ok(format!("{:?}", value)),
                _ => Err("Unsupported file type".to_string()),
            }
        });
    };
}
