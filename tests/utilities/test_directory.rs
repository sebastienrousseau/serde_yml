#[cfg(test)]
mod tests {
    use serde_yml::utilities::directory::{
        cleanup_directory, create_directory, directory,
        move_output_directory, truncate,
    };
    use std::{fs, io::Error, path::Path};
    use tempfile::tempdir;

    /// Tests that the `directory` function correctly creates a directory if it does not exist.
    #[test]
    fn test_directory_not_exists() {
        let temp_dir = tempdir().unwrap();
        let dir = temp_dir.path().join("test_dir");
        assert_eq!(directory(&dir, "test_dir"), Ok(()));
        assert!(dir.exists());
    }

    /// Tests that the `directory` function correctly handles the case where the directory already exists.
    #[test]
    fn test_directory_exists() {
        let temp_dir = tempdir().unwrap();
        let dir = temp_dir.path().join("test_dir");
        fs::create_dir(&dir).unwrap();
        assert_eq!(directory(&dir, "test_dir"), Ok(()));
    }

    /// Tests that the `directory` function correctly handles the case where the path is not a directory.
    #[test]
    fn test_directory_not_dir() {
        let temp_dir = tempdir().unwrap();
        let dir = temp_dir.path().join("test_file");
        fs::write(&dir, "test").unwrap();
        assert!(directory(&dir, "test_file").is_err());
    }

    /// Tests that the `move_output_directory` function correctly moves the output directory to the public directory.
    #[test]
    fn test_move_output_directory() {
        let temp_dir = tempdir().unwrap();
        let out_dir = temp_dir.path().join("output");
        fs::create_dir(&out_dir).unwrap();
        let file_path = out_dir.join("test.txt");
        fs::write(&file_path, "test").unwrap();
        assert!(move_output_directory("test_site", &out_dir).is_ok());
        assert!(Path::new("public/test_site/test.txt").exists());
    }

    /// Tests that the `cleanup_directory` function correctly removes the directories.
    #[test]
    fn test_cleanup_directory() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");
        fs::create_dir_all(&dir1).unwrap();
        fs::create_dir_all(&dir2).unwrap();
        assert!(cleanup_directory(&[&dir1, &dir2]).is_ok());
        assert!(!dir1.exists());
        assert!(!dir2.exists());
    }

    /// Tests that the `cleanup_directory` function does nothing if the directories do not exist.
    #[test]
    fn test_cleanup_directory_not_exists() {
        let temp_dir = tempdir().unwrap();
        let dir = temp_dir.path().join("dir");
        assert!(cleanup_directory(&[&dir]).is_ok());
    }

    /// Tests that the `create_directory` function correctly creates the directories.
    #[test]
    fn test_create_directory() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");
        assert!(create_directory(&[&dir1, &dir2]).is_ok());
        assert!(dir1.exists());
        assert!(dir2.exists());
    }

    /// Tests that the `create_directory` function does nothing if the directories already exist.
    #[test]
    fn test_create_directory_exists() {
        let temp_dir = tempdir().unwrap();
        let dir = temp_dir.path().join("dir");
        fs::create_dir(&dir).unwrap();
        assert!(create_directory(&[&dir]).is_ok());
    }

    /// Tests the `truncate` function with different path lengths.
    #[test]
    fn test_truncate_path() {
        let path = Path::new("/a/b/c/d/e");

        let result = truncate(&path, 3);
        assert_eq!(result, Some("c/d/e".to_string()));

        let result = truncate(&path, 0);
        assert_eq!(result, None);

        let result = truncate(&path, 10);
        assert_eq!(result, None);
    }
}
