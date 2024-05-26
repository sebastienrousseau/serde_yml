#[cfg(test)]
mod tests {
    use serde_yml::{
        macro_check_directory, macro_cleanup_directories,
        macro_create_directories,
    };
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_macro_check_directory() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("logs");

        // Test creating a new directory
        macro_check_directory!(&path, "logs");
        assert!(path.exists() && path.is_dir());

        // Test directory already exists
        macro_check_directory!(&path, "logs");
        assert!(path.exists() && path.is_dir());
    }

    #[test]
    fn test_macro_create_directories() {
        let temp_dir = tempdir().unwrap();
        let path1 = temp_dir.path().join("logs1");
        let path2 = temp_dir.path().join("logs2");

        // Test creating multiple directories
        macro_create_directories!(&path1, &path2).unwrap();
        assert!(path1.exists() && path1.is_dir());
        assert!(path2.exists() && path2.is_dir());
    }

    #[test]
    fn test_macro_cleanup_directories() {
        let temp_dir = tempdir().unwrap();
        let path1 = temp_dir.path().join("logs1");
        let path2 = temp_dir.path().join("logs2");

        // Create directories to clean up
        fs::create_dir_all(&path1).unwrap();
        fs::create_dir_all(&path2).unwrap();

        // Test cleaning up directories
        macro_cleanup_directories!(&path1, &path2);
        assert!(!path1.exists());
        assert!(!path2.exists());
    }
}
