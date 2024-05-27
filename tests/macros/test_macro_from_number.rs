#[cfg(test)]
mod tests {
    use serde_yml::Number;
    use serde_yml::Value;

    /// Test converting an i8 to a Value.
    #[test]
    fn test_from_number_i8() {
        let num: i8 = 42;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(42)));
    }

    /// Test converting an i16 to a Value.
    #[test]
    fn test_from_number_i16() {
        let num: i16 = 1337;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(1337)));
    }

    /// Test converting an i32 to a Value.
    #[test]
    fn test_from_number_i32() {
        let num: i32 = -100_000;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(-100_000)));
    }

    /// Test converting an i64 to a Value.
    #[test]
    fn test_from_number_i64() {
        let num: i64 = 9_223_372_036_854_775_807;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num)));
    }

    /// Test converting a u8 to a Value.
    #[test]
    fn test_from_number_u8() {
        let num: u8 = 255;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(255)));
    }

    /// Test converting a u16 to a Value.
    #[test]
    fn test_from_number_u16() {
        let num: u16 = 65_535;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(65_535)));
    }

    /// Test converting a u32 to a Value.
    #[test]
    fn test_from_number_u32() {
        let num: u32 = 4_294_967_295;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num)));
    }
    /// Test converting a u64 to a Value.
    #[test]
    fn test_from_number_u64() {
        let num: u64 = 18_446_744_073_709_551_615;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num)));
    }

    /// Test converting an f32 to a Value.
    #[test]
    fn test_from_number_f32() {
        let num: f32 = 3.14;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num as f64)));
    }

    /// Test converting an f64 to a Value.
    #[test]
    fn test_from_number_f64() {
        let num: f64 = 2.71828;
        let value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num)));
    }
}
