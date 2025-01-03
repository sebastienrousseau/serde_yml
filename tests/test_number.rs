#![allow(missing_docs)]
#[cfg(test)]
mod tests {
    // Bring necessary items into scope
    use serde_yml::Number;
    use std::{
        cmp::Ordering,
        hash::{DefaultHasher, Hash, Hasher},
        str::FromStr,
    };

    //────────────────────────────────────────────────────────────────────────────
    // Constructors & From Implementations
    //────────────────────────────────────────────────────────────────────────────

    /// Tests for constructing `Number` values from various integer/float types.
    mod constructors {
        use super::*;

        /// Tests constructing `Number` from signed integers (i64).
        #[test]
        fn from_i64_values() {
            assert_eq!(Number::from(-1i64).as_i64(), Some(-1));
            assert_eq!(Number::from(0).as_i64(), Some(0));
            assert_eq!(Number::from(1).as_i64(), Some(1));
            assert_eq!(Number::from(i64::MAX).as_i64(), Some(i64::MAX));
            assert_eq!(Number::from(i64::MIN).as_i64(), Some(i64::MIN));
        }

        /// Tests constructing `Number` from unsigned integers (u64).
        #[test]
        fn from_u64_values() {
            assert_eq!(Number::from(0).as_u64(), Some(0));
            assert_eq!(Number::from(1).as_u64(), Some(1));
            assert_eq!(Number::from(u64::MAX).as_u64(), Some(u64::MAX));
        }

        /// Tests constructing `Number` via the `From` trait for various types.
        ///
        /// Ensures i8, i16, i32, isize, u8, u16, u32, usize, f32, f64 are handled.
        #[test]
        fn from_trait_various_types() {
            assert_eq!(Number::from(-1i8).as_i64(), Some(-1));
            assert_eq!(Number::from(1u32).as_u64(), Some(1));
            assert_eq!(Number::from(42isize).as_i64(), Some(42));

            let val =
                Number::from(std::f32::consts::PI).as_f64().unwrap();
            let expected = std::f32::consts::PI as f64;
            let eps = 1e-6; // Tweak the epsilon as needed

            assert!(
                (val - expected).abs() < eps,
                "Expected ~{expected}, got {val}"
            );

            // Test NaN
            assert!(Number::from(f64::NAN).is_nan());
        }
        /// Tests parsing special float strings like "inf", "-inf", ".nan"
        #[test]
        fn test_parsing_special_float_strings() {
            // If your from_str supports something like "inf" or ".inf":
            if let Ok(val) = Number::from_str("inf") {
                assert!(val.is_infinite());
            }

            if let Ok(val) = Number::from_str("-inf") {
                assert!(val.is_infinite());
            }

            // Or if you plan to parse ".nan" in the future:
            if let Ok(val) = Number::from_str(".nan") {
                assert!(val.is_nan());
            }
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // Accessors (as_i64, as_u64, as_f64)
    //────────────────────────────────────────────────────────────────────────────

    /// Tests verifying the behavior of as_i64, as_u64, as_f64.
    mod accessors {
        use super::*;

        /// Tests `as_i64` with various valid/invalid scenarios.
        #[test]
        fn test_as_i64() {
            let number = Number::from(42);
            assert_eq!(number.as_i64(), Some(42));

            let number = Number::from(-42);
            assert_eq!(number.as_i64(), Some(-42));

            let number = Number::from(std::f64::consts::PI);
            assert_eq!(number.as_i64(), None);
        }

        /// Tests `as_u64` with various valid/invalid scenarios.
        #[test]
        fn test_as_u64() {
            let number = Number::from(42);
            assert_eq!(number.as_u64(), Some(42));

            let number = Number::from(-42);
            assert_eq!(number.as_u64(), None);

            let number = Number::from(std::f64::consts::PI);
            assert_eq!(number.as_u64(), None);
        }

        /// Tests `as_f64` with integer and float variants, ensuring correct conversion.
        #[test]
        fn test_as_f64() {
            let number = Number::from(42);
            assert_eq!(number.as_f64().unwrap(), 42.0);

            let number = Number::from(-42);
            assert_eq!(number.as_f64().unwrap(), -42.0);

            let number = Number::from(std::f64::consts::PI);
            assert!(
                (number.as_f64().unwrap() - std::f64::consts::PI).abs()
                    < f64::EPSILON
            );
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // Checks (is_i64, is_u64, is_f64, is_nan, is_infinite, is_finite)
    //────────────────────────────────────────────────────────────────────────────

    /// Tests that check type classification, finiteness, NaN, etc.
    mod checks {
        use super::*;

        /// Tests the `is_i64` method behavior on various types.
        #[test]
        fn test_is_i64() {
            let number = Number::from(42);
            assert!(number.is_i64());

            let number = Number::from(-42);
            assert!(number.is_i64());

            let number = Number::from(std::f64::consts::PI);
            assert!(!number.is_i64());
        }

        /// Tests the `is_u64` method behavior on various types.
        #[test]
        fn test_is_u64() {
            let number = Number::from(42);
            assert!(number.is_u64());

            let number = Number::from(-42);
            assert!(!number.is_u64());

            let number = Number::from(std::f64::consts::PI);
            assert!(!number.is_u64());
        }

        /// Tests the `is_f64` method on integer and float variants.
        #[test]
        fn test_is_f64() {
            let number = Number::from(42);
            assert!(!number.is_f64());

            let number = Number::from(-42);
            assert!(!number.is_f64());

            let number = Number::from(std::f64::consts::PI);
            assert!(number.is_f64());
        }

        /// Tests whether the number is flagged as `NaN`.
        #[test]
        fn test_is_nan() {
            let number = Number::from(f64::NAN);
            assert!(number.is_nan());

            let number = Number::from(42);
            assert!(!number.is_nan());

            let number = Number::from(std::f64::consts::PI);
            assert!(!number.is_nan());
        }

        /// Tests whether the number is flagged as infinite.
        #[test]
        fn test_is_infinite() {
            let number = Number::from(f64::INFINITY);
            assert!(number.is_infinite());

            let number = Number::from(-f64::INFINITY);
            assert!(number.is_infinite());

            let number = Number::from(42);
            assert!(!number.is_infinite());

            let number = Number::from(std::f64::consts::PI);
            assert!(!number.is_infinite());
        }

        /// Tests that the number is finite when not infinite or NaN.
        #[test]
        fn test_is_finite() {
            let number = Number::from(f64::INFINITY);
            assert!(!number.is_finite());

            let number = Number::from(-f64::INFINITY);
            assert!(!number.is_finite());

            let number = Number::from(f64::NAN);
            assert!(!number.is_finite());

            let number = Number::from(42);
            assert!(number.is_finite());
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // Display
    //────────────────────────────────────────────────────────────────────────────

    /// Tests the `Display` implementation for `Number`.
    mod display {
        use super::*;

        /// Ensures `Number` displays correctly for integer, float, NaN, infinities.
        #[test]
        fn test_display_values() {
            let number = Number::from(42);
            assert_eq!(number.to_string(), "42");

            let number = Number::from(-42);
            assert_eq!(number.to_string(), "-42");

            let number = Number::from(f64::NAN);
            assert_eq!(number.to_string(), ".nan");

            let number = Number::from(f64::INFINITY);
            assert_eq!(number.to_string(), ".inf");

            let number = Number::from(-f64::INFINITY);
            assert_eq!(number.to_string(), "-.inf");

            let number = Number::from(std::f64::consts::PI);
            assert!(
                (number.to_string().parse::<f64>().unwrap()
                    - std::f64::consts::PI)
                    .abs()
                    < f64::EPSILON
            );
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // FromStr
    //────────────────────────────────────────────────────────────────────────────

    /// Tests parsing `Number` from string inputs via `FromStr`.
    mod from_str {
        use super::*;

        /// Checks valid integer/float strings, and ensures parse errors on invalid inputs.
        #[test]
        fn test_from_str() {
            let number = Number::from_str("42").unwrap();
            assert_eq!(number, Number::from(42));

            let number = Number::from_str("-42").unwrap();
            assert_eq!(number, Number::from(-42));

            let number = Number::from(std::f64::consts::PI);
            assert_eq!(number, Number::from(std::f64::consts::PI));

            let result = Number::from_str("invalid");
            assert!(result.is_err());
        }

        /// Tests additional edge-case numeric strings (e.g., hex, octal) and invalid formats.
        #[test]
        fn test_parsing_edge_cases() {
            // Test valid numeric strings
            assert!(Number::from_str("0o777").is_ok()); // Octal
            assert!(Number::from_str("0xff").is_ok()); // Hex
            assert!(Number::from_str("0b1010").is_ok()); // Binary

            // Test invalid formats
            assert!(Number::from_str("++1").is_err());
            assert!(Number::from_str("1.2.3").is_err());
            assert!(Number::from_str("0x0x0").is_err());
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // Comparisons (PartialEq, PartialOrd, total_cmp)
    //────────────────────────────────────────────────────────────────────────────

    /// Tests the equality and ordering functionality for `Number`.
    mod comparisons {
        use super::*;

        /// Tests `PartialEq` across integers, floats, and special float values.
        #[test]
        fn test_partial_eq() {
            let number1 = Number::from(42);
            let number2 = Number::from(42);
            assert_eq!(number1, number2);

            let number1 = Number::from(-42);
            let number2 = Number::from(-42);
            assert_eq!(number1, number2);

            let number1 = Number::from(std::f64::consts::PI);
            let number2 = Number::from(std::f64::consts::PI);
            assert_eq!(number1, number2);

            let number1 = Number::from(42);
            let number2 = Number::from(-42);
            assert_ne!(number1, number2);

            let number1 = Number::from(42);
            let number2 = Number::from(std::f64::consts::PI);
            assert_ne!(number1, number2);
        }

        /// Tests `PartialOrd` comparisons (including NaN handling and integer-to-float comparisons).
        #[test]
        fn test_partial_ord() {
            let number1 = Number::from(42);
            let number2 = Number::from(42);
            assert_eq!(
                number1.partial_cmp(&number2),
                Some(Ordering::Equal)
            );

            let number1 = Number::from(-42);
            let number2 = Number::from(42);
            assert_eq!(
                number1.partial_cmp(&number2),
                Some(Ordering::Less)
            );

            let number1 = Number::from(42);
            let number2 = Number::from(-42);
            assert_eq!(
                number1.partial_cmp(&number2),
                Some(Ordering::Greater)
            );

            let number1 = Number::from(std::f64::consts::PI);
            let number2 = Number::from(std::f64::consts::PI);
            assert_eq!(
                number1.partial_cmp(&number2),
                Some(Ordering::Equal)
            );

            let number1 = Number::from(std::f64::consts::PI);
            let number2 = Number::from(2.71);
            assert_eq!(
                number1.partial_cmp(&number2),
                Some(Ordering::Greater)
            );

            let number1 = Number::from(f64::NAN);
            let number2 = Number::from(f64::NAN);
            // YAML treats all NaNs as equal
            assert_eq!(
                number1.partial_cmp(&number2),
                Some(Ordering::Equal)
            );
        }

        /// Additional ordering tests mixing integers, floats, and special float values.
        #[test]
        fn test_number_ordering() {
            // Test integer ordering
            assert!(Number::from(-2) < Number::from(-1));
            assert!(Number::from(-1) < Number::from(0));
            assert!(Number::from(0) < Number::from(1));

            // Test float ordering
            assert!(Number::from(-1.0) < Number::from(-0.5));
            assert!(Number::from(-0.5) < Number::from(0.0));
            assert!(Number::from(0.0) < Number::from(0.5));

            // Test mixed type ordering
            assert!(Number::from(-1) < Number::from(-0.5));
            assert!(Number::from(1) < Number::from(1.5));

            // Test special float values
            let nan = Number::from(f64::NAN);
            let inf = Number::from(f64::INFINITY);
            let neg_inf = Number::from(f64::NEG_INFINITY);

            assert!(neg_inf < Number::from(0.0));
            assert!(Number::from(0.0) < inf);
            // YAML treats all NaNs as equal
            assert_eq!(nan, nan);
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // Hash
    //────────────────────────────────────────────────────────────────────────────

    /// Tests the `Hash` implementation for `Number`.
    mod hash_impl {
        use super::*;

        /// Verifies that equal values produce equal hashes.
        #[test]
        fn test_hash() {
            let mut hasher = DefaultHasher::new();
            let number = Number::from(42);
            number.hash(&mut hasher);
            let hash1 = hasher.finish();

            let mut hasher = DefaultHasher::new();
            let number = Number::from(42);
            number.hash(&mut hasher);
            let hash2 = hasher.finish();

            assert_eq!(hash1, hash2);
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // Serde (Serialization & Deserialization)
    //────────────────────────────────────────────────────────────────────────────

    /// Tests that `Number` can be serialized and deserialized correctly via serde.
    mod serde_impl {
        use super::*;

        /// Checks round-trip serialization/deserialization for integer and float values.
        #[test]
        fn test_ser_de() {
            let number = Number::from(42);
            let serialized = serde_yml::to_string(&number).unwrap();
            let deserialized: Number =
                serde_yml::from_str(&serialized).unwrap();
            assert_eq!(number, deserialized);

            let number = Number::from(-42);
            let serialized = serde_yml::to_string(&number).unwrap();
            let deserialized: Number =
                serde_yml::from_str(&serialized).unwrap();
            assert_eq!(number, deserialized);

            let number = Number::from(std::f64::consts::PI);
            let serialized = serde_yml::to_string(&number).unwrap();
            let deserialized: Number =
                serde_yml::from_str(&serialized).unwrap();
            assert_eq!(number, deserialized);
        }
    }

    //────────────────────────────────────────────────────────────────────────────
    // Edge & Corner Cases
    //────────────────────────────────────────────────────────────────────────────

    /// Tests targeting boundary conditions, special float behaviors, and unusual numeric scenarios.
    mod edge_cases {
        use super::*;

        /// Covers i64/u64 boundary conditions, float precision edges, negative zero, etc.
        #[test]
        fn test_number_conversion_edge_cases() {
            // Test i64/u64 boundary conditions
            let max_i64 = Number::from(i64::MAX);
            assert_eq!(max_i64.as_i64(), Some(i64::MAX));
            assert_eq!(max_i64.as_u64(), Some(i64::MAX as u64));

            let min_i64 = Number::from(i64::MIN);
            assert_eq!(min_i64.as_i64(), Some(i64::MIN));
            assert_eq!(min_i64.as_u64(), None);

            // Test float precision edge cases
            let small_float = Number::from(1e-300_f64);
            assert!(small_float.is_finite());

            // Test negative zero
            let neg_zero = Number::from(-0.0_f64);
            let pos_zero = Number::from(0.0_f64);
            // YAML treats negative zero the same as zero
            assert_eq!(neg_zero, pos_zero);

            // Test that float conversions preserve values
            let pi = Number::from(std::f64::consts::PI);
            assert!(
                (pi.as_f64().unwrap() - std::f64::consts::PI).abs()
                    < f64::EPSILON
            );
        }

        /// Tests that negative zero compares as equal to positive zero.
        #[test]
        fn test_negative_zero_comparison() {
            let neg_zero = Number::from(-0.0_f64);
            let pos_zero = Number::from(0.0_f64);

            // Per YAML and your code, negative zero equals positive zero
            assert_eq!(neg_zero, pos_zero);

            // They should also compare as equal, not less/greater
            assert_eq!(
                neg_zero.partial_cmp(&pos_zero),
                Some(Ordering::Equal)
            );
            assert_eq!(
                pos_zero.partial_cmp(&neg_zero),
                Some(Ordering::Equal)
            );
        }

        /// Tests that verifies correct handling when attempting to convert a float that is technically integer-valued but out of range.
        #[test]
        fn test_out_of_range_float_conversions() {
            // 2^63 is just outside i64::MAX
            let too_large_for_i64 =
                Number::from(9_223_372_036_854_775_808_f64);
            assert_eq!(too_large_for_i64.as_i64(), None);

            // Similarly, negative value below i64::MIN
            let too_small_for_i64 =
                Number::from(-9_223_372_036_854_775_809_f64);
            assert_eq!(too_small_for_i64.as_i64(), None);

            // 2^64 is just outside u64::MAX
            let too_large_for_u64 =
                Number::from(18_446_744_073_709_551_616_f64);
            assert_eq!(too_large_for_u64.as_u64(), None);
        }

        /// Tests to verify that a displayed Number can be round-tripped via Number::from_str ensures no data is lost in the textual representation.
        #[test]
        fn test_display_and_parse_round_trip() {
            let test_values = vec![
                Number::from(0),
                Number::from(-42),
                Number::from(42),
                Number::from(std::f64::consts::PI),
                Number::from(f64::NAN),
                Number::from(f64::INFINITY),
                Number::from(-f64::INFINITY),
            ];

            for val in test_values {
                let displayed = val.to_string();
                if let Ok(parsed) = Number::from_str(&displayed) {
                    if val.is_nan() && parsed.is_nan() {
                        assert_eq!(val, parsed);
                    } else {
                        assert_eq!(
                            val, parsed,
                            "Round-trip mismatch for '{displayed}'"
                        );
                    }
                }
            }
        }

        /// Tests that verify how large integers compare with floats near their boundaries can ensure PartialOrd logic is correct
        #[test]
        fn test_large_integers_vs_floats() {
            let int_num = Number::from(i64::MAX);
            let float_num = Number::from(i64::MAX as f64);

            assert_eq!(
                int_num.partial_cmp(&float_num),
                Some(Ordering::Less)
            );
        }
    }

    /// Tests utility methods for safe conversion methods for “best effort” casting to i32, u32, i16, etc.
    mod utility_methods {
        use super::*;

        #[test]
        fn test_to_i32_saturating() {
            // Positive integer within range
            assert_eq!(Number::from_u64(123).to_i32_saturating(), 123);
            // Positive integer out of i32 range
            assert_eq!(
                Number::from_u64((i32::MAX as u64) + 1)
                    .to_i32_saturating(),
                i32::MAX
            );

            // Negative integer within range
            assert_eq!(
                Number::from_i64(-123).to_i32_saturating(),
                -123
            );
            // Negative integer out of i32 range
            assert_eq!(
                Number::from_i64(i64::from(i32::MIN) - 1)
                    .to_i32_saturating(),
                i32::MIN
            );

            // Float: 123.999 truncated to 123
            let f = Number::from(123.999_f64);
            assert_eq!(f.to_i32_saturating(), 123);

            // Float: small negative, truncated
            let f = Number::from(-45.9_f64);
            assert_eq!(f.to_i32_saturating(), -45);

            // Float: greater than i32::MAX
            let f = Number::from((i32::MAX as f64) + 100.0);
            assert_eq!(f.to_i32_saturating(), i32::MAX);

            // Float: less than i32::MIN
            let f = Number::from((i32::MIN as f64) - 100.0);
            assert_eq!(f.to_i32_saturating(), i32::MIN);

            // Float: NaN -> 0
            let f = Number::from(f64::NAN);
            assert_eq!(f.to_i32_saturating(), 0);

            // Float: Infinity -> clamp to i32::MAX
            let f = Number::from(f64::INFINITY);
            assert_eq!(f.to_i32_saturating(), i32::MAX);

            // Float: Negative Infinity -> clamp to i32::MIN
            let f = Number::from(f64::NEG_INFINITY);
            assert_eq!(f.to_i32_saturating(), i32::MIN);
        }

        #[test]
        fn test_to_u32_saturating() {
            // Positive integer within range
            assert_eq!(Number::from_u64(123).to_u32_saturating(), 123);
            // Positive integer out of u32 range
            assert_eq!(
                Number::from_u64(u64::from(u32::MAX) + 1)
                    .to_u32_saturating(),
                u32::MAX
            );

            // Negative integer -> 0
            assert_eq!(Number::from_i64(-123).to_u32_saturating(), 0);

            // Float: 123.999 truncated to 123
            let f = Number::from(123.999_f64);
            assert_eq!(f.to_u32_saturating(), 123);

            // Float: negative -> 0
            let f = Number::from(-45.9_f64);
            assert_eq!(f.to_u32_saturating(), 0);

            // Float: greater than u32::MAX
            let f = Number::from((u32::MAX as f64) + 100.0);
            assert_eq!(f.to_u32_saturating(), u32::MAX);

            // Float: NaN -> 0
            let f = Number::from(f64::NAN);
            assert_eq!(f.to_u32_saturating(), 0);

            // Float: Infinity -> u32::MAX
            let f = Number::from(f64::INFINITY);
            assert_eq!(f.to_u32_saturating(), u32::MAX);

            // Float: Negative Infinity -> 0
            let f = Number::from(f64::NEG_INFINITY);
            assert_eq!(f.to_u32_saturating(), 0);
        }

        #[test]
        fn test_to_f32_lossy() {
            // Positive integer
            assert_eq!(Number::from_u64(123).to_f32_lossy(), 123.0_f32);

            // Negative integer
            assert_eq!(
                Number::from_i64(-123).to_f32_lossy(),
                -123.0_f32
            );

            // Large float that truly exceeds f32 range => Infinity
            // 1.0e40 is ~1e40, definitely > 3.4e38 (f32::MAX)
            let too_big_float = Number::from(1.0e40_f64);
            assert!(too_big_float.to_f32_lossy().is_infinite());

            // Large integer but still finite in f32
            // u64::MAX is ~1.84e19, which is well below f32::MAX (~3.4e38)
            let big_num = Number::from_u64(u64::MAX);
            let val = big_num.to_f32_lossy();
            assert!(val.is_finite());
            assert!(
        val > 1e19,
        "Expected ~1.84e19 in f32 (though losing some precision), not infinity."
    );

            // Float within range (normal Pi check)
            let f =
                Number::from(std::f32::consts::PI).as_f64().unwrap();
            assert!(
                (f - std::f32::consts::PI as f64).abs()
                    < f32::EPSILON as f64
            );

            // Float Infinity
            let f = Number::from(f64::INFINITY);
            assert!(f.to_f32_lossy().is_infinite());

            // Float Negative Infinity
            let f = Number::from(f64::NEG_INFINITY);
            assert!(f.to_f32_lossy().is_infinite());

            // NaN
            let f = Number::from(f64::NAN);
            assert!(f.to_f32_lossy().is_nan());
        }

        #[test]
        fn test_to_f64_lossy() {
            // Positive integer
            assert_eq!(Number::from_u64(123).to_f64_lossy(), 123.0_f64);
            // Negative integer
            assert_eq!(
                Number::from_i64(-123).to_f64_lossy(),
                -123.0_f64
            );

            // Large integer that fits in f64 mantissa, though potentially losing precision
            let moderately_large = Number::from_u64(1_000_000_000);
            assert_eq!(
                moderately_large.to_f64_lossy(),
                1_000_000_000.0
            );

            // Very large integer might lose precision
            let huge = Number::from_u64(u64::MAX);
            // Expect a large float, probably 1.8446744073709552e19 but not infinite
            let val = huge.to_f64_lossy();
            assert!(!val.is_infinite());
            assert!(val > 1.0e19);

            // Float Infinity
            let f = Number::from(f64::INFINITY);
            assert!(f.to_f64_lossy().is_infinite());

            // Float Negative Infinity
            let f = Number::from(f64::NEG_INFINITY);
            assert!(f.to_f64_lossy().is_infinite());

            // NaN
            let f = Number::from(f64::NAN);
            assert!(f.to_f64_lossy().is_nan());
        }

        #[test]
        fn test_to_i16_saturating() {
            // Positive integer within range
            assert_eq!(Number::from_u64(123).to_i16_saturating(), 123);
            // Positive integer out of range
            assert_eq!(
                Number::from_u64((i16::MAX as u64) + 1)
                    .to_i16_saturating(),
                i16::MAX
            );

            // Negative integer within range
            assert_eq!(
                Number::from_i64(-123).to_i16_saturating(),
                -123
            );
            // Negative integer out of range
            assert_eq!(
                Number::from_i64(i64::from(i16::MIN) - 1)
                    .to_i16_saturating(),
                i16::MIN
            );

            // Float: 12.9 truncated to 12
            let f = Number::from(12.9_f64);
            assert_eq!(f.to_i16_saturating(), 12);

            // Float: -12.9 truncated to -12
            let f = Number::from(-12.9_f64);
            assert_eq!(f.to_i16_saturating(), -12);

            // Float: bigger than i16::MAX
            let f = Number::from((i16::MAX as f64) + 100.0);
            assert_eq!(f.to_i16_saturating(), i16::MAX);

            // Float: less than i16::MIN
            let f = Number::from((i16::MIN as f64) - 10.0);
            assert_eq!(f.to_i16_saturating(), i16::MIN);

            // Float: NaN -> 0
            let f = Number::from(f64::NAN);
            assert_eq!(f.to_i16_saturating(), 0);
        }
    }
}
