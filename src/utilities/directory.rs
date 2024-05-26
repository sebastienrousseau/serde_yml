use std::io::Error as ioError;
use std::{
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
};
/// Ensures a directory exists, creating it if necessary.
///
/// This function takes a reference to a `Path` object for a directory and a
/// human-readable name for the directory, and creates the directory if it
/// does not already exist.
///
/// # Arguments
///
/// * `dir` - A reference to a `Path` object for the directory.
/// * `name` - A human-readable name for the directory, used in error messages.
///
/// # Returns
///
/// * `Result<(), String>` - A result indicating success or failure.
///     - `Ok(())` if the directory exists or was created successfully.
///     - `Err(String)` if the directory does not exist and could not be created.
///
/// # Example
///
/// ```
/// use serde_yml::utilities::directory::directory;
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let temp_dir = tempdir().unwrap();
/// let dir = temp_dir.path().join("logs");
/// directory(&dir, "logs").expect("Could not create logs directory");
/// ```
///
pub fn directory(dir: &Path, name: &str) -> Result<String, String> {
    if dir.exists() {
        if !dir.is_dir() {
            return Err(format!(
                "❌ Error: {} is not a directory.",
                name
            ));
        }
    } else {
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "❌ Error: Cannot create {} directory: {}",
                    name, e
                ))
            }
        }
    }
    Ok(String::new())
}

/// Moves the output directory to the public directory.
///
/// This function takes a reference to a `Path` object for the output directory
/// and a string for the site name, and moves the output directory to the
/// public directory.
///
/// # Arguments
///
/// * `site_name` - A string for the site name.
/// * `out_dir` - A reference to a `Path` object for the output directory.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - A result indicating success or failure.
///     - `Ok(())` if the output directory was moved successfully.
///     - `Err(std::io::Error)` if the output directory could not be moved.
///
/// # Example
///
/// ```
/// use serde_yml::utilities::directory::move_output_directory;
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let temp_dir = tempdir().unwrap();
/// let out_dir = temp_dir.path().join("output");
/// std::fs::create_dir(&out_dir).unwrap();
/// move_output_directory("example_site", &out_dir).unwrap();
/// ```
///
pub fn move_output_directory(
    site_name: &str,
    out_dir: &Path,
) -> std::io::Result<()> {
    println!("❯ Moving output directory...");

    let public_dir = Path::new("public");

    if public_dir.exists() {
        fs::remove_dir_all(public_dir)?;
    }

    fs::create_dir(public_dir)?;

    let site_name = site_name.replace(' ', "_");
    let new_project_dir = public_dir.join(site_name);
    fs::create_dir_all(&new_project_dir)?;

    fs::rename(out_dir, &new_project_dir)?;

    println!("  Done.\n");

    Ok(())
}

/// Cleans up the directory at the given path.
///
/// If the directory does not exist, this function does nothing.
///
/// # Arguments
///
/// * `directories` - An array of references to `Path` objects representing the
///    directories to be cleaned up.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or failure.
///     - `Ok(())` if the directories were cleaned up successfully.
///     - `Err(Box<dyn Error>)` if an error occurred during the cleanup process.
///
/// # Example
///
/// ```
/// use serde_yml::utilities::directory::cleanup_directory;
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let temp_dir = tempdir().unwrap();
/// let dir1 = temp_dir.path().join("dir1");
/// let dir2 = temp_dir.path().join("dir2");
/// std::fs::create_dir_all(&dir1).unwrap();
/// std::fs::create_dir_all(&dir2).unwrap();
/// cleanup_directory(&[&dir1, &dir2]).unwrap();
/// ```
///
pub fn cleanup_directory(directories: &[&Path]) -> Result<(), ioError> {
    for dir in directories {
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        } else {
            // Log a warning if the directory does not exist.
            log::warn!(
                "Directory '{}' does not exist, skipping cleanup.",
                dir.display()
            );
        }
    }
    Ok(())
}

/// Creates a new directory at the given path.
///
/// If the directory already exists, this function does nothing.
///
/// # Arguments
///
/// * `directories` - An array of references to `Path` objects representing the
///    directories to be created.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or failure.
///     - `Ok(())` if the directories were created successfully.
///     - `Err(Box<dyn Error>)` if an error occurred during the creation process.
///
/// # Example
///
/// ```
/// use serde_yml::utilities::directory::create_directory;
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let temp_dir = tempdir().unwrap();
/// let dir1 = temp_dir.path().join("dir1");
/// let dir2 = temp_dir.path().join("dir2");
/// create_directory(&[&dir1, &dir2]).unwrap();
/// ```
///
pub fn create_directory(
    directories: &[&Path],
) -> Result<(), Box<dyn Error>> {
    for directory in directories {
        if directory.exists() {
            continue;
        }

        fs::create_dir(directory)?;
    }

    Ok(())
}

/// Truncates a path to only have a set number of path components.
///
/// Will truncate a path to only show the last `length` components in a path.
/// If a length of `0` is provided, the path will not be truncated.
/// A value will only be returned if the path has been truncated.
///
/// # Arguments
///
/// * `path` - The path to truncate.
/// * `length` - The number of path components to keep.
///
/// # Returns
///
/// * An `Option` of the truncated path as a string. If the path was not truncated, `None` is returned.
///
/// # Example
///
/// ```
/// use serde_yml::utilities::directory::truncate;
/// use std::path::Path;
///
/// let path = Path::new("/foo/bar/baz");
/// assert_eq!(truncate(path, 2), Some("baz/baz".to_string()));
/// assert_eq!(truncate(path, 0), None);
/// ```
///
pub fn truncate(path: &Path, length: usize) -> Option<String> {
    // Checks if the length is 0. If it is, returns `None`.
    if length == 0 {
        return None;
    }

    // Creates a new PathBuf object to store the truncated path.
    let mut truncated = PathBuf::new();

    // Iterates over the components of the path in reverse order.
    let mut count = 0;
    while let Some(component) = path.components().next_back() {
        // Adds the component to the truncated path.
        truncated.push(component);
        count += 1;

        // If the count reaches the desired length, breaks out of the loop.
        if count == length {
            break;
        }
    }

    // If the count is equal to the desired length, returns the truncated path as a string.
    if count == length {
        Some(truncated.to_string_lossy().to_string())
    } else {
        // Otherwise, returns `None`.
        None
    }
}
