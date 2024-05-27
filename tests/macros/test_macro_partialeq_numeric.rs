mod tests {
    use serde_yml::Value;

    /// Tests the partial equality of a numeric i8 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_i8() {
        let v1: Value = 10i8.into();
        assert_eq!(v1, 10i8);
    }

    /// Tests the partial equality of a numeric i16 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_i16() {
        let v1: Value = 10i16.into();
        assert_eq!(v1, 10i16);
    }

    /// Tests the partial equality of a numeric i32 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_i32() {
        let v1: Value = 10i32.into();
        assert_eq!(v1, 10i32);
    }

    /// Tests the partial equality of a numeric i64 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_i64() {
        let v1: Value = 10i64.into();
        assert_eq!(v1, 10i64);
    }

    /// Tests the partial equality of a numeric isize value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_isize() {
        let v1: Value = 10isize.into();
        assert_eq!(v1, 10isize);
    }

    /// Tests the partial equality of a numeric u8 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_u8() {
        let v1: Value = 10u8.into();
        assert_eq!(v1, 10u8);
    }

    /// Tests the partial equality of a numeric u16 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_u16() {
        let v1: Value = 10u16.into();
        assert_eq!(v1, 10u16);
    }

    /// Tests the partial equality of a numeric u32 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_u32() {
        let v1: Value = 10u32.into();
        assert_eq!(v1, 10u32);
    }

    /// Tests the partial equality of a numeric u64 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_u64() {
        let v1: Value = 10u64.into();
        assert_eq!(v1, 10u64);
    }

    /// Tests the partial equality of a numeric usize value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_usize() {
        let v1: Value = 10usize.into();
        assert_eq!(v1, 10usize);
    }

    /// Tests the partial equality of a numeric f32 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_f32() {
        let v1: Value = 10f32.into();
        assert_eq!(v1, 10f32);
    }

    /// Tests the partial equality of a numeric f64 value converted to a `serde_yml::Value`.
    #[test]
    fn test_partialeq_numeric_f64() {
        let v1: Value = 10f64.into();
        assert_eq!(v1, 10f64);
    }
}
