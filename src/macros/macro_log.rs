//! This module contains macros related to logging messages at various log levels and formats.
//!
//! It includes a custom logging macro, `macro_log_info`, which allows logging messages with
//! specified log levels, components, descriptions, and formats.
//!
//! # Custom Logging Macro
//!
//! The `macro_log_info` macro is designed for logging messages with customizable log levels,
//! components, descriptions, and formats. It provides flexibility in defining log messages
//! according to specific requirements.
//!
//! # Parameters
//!
//! - `$level`: The log level of the message.
//! - `$component`: The component where the log is coming from.
//! - `$description`: A description of the log message.
//! - `$format`: The format of the log message.
//!

/// Custom logging macro for various log levels and formats.
///
/// # Parameters
///
/// * `$level`: The log level of the message.
/// * `$component`: The component where the log is coming from.
/// * `$description`: A description of the log message.
/// * `$format`: The format of the log message.
///
#[macro_export]
macro_rules! macro_log_info {
    ($level:expr, $component:expr, $description:expr, $format:expr) => {{
        // Get the current date and time in ISO 8601 format.
        let date = DateTime::new(); // Create a new DateTime instance
        let iso = date.iso_8601; // Get ISO 8601 formatted date and time

        // Create a new random number generator
        let mut rng = Random::default(); // Default random number generator
        let session_id = rng.rand().to_string(); // Generate session ID

        // Create a new log instance
        let log = Log::new(
            &session_id, // Session ID
            &iso, // ISO 8601 formatted date and time
            $level, // Log level
            $component, // Component name
            $description, // Log description
            $format, // Log format
        );
        log // Return the Log instance
    }};
}
