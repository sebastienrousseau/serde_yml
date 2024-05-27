#[cfg(test)]
mod tests {
    use serde_yml::{
        macro_check_directory, macro_cleanup_directories,
        macro_create_directories,
    };
    use std::{fs, path::Path};
    use tempfile::tempdir;

    /// Tests the `macro_check_directory` macro when the directory doesn't exist.
    #[test]
    fn test_macro_check_directory_create() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("logs");

        macro_check_directory!(&path, "logs");
        assert!(path.exists() && path.is_dir());
    }

    /// Tests the `macro_check_directory` macro when the directory already exists.
    #[test]
    fn test_macro_check_directory_exists() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("logs");

        fs::create_dir(&path).unwrap();
        macro_check_directory!(&path, "logs");
        assert!(path.exists() && path.is_dir());
    }

    /// Tests the `macro_check_directory` macro when the path is not a directory.
    #[test]
    #[should_panic(
        expected = "❌ 'test_file' exists but is not a directory"
    )]
    fn test_macro_check_directory_not_dir() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("test_file");
        fs::write(&path, "test").unwrap();

        macro_check_directory!(&path, "test_file");
    }

    /// Tests the `macro_create_directories` macro when creating multiple directories.
    #[test]
    fn test_macro_create_directories() {
        let temp_dir = tempdir().unwrap();
        let path1 = temp_dir.path().join("logs1");
        let path2 = temp_dir.path().join("logs2");

        macro_create_directories!(&path1, &path2).unwrap();
        assert!(path1.exists() && path1.is_dir());
        assert!(path2.exists() && path2.is_dir());
    }

    /// Tests the `macro_cleanup_directories` macro when cleaning up multiple directories.
    #[test]
    fn test_macro_cleanup_directories() {
        let temp_dir = tempdir().unwrap();
        let path1 = temp_dir.path().join("logs1");
        let path2 = temp_dir.path().join("logs2");

        fs::create_dir_all(&path1).unwrap();
        fs::create_dir_all(&path2).unwrap();

        macro_cleanup_directories!(&path1, &path2);
        assert!(!path1.exists());
        assert!(!path2.exists());
    }
}
