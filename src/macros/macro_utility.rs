/// Replaces placeholders in a given line with corresponding values from the provided parameters.
///
/// # Arguments
///
/// * `line` - The line containing placeholders to be replaced.
/// * `params` - The parameters containing values to replace the placeholders.
/// * `$($field:ident),+` - Identifiers representing the fields in `params` to be replaced.
///
/// # Returns
///
/// The line with placeholders replaced by their corresponding values.
///
#[macro_export]
macro_rules! macro_replace_placeholder {
    ($line:expr, $params:expr, $($field:ident),+) => {
        {
            let mut line = $line;
            $(
                line = line.replace(
                    concat!("{", stringify!($field), "}"),
                    &$params.$field.as_deref().unwrap_or(""),
                );
            )+
            line
        }
    };
}

/// Macro to generate a function that retrieves a field value from a JSON file.
///
/// # Arguments
///
/// * `$func_name` - The name of the generated function.
/// * `$deserializer` - The deserializer used to parse the JSON file.
///
/// # Returns
///
/// The generated function returns a `Result` containing the field value as a `String`,
/// or a `Box<dyn std::error::Error>` if an error occurs.
///
#[macro_export]
macro_rules! macro_get_field {
    ($func_name:ident, $deserializer:expr) => {
        /// Reads a file and deserializes its content using the specified
        /// deserializer function.
        pub fn $func_name(
            // The path of the JSON file to read.
            file_path: Option<&str>,
            // The name of the field to retrieve.
            field_name: &str,
        ) -> Result<String, Box<dyn std::error::Error>> {
            file_path.map_or_else(
                || Ok(String::new()),
                |file_path| {
                    let current_dir = env::current_dir()?;
                    let file_path =
                        Path::new(&current_dir).join(file_path);
                    read_file(&file_path, |file| {
                        let value: serde_json::Value =
                            $deserializer(file)?;
                        let field_value = value
                            .get(field_name)
                            .ok_or_else(|| {
                                format!(
                                    "Field '{}' not found",
                                    field_name
                                )
                            })?
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                value[field_name].to_string()
                            });
                        Ok(field_value)
                    })
                },
            )
        }
    };
}
