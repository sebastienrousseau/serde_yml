#[cfg(test)]
mod tests {
    use serde_yml::modules::path::Path;

    /// Test the Path::Root variant.
    ///
    /// This test ensures that the root path is correctly formatted as ".".
    #[test]
    fn test_path_root() {
        let path = Path::Root;
        assert_eq!(format!("{}", path), ".");
    }

    /// Test the Path::Seq variant.
    ///
    /// This test checks that a sequence path with a given index is correctly formatted.
    #[test]
    fn test_path_seq() {
        let root = Path::Root;
        let path = Path::Seq {
            parent: &root,
            index: 42,
        };
        assert_eq!(format!("{}", path), "\\[42\\]");
    }

    /// Test the Path::Map variant.
    ///
    /// This test checks that a map path with a given key is correctly formatted.
    #[test]
    fn test_path_map() {
        let root = Path::Root;
        let path = Path::Map {
            parent: &root,
            key: "key",
        };
        assert_eq!(format!("{}", path), "key");
    }

    /// Test the Path::Alias variant.
    ///
    /// This test checks that an alias path is correctly formatted.
    #[test]
    fn test_path_alias() {
        let root = Path::Root;
        let path = Path::Alias { parent: &root };
        assert_eq!(format!("{}", path), "");
    }

    /// Test the Path::Unknown variant.
    ///
    /// This test checks that an unknown path is correctly formatted.
    #[test]
    fn test_path_unknown() {
        let root = Path::Root;
        let path = Path::Unknown { parent: &root };
        assert_eq!(format!("{}", path), "?");
    }

    /// Test nested paths.
    ///
    /// This test ensures that nested paths with various combinations of variants are correctly formatted.
    #[test]
    fn test_path_nested() {
        let root = Path::Root;
        let seq = Path::Seq {
            parent: &root,
            index: 0,
        };
        let map = Path::Map {
            parent: &seq,
            key: "key",
        };
        let alias = Path::Alias { parent: &map };
        let unknown = Path::Unknown { parent: &alias };
        assert_eq!(format!("{}", unknown), "\\[0\\].key..?");
    }

    /// Test deeply nested paths.
    ///
    /// This test checks the formatting of a deeply nested path with multiple levels of sequences and maps.
    #[test]
    fn test_deeply_nested_path() {
        let root = Path::Root;
        let seq1 = Path::Seq {
            parent: &root,
            index: 1,
        };
        let map1 = Path::Map {
            parent: &seq1,
            key: "first",
        };
        let seq2 = Path::Seq {
            parent: &map1,
            index: 2,
        };
        let map2 = Path::Map {
            parent: &seq2,
            key: "second",
        };
        let alias = Path::Alias { parent: &map2 };
        let unknown = Path::Unknown { parent: &alias };
        assert_eq!(
            format!("{}", unknown),
            "\\[1\\].first.\\[2\\].second..?"
        );
    }

    /// Test empty key in Path::Map.
    ///
    /// This test ensures that a map path with an empty key is correctly handled and formatted.
    #[test]
    fn test_path_map_empty_key() {
        let root = Path::Root;
        let path = Path::Map {
            parent: &root,
            key: "",
        };
        assert_eq!(format!("{}", path), "");
    }
}
