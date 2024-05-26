#[cfg(test)]
mod tests {
    use serde_yml::modules::path::Path;

    // Tests for Path::Root variant
    #[test]
    fn test_path_root() {
        let path = Path::Root;
        assert_eq!(format!("{}", path), ".");
    }

    // Tests for Path::Seq variant
    #[test]
    fn test_path_seq() {
        let root = Path::Root;
        let path = Path::Seq {
            parent: &root,
            index: 42,
        };
        assert_eq!(format!("{}", path), "\\[42\\]");
    }

    // Tests for Path::Map variant
    #[test]
    fn test_path_map() {
        let root = Path::Root;
        let path = Path::Map {
            parent: &root,
            key: "key",
        };
        assert_eq!(format!("{}", path), "key");
    }

    // Tests for Path::Alias variant
    #[test]
    fn test_path_alias() {
        let root = Path::Root;
        let path = Path::Alias { parent: &root };
        assert_eq!(format!("{}", path), "");
    }

    // Tests for Path::Unknown variant
    #[test]
    fn test_path_unknown() {
        let root = Path::Root;
        let path = Path::Unknown { parent: &root };
        assert_eq!(format!("{}", path), "?");
    }

    // Tests for nested paths
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
}
