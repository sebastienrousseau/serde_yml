use crate::{
    de,
    modules::error::{self, Error, ErrorImpl},
};
use serde::{
    de::{Unexpected, Visitor},
    forward_to_deserialize_any, Deserialize, Deserializer, Serialize,
    Serializer,
};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    str::FromStr,
};

/// Represents a YAML number, which can be integer (`u64`/`i64`) or floating point (`f64`).
///
/// # Overview
/// In YAML, numeric scalars can be integers (positive or negative) or floats.
/// The [`Number`] type wraps these possibilities in one enum variant,
/// accessible by methods like [`Number::as_i64`], [`Number::as_u64`], and
/// [`Number::as_f64`].
///
/// # Returns
/// Each accessor method either returns the requested numeric value (if valid)
/// or `None`.
///
/// # Errors
/// When deserializing from YAML strings, parsing errors may occur if the input
/// cannot be interpreted as a valid integer or float.
///
/// # Examples
/// ```
/// use serde_yml::number::Number;
///
/// // Construct from an i64
/// let neg = Number::from(-123i64);
/// assert!(neg.is_i64());
/// assert_eq!(neg.as_i64(), Some(-123));
///
/// // Construct from a float
/// let float = Number::from(3.14_f64);
/// assert!(float.is_f64());
/// assert_eq!(float.as_f64(), Some(3.14));
/// ```
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Number {
    n: N,
}

/// Enum representing different variants of numbers.
///
/// # Overview
/// This enum is not directly exposed to users. Instead, [`Number`]
/// provides a public API to query whether it's a `u64`, `i64`, or `f64`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
enum N {
    /// Represents a positive integer (`u64`).
    PositiveInteger(u64),
    /// Represents a negative integer (`i64`).
    NegativeInteger(i64),
    /// Represents a floating-point value (`f64`).
    Float(f64),
}

impl Number {
    /// Returns true if the `Number` is an integer between `i64::MIN` and `i64::MAX`.
    ///
    /// For any Number on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::PositiveInteger(v) => v <= i64::MAX as u64,
            N::NegativeInteger(_) => true,
            N::Float(_) => false,
        }
    }

    /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
    ///
    /// For any Number on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::PositiveInteger(_) => true,
            N::NegativeInteger(_) | N::Float(_) => false,
        }
    }

    /// Returns true if the `Number` can be represented by f64.
    ///
    /// For any Number on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    #[inline]
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::PositiveInteger(_) | N::NegativeInteger(_) => false,
        }
    }

    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::PositiveInteger(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegativeInteger(n) => Some(n),
            N::Float(_) => None,
        }
    }

    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::PositiveInteger(n) => Some(n),
            N::NegativeInteger(_) | N::Float(_) => None,
        }
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::PositiveInteger(n) => Some(n as f64),
            N::NegativeInteger(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }

    /// Returns true if this value is NaN and false otherwise.
    #[inline]
    pub fn is_nan(&self) -> bool {
        match self.n {
            N::PositiveInteger(_) | N::NegativeInteger(_) => false,
            N::Float(f) => f.is_nan(),
        }
    }

    /// Returns true if this value is positive infinity or negative infinity and
    /// false otherwise.
    #[inline]
    pub fn is_infinite(&self) -> bool {
        match self.n {
            N::PositiveInteger(_) | N::NegativeInteger(_) => false,
            N::Float(f) => f.is_infinite(),
        }
    }

    /// Returns true if this number is neither infinite nor NaN.
    #[inline]
    pub fn is_finite(&self) -> bool {
        match self.n {
            N::PositiveInteger(_) | N::NegativeInteger(_) => true,
            N::Float(f) => f.is_finite(),
        }
    }
    /// Returns true if this number is neither infinite nor NaN.
    pub const fn from_i64(n: i64) -> Self {
        if n < 0 {
            Number {
                n: N::NegativeInteger(n),
            }
        } else {
            Number {
                n: N::PositiveInteger(n as u64),
            }
        }
    }
    /// Returns a new `Number` with the given value.
    pub const fn from_u64(n: u64) -> Self {
        Number {
            n: N::PositiveInteger(n),
        }
    }
    /// Converts to `i32`, saturating if out of range.
    ///
    /// - Positive overflow becomes `i32::MAX`.
    /// - Negative overflow becomes `i32::MIN`.
    /// - **Float values are truncated** toward zero, then clamped within `[i32::MIN, i32::MAX]`.
    pub fn to_i32_saturating(&self) -> i32 {
        match self.n {
            N::PositiveInteger(u) => {
                // Saturate on u64 > i32::MAX
                u.min(i32::MAX as u64) as i32
            }
            N::NegativeInteger(i) => {
                // Saturate on i64 < i32::MIN
                if i < i32::MIN as i64 {
                    i32::MIN
                } else {
                    i as i32
                }
            }
            N::Float(f) => {
                // Truncate and clamp within [i32::MIN, i32::MAX]
                if f.is_nan() {
                    0
                } else {
                    // You could do different rounding modes, but typically
                    // "truncate toward zero" means to cast directly to i32.
                    // Then saturate if it goes out of range.
                    let truncated = f.trunc();
                    if truncated > i32::MAX as f64 {
                        i32::MAX
                    } else if truncated < i32::MIN as f64 {
                        i32::MIN
                    } else {
                        truncated as i32
                    }
                }
            }
        }
    }

    /// Converts to `u32`, saturating if out of range.
    ///
    /// - Negative values become 0.
    /// - Positive overflow becomes `u32::MAX`.
    /// - **Float values are truncated** toward zero, then clamped within `[0, u32::MAX]`.
    pub fn to_u32_saturating(&self) -> u32 {
        match self.n {
            N::PositiveInteger(u) => {
                // Saturate on u64 > u32::MAX
                u.min(u32::MAX as u64) as u32
            }
            N::NegativeInteger(_) => {
                // Negative becomes zero
                0
            }
            N::Float(f) => {
                if f.is_nan() || f.is_sign_negative() {
                    0
                } else {
                    let truncated = f.trunc();
                    if truncated > u32::MAX as f64 {
                        u32::MAX
                    } else {
                        truncated as u32
                    }
                }
            }
        }
    }

    /// Converts to `f32` *lossily*.
    /// - Simply casts `i64` or `u64` to `f32`.
    /// - For `f64`, truncates extra precision (the usual `as f32` conversion).
    /// - This will produce `f32::INFINITY` or `f32::NEG_INFINITY` if the value
    ///   exceeds `f32` range, and normalizes `NaN` as per IEEE 754 rules.
    pub fn to_f32_lossy(&self) -> f32 {
        match self.n {
            N::PositiveInteger(u) => u as f32,
            N::NegativeInteger(i) => i as f32,
            N::Float(f) => f as f32,
        }
    }

    /// Converts to `f64` *lossily*, but returns a guaranteed finite value
    /// for non-float variants.
    /// - For `i64`/`u64`, uses direct casting.
    /// - For `f64`, just returns the original floating value (including NaN/∞).
    pub fn to_f64_lossy(&self) -> f64 {
        match self.n {
            N::PositiveInteger(u) => u as f64,
            N::NegativeInteger(i) => i as f64,
            N::Float(f) => f,
        }
    }

    /// Converts to `i16` with saturating semantics, for demonstration.
    /// You can replicate the same pattern for other numeric types.
    pub fn to_i16_saturating(&self) -> i16 {
        match self.n {
            N::PositiveInteger(u) => {
                // clamp to i16::MAX if larger
                u.min(i16::MAX as u64) as i16
            }
            N::NegativeInteger(i) => {
                if i < i16::MIN as i64 {
                    i16::MIN
                } else {
                    i as i16
                }
            }
            N::Float(f) => {
                if f.is_nan() {
                    0
                } else {
                    let truncated = f.trunc();
                    if truncated > i16::MAX as f64 {
                        i16::MAX
                    } else if truncated < i16::MIN as f64 {
                        i16::MIN
                    } else {
                        truncated as i16
                    }
                }
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.n {
            N::PositiveInteger(i) => write!(formatter, "{}", i),
            N::NegativeInteger(i) => write!(formatter, "{}", i),
            N::Float(f) if f.is_nan() => formatter.write_str(".nan"),
            N::Float(f) if f.is_infinite() => {
                if f.is_sign_negative() {
                    formatter.write_str("-.inf")
                } else {
                    formatter.write_str(".inf")
                }
            }
            N::Float(f) => {
                write!(formatter, "{}", ryu::Buffer::new().format(f))
            }
        }
    }
}

impl FromStr for Number {
    type Err = Error;

    fn from_str(repr: &str) -> Result<Self, Self::Err> {
        // 1) Attempt to parse as integer first
        if let Ok(result) = de::visit_int(NumberVisitor, repr) {
            return result;
        }

        // 2) If integer parsing failed, check if it's "digits but not a valid number."
        //    - If `digits_but_not_number(repr)` is true, we know the string
        //      has some digits but is still invalid as either integer or float.
        //    - So we immediately return a generic parse error.
        if de::digits_but_not_number(repr) {
            return Err(error::new(ErrorImpl::FailedToParseNumber));
        }

        // 3) If it's not obviously invalid, attempt to parse as float
        if let Some(float) = de::parse_f64(repr) {
            Ok(float.into())
        } else {
            // If float parsing fails here, return an explicit float parse failure
            Err(error::new(ErrorImpl::FailedToParseFloat))
        }
    }
}

impl PartialEq for N {
    fn eq(&self, other: &N) -> bool {
        match (*self, *other) {
            (N::PositiveInteger(a), N::PositiveInteger(b)) => a == b,
            (N::NegativeInteger(a), N::NegativeInteger(b)) => a == b,
            (N::Float(a), N::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    // YAML only has one NaN;
                    // the bit representation isn't preserved
                    true
                } else {
                    a == b
                }
            }
            _ => false,
        }
    }
}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (N::Float(a), N::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    // YAML only has one NaN
                    Some(Ordering::Equal)
                } else {
                    a.partial_cmp(&b)
                }
            }
            _ => Some(self.total_cmp(other)),
        }
    }
}

impl N {
    fn total_cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (N::PositiveInteger(a), N::PositiveInteger(b)) => a.cmp(&b),
            (N::NegativeInteger(a), N::NegativeInteger(b)) => a.cmp(&b),
            // negint is always less than zero
            (N::NegativeInteger(_), N::PositiveInteger(_)) => {
                Ordering::Less
            }
            (N::PositiveInteger(_), N::NegativeInteger(_)) => {
                Ordering::Greater
            }
            (N::Float(a), N::Float(b)) => {
                a.partial_cmp(&b).unwrap_or_else(|| {
                    // arbitrarily sort the NaN last
                    if !a.is_nan() {
                        Ordering::Less
                    } else if !b.is_nan() {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
            }
            // arbitrarily sort integers below floats
            (_, N::Float(_)) => Ordering::Less,
            (N::Float(_), _) => Ordering::Greater,
        }
    }
}

impl Number {
    /// Provides a total ordering against another [`Number`].
    ///
    /// Sorts integers below floats, places negative below positive, and
    /// treats all `NaN` values as “last.”
    pub(crate) fn total_cmp(&self, other: &Self) -> Ordering {
        self.n.total_cmp(&other.n)
    }
}

impl Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PositiveInteger(i) => serializer.serialize_u64(i),
            N::NegativeInteger(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}

struct NumberVisitor;

impl Visitor<'_> for NumberVisitor {
    type Value = Number;

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str("a number")
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
        Ok(value.into())
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
        Ok(value.into())
    }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<Number, E> {
        Ok(value.into())
    }
}

impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberVisitor)
    }
}

impl<'de> Deserializer<'de> for Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.n {
            N::PositiveInteger(i) => visitor.visit_u64(i),
            N::NegativeInteger(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> Deserializer<'de> for &Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.n {
            N::PositiveInteger(i) => visitor.visit_u64(i),
            N::NegativeInteger(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

macro_rules! from_signed {
    ($($signed_ty:ident)*) => {
        $(
            impl From<$signed_ty> for Number {
                #[inline]
                #[allow(clippy::cast_sign_loss)]
                fn from(i: $signed_ty) -> Self {
                    if i < 0 {
                        Number { n: N::NegativeInteger(i.try_into().unwrap()) }
                    } else {
                        Number { n: N::PositiveInteger(i as u64) }
                    }
                }
            }
        )*
    };
}

macro_rules! from_unsigned {
    ($($unsigned_ty:ident)*) => {
        $(
            impl From<$unsigned_ty> for Number {
                #[inline]
                fn from(u: $unsigned_ty) -> Self {
                    Number { n: N::PositiveInteger(u.try_into().unwrap()) }
                }
            }
        )*
    };
}

from_signed!(i8 i16 i32 i64 isize);
from_unsigned!(u8 u16 u32 u64 usize);

impl From<f32> for Number {
    fn from(f: f32) -> Self {
        Number::from(f as f64)
    }
}

impl From<f64> for Number {
    fn from(mut f: f64) -> Self {
        if f.is_nan() {
            // Destroy NaN sign, signalling, and payload. YAML only has one NaN.
            f = f64::NAN.copysign(1.0);
        }
        Number { n: N::Float(f) }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.n {
            N::PositiveInteger(u) => {
                u.hash(state);
            }
            N::NegativeInteger(i) => {
                i.hash(state);
            }
            N::Float(f) => {
                f.to_bits().hash(state);
            }
        }
    }
}

/// Returns an `Unexpected` variant based on the given `Number`.
pub(crate) fn unexpected(number: &Number) -> Unexpected<'_> {
    match number.n {
        N::PositiveInteger(u) => Unexpected::Unsigned(u),
        N::NegativeInteger(i) => Unexpected::Signed(i),
        N::Float(f) => Unexpected::Float(f),
    }
}
