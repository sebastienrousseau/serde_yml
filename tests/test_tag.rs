#[cfg(test)]
mod tests {
    use serde_yml::libyml::tag::{Tag, TagFormatError};

    #[test]
    fn test_tag_new() {
        let tag = Tag::new("tag:yaml.org,2002:test");
        assert_eq!(&*tag, b"tag:yaml.org,2002:test");
    }

    #[test]
    fn test_tag_starts_with() {
        let tag = Tag::new("tag:yaml.org,2002:test");

        // Test positive case
        assert_eq!(tag.starts_with("tag:yaml.org"), Ok(true));

        // Test negative case
        assert_eq!(tag.starts_with("tag:other.org"), Ok(false));

        // Test error case
        assert_eq!(
            tag.starts_with("tag:yaml.org,2002:test:extra"),
            Err(TagFormatError)
        );
    }

    #[test]
    fn test_tag_partial_eq() {
        let tag = Tag::new("tag:yaml.org,2002:test");

        // Test equality
        assert_eq!(tag, "tag:yaml.org,2002:test");

        // Test inequality
        assert_ne!(tag, "tag:yaml.org,2002:other");
    }

    #[test]
    fn test_tag_deref() {
        let tag = Tag::new("tag:yaml.org,2002:test");
        let tag_bytes: &[u8] = &tag;
        assert_eq!(tag_bytes, b"tag:yaml.org,2002:test");
    }

    #[test]
    fn test_tag_debug() {
        let tag = Tag::new("tag:yaml.org,2002:test");
        let debug_str = format!("{:?}", tag);
        assert_eq!(debug_str, "\"tag:yaml.org,2002:test\"");
    }

    #[test]
    fn test_tag_constants() {
        assert_eq!(Tag::NULL, "tag:yaml.org,2002:null");
        assert_eq!(Tag::BOOL, "tag:yaml.org,2002:bool");
        assert_eq!(Tag::INT, "tag:yaml.org,2002:int");
        assert_eq!(Tag::FLOAT, "tag:yaml.org,2002:float");
    }
}
