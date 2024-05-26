/// A macro that generates implementations of the `PartialEq` trait for
/// primitive numeric types and `Value` based on the specified conversion
/// method and base type.
///
/// # Examples
///
/// ```
/// use serde_yml::Value;
///
/// let v1: Value = 10.into();
/// assert_eq!(v1, 10);
///
/// let v2: Value = serde_yml::from_str("10").unwrap();
/// assert_eq!(v2, 10);
/// ```
#[macro_export]
macro_rules! partialeq_numeric {
    ($([$($ty:ty)*], $conversion:ident, $base:ty)*) => {
        $($(
            impl PartialEq<$ty> for Value {
                fn eq(&self, other: &$ty) -> bool {
                    self.$conversion().map_or(false, |i| compare_numeric(i, (*other).try_into().unwrap()))
                }
            }

            impl PartialEq<$ty> for &Value {
                fn eq(&self, other: &$ty) -> bool {
                    self.$conversion().map_or(false, |i| compare_numeric(i, (*other).try_into().unwrap()))
                }
            }

            impl PartialEq<$ty> for &mut Value {
                fn eq(&self, other: &$ty) -> bool {
                    self.$conversion().map_or(false, |i| compare_numeric(i, (*other).try_into().unwrap()))
                }
            }
        )*)*
    }
}
