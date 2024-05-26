//! This module provides macros for directory operations, including checking directory existence,
//! creating multiple directories at once, and cleaning up directories.
//!

/// # `macro_check_directory` Macro
///
/// Check if a directory exists and create it if necessary.
///
/// ## Usage
///
/// ```rust
/// use serde_yml::macro_check_directory;
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let temp_dir = tempdir().unwrap();
/// let path = temp_dir.path().join("logs");
/// macro_check_directory!(&path, "logs");
/// ```
///
/// ## Arguments
///
/// * `_dir` - The path to check, as a `std::path::Path`.
/// * `_name` - A string literal representing the directory name. This is used in error messages.
///
/// ## Behaviour
///
/// The `macro_check_directory` macro checks if the directory specified by `_dir` exists. If it exists and is not a directory, a panic with an error message is triggered. If the directory doesn't exist, the macro attempts to create it using `std::fs::create_dir_all(_dir)`. If the creation is successful, no action is taken. If an error occurs during the directory creation, a panic is triggered with an error message indicating the failure.
///
/// Please note that the macro panics on failure. Consider using this macro in scenarios where panicking is an acceptable behaviour, such as during application startup or setup.
///
#[macro_export]
macro_rules! macro_check_directory {
    ($_dir:expr, $_name:expr) => {{
        let directory: &std::path::Path = $_dir;
        let name = $_name;
        if !directory.exists() {
            match std::fs::create_dir_all(directory) {
                Ok(_) => {}
                Err(e) => {
                    log::error!(
                        "❌ Cannot create '{}' directory: {}",
                        name,
                        e
                    );
                    panic!(
                        "❌ Cannot create '{}' directory: {}",
                        name, e
                    );
                }
            }
        } else if !directory.is_dir() {
            panic!("❌ '{}' exists but is not a directory", name);
        }
    }};
}

/// # `macro_cleanup_directories` Macro
///
/// Cleanup multiple directories by invoking the `cleanup_directory` function.
///
/// ## Usage
///
/// ```rust
/// use serde_yml::macro_cleanup_directories;
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let temp_dir = tempdir().unwrap();
/// let path1 = temp_dir.path().join("logs");
/// let path2 = temp_dir.path().join("logs2");
/// macro_cleanup_directories!(&path1, &path2);
/// ```
///
/// ## Arguments
///
/// * `$( $_dir:expr ),*` - A comma-separated list of directory paths to clean up.
///
/// ## Behaviour
///
/// The `macro_cleanup_directories` macro takes multiple directory paths as arguments and invokes the `cleanup_directory` function for each path. It is assumed that the `cleanup_directory` function is available in the crate's utilities module (`$crate::utilities::cleanup_directory`).
///
/// The macro creates an array `directories` containing the provided directory paths and passes it as an argument to `cleanup_directory`. The `cleanup_directory` function is responsible for performing the cleanup operations.
///
/// Please note that the macro uses the `?` operator for error propagation. It expects the `cleanup_directory` function to return a `Result` type. If an error occurs during the cleanup process, it will be propagated up the call stack, allowing the caller to handle it appropriately.
///
#[macro_export]
macro_rules! macro_cleanup_directories {
    ( $( $_dir:expr ),* ) => {
        {
            use $crate::utilities::directory::cleanup_directory;
            let directories: &[&Path] = &[ $( $_dir ),* ];
            match cleanup_directory(directories) {
                Ok(()) => (),
                Err(err) => {
                    log::error!("Cleanup failed: {:?}", err);
                    panic!("Cleanup failed: {:?}", err);
                },
            }
        }
    };
}

/// # `macro_create_directories` Macro
///
/// Create multiple directories at once.
///
/// ## Usage
///
/// ```rust
/// use serde_yml::macro_create_directories;
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let temp_dir = tempdir().unwrap();
/// let path1 = temp_dir.path().join("logs1");
/// let path2 = temp_dir.path().join("logs2");
///  macro_create_directories!(&path1, &path2).unwrap();
/// ```
///
/// ## Arguments
///
/// * `...` - Variable number of directory paths, each specified as an expression (`expr`).
///
/// ## Behaviour
///
/// The `macro_create_directories` macro creates multiple directories at once. It takes a variable number of directory paths as arguments and uses the `create_directory` utility function from the `$crate` crate to create the directories.
///
/// The directories are specified as expressions and separated by commas.
///
/// The macro internally creates a slice of the directory paths and passes it to the `create_directory` function. If any error occurs during the directory creation, the macro returns an `Err` value, indicating the first encountered error. Otherwise, it returns `Ok(())`.
///
#[macro_export]
macro_rules! macro_create_directories {
    ( $( $_dir:expr ),* ) => {{
        use $crate::utilities::directory::create_directory;
        use std::path::Path;
        let directories: Vec<&Path> = vec![ $( Path::new($_dir) ),* ];
        match create_directory(&directories) {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("Directory creation failed: {:?}", err);
                Err(err)
            },
        }
    }};
}
