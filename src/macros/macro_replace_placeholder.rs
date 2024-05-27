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
///
/// # Examples
///
/// ```
/// use serde_yml::macro_replace_placeholder;
//
/// #[derive(Default)]
/// struct Params {
///     field1: Option<&'static str>,
///     field2: Option<&'static str>,
/// }
//
///     let params = Params {
///         field1: Some("value1"),
///         field2: Some("value2"),
///     };
//
///     // Test replacing both fields
///     let line = macro_replace_placeholder!(
///         "Field 1: {field1}, Field 2: {field2}",
///         &params,
///         field1,
///         field2
///     );
///     assert_eq!(line, "Field 1: value1, Field 2: value2");
/// ```
///
#[macro_export]
macro_rules! macro_replace_placeholder {
    ($line:expr, $params:expr, $($field:ident),+) => {
        {
            let mut line = $line.to_owned(); // Convert line to owned String
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
