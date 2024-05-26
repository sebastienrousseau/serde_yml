#[cfg(test)]
mod tests {
    use serde_yml::libyml::safe_cstr::CStr;
    use std::ptr::NonNull;

    #[test]
    fn test_from_ptr() {
        let valid_bytes = b"hello\0";
        let ptr = NonNull::from(valid_bytes).cast();
        let cstr = CStr::from_ptr(ptr);
        assert_eq!(
            cstr.to_bytes(),
            &valid_bytes[..valid_bytes.len() - 1]
        );
    }

    #[test]
    fn test_len() {
        let valid_bytes = b"hello\0";
        let cstr = CStr::from_bytes_with_nul(valid_bytes).unwrap();
        assert_eq!(cstr.len(), 5);

        let empty_bytes = b"";
        let result = CStr::from_bytes_with_nul(empty_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_bytes() {
        let valid_bytes = b"hello\0";
        let cstr = CStr::from_bytes_with_nul(valid_bytes).unwrap();
        assert_eq!(
            cstr.to_bytes(),
            &valid_bytes[..valid_bytes.len() - 1]
        );

        let empty_bytes = b"";
        let result = CStr::from_bytes_with_nul(empty_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let valid_bytes = b"hello\0";
        let cstr = CStr::from_bytes_with_nul(valid_bytes).unwrap();
        assert_eq!(format!("{}", cstr), "hello");

        let invalid_bytes = b"hello\xff\0";
        let cstr = CStr::from_bytes_with_nul(invalid_bytes).unwrap();
        assert_eq!(format!("{}", cstr), "hello�");
    }

    #[test]
    fn test_debug() {
        let valid_bytes = b"hello\0";
        let cstr = CStr::from_bytes_with_nul(valid_bytes).unwrap();
        assert_eq!(format!("{:?}", cstr), "\"hello\"");

        let invalid_bytes = b"hello\xff\0";
        let cstr = CStr::from_bytes_with_nul(invalid_bytes).unwrap();
        assert_eq!(format!("{:?}", cstr), "\"hello\\xff\"");
    }
}
