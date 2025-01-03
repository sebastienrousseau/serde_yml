//! Customizations to use with Serde's `#[serde(with = …)]` attribute.

/// Serialize/deserialize an enum using a YAML map containing one entry in which
/// the key identifies the variant name.
///
/// # Overview
///
/// This module provides functionality that wraps each enum variant in a
/// "singleton map." This means each enum is serialized into a YAML map that
/// has exactly one key–the variant name–and a corresponding value that
/// contains the data for that variant.
///
/// # Returns
///
/// By applying `#[serde(with = "serde_yml::with::singleton_map")]` to an enum
/// field, that field will be serialized and deserialized using a singleton map
/// representation:
///
/// ```yaml
/// VariantName:
///   ...
/// ```
///
/// # Errors
///
/// Serialization or deserialization errors can occur if:
/// - The provided data does not match the singleton map structure.
/// - An unknown variant name is encountered, or the underlying I/O fails.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum Enum {
///     Unit,
///     Newtype(usize),
///     Tuple(usize, usize),
///     Struct { value: usize },
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Struct {
///     #[serde(with = "serde_yml::with::singleton_map")]
///     w: Enum,
///     #[serde(with = "serde_yml::with::singleton_map")]
///     x: Enum,
///     #[serde(with = "serde_yml::with::singleton_map")]
///     y: Enum,
///     #[serde(with = "serde_yml::with::singleton_map")]
///     z: Enum,
/// }
///
///  let object = Struct {
///      w: Enum::Unit,
///      x: Enum::Newtype(1),
///      y: Enum::Tuple(1, 1),
///      z: Enum::Struct { value: 1 },
///  };
///
///  let yaml = serde_yml::to_string(&object).unwrap();
///  print!("{}", yaml);
///
///  let deserialized: Struct = serde_yml::from_str(&yaml).unwrap();
///  assert_eq!(object, deserialized);
/// ```
///
/// The representation using `singleton_map` on all fields is:
///
/// ```yaml
/// w: Unit
/// x:
///   Newtype: 1
/// y:
///   Tuple:
///   - 1
///   - 1
/// z:
///   Struct:
///     value: 1
/// ```
///
/// Without `singleton_map`, the default behavior would have been to serialize
/// as:
///
/// ```yaml
/// w: Unit
/// x: !Newtype 1
/// y: !Tuple
/// - 1
/// - 1
/// z: !Struct
///   value: 1
/// ```
pub mod singleton_map {
    use crate::value::{Mapping, Sequence};
    use serde::de::{
        self, Deserialize, DeserializeSeed, Deserializer, EnumAccess,
        IgnoredAny, MapAccess, Unexpected, VariantAccess, Visitor,
    };
    use serde::ser::{
        self, Serialize, SerializeMap, SerializeStructVariant,
        SerializeTupleVariant, Serializer,
    };
    use std::fmt::{self, Display};

    /// Serializes a given value using a singleton map representation.
    ///
    /// # Overview
    ///
    /// Converts the provided value (typically an enum) into a YAML map with
    /// a single entry: the variant name is the key, and the variant data
    /// is the value.
    ///
    /// # Returns
    ///
    /// A `Result<S::Ok, S::Error>` indicating whether serialization was
    /// successful. If successful, `S::Ok` is returned.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - The provided value cannot be represented as a singleton map.
    /// - An I/O or structural error occurs in the underlying serializer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use serde_yml::with::singleton_map;
    ///
    /// #[derive(Serialize, Deserialize, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Unit,
    ///     Newtype(u32),
    ///     Struct { x: i64 },
    /// }
    ///
    /// let value = MyEnum::Newtype(42);
    /// let yaml = serde_yml::to_string(&value).unwrap();
    /// assert!(yaml.contains("Newtype"));
    /// ```
    pub fn serialize<T, S>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        // We simply forward to our `SingletonMap` wrapper, which enforces the "singleton map" layout.
        value.serialize(SingletonMap {
            delegate: serializer,
        })
    }

    /// Deserializes a value that was serialized using the singleton map representation.
    ///
    /// # Overview
    ///
    /// Expects a YAML map with exactly one key: the variant name. The value
    /// associated with that key is used to reconstruct the original variant data.
    ///
    /// # Returns
    ///
    /// A `Result<T, D::Error>` which contains the reconstructed type `T` on
    /// success.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - The input is not a map with exactly one key-value pair.
    /// - The key is not recognized as a valid variant name.
    /// - There is an I/O or structural mismatch during deserialization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde::{Serialize, Deserialize};
    /// # use serde_yml::with::singleton_map;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize, Deserialize, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Unit,
    ///     Newtype(u32),
    ///     Struct { x: i64 },
    /// }
    ///
    /// let yaml = r#"Struct:
    ///   x: 123
    /// "#;
    ///
    /// let my_enum: MyEnum = singleton_map::deserialize(
    ///     serde_yml::Deserializer::from_str(yaml)
    /// )?;
    ///
    /// assert_eq!(my_enum, MyEnum::Struct { x: 123 });
    /// # Ok(())
    /// # }
    /// ```
    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::deserialize(SingletonMap {
            delegate: deserializer,
        })
    }

    /// A lightweight wrapper that enforces the "singleton map" layout for
    /// serialization and deserialization of enums.
    ///
    /// # Overview
    ///
    /// During serialization, each enum variant is emitted as a key-value pair,
    /// with the key holding the variant name and the value holding the variant
    /// fields. During deserialization, the wrapper expects a similar structure
    /// and reconstructs the appropriate variant.
    ///
    /// # Returns
    ///
    /// This wrapper is used internally and does not directly return data.
    /// Instead, it delegates all serialization or deserialization tasks
    /// to the underlying `delegate`.
    ///
    /// # Errors
    ///
    /// Errors depend on the underlying serializer or deserializer, typically
    /// arising when the data does not match the singleton map format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde::{Serialize, Deserialize};
    /// # use serde_yml::with::singleton_map::SingletonMap;
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// enum MyEnum {
    ///     Unit,
    ///     Newtype(u32),
    /// }
    ///
    /// // Typically you won't instantiate `SingletonMap` directly, but rely on
    /// // `serialize` / `deserialize` or `#[serde(with = "singleton_map")]`.
    /// ```
    #[derive(Clone, Copy, Debug)]
    pub struct SingletonMap<D> {
        /// The underlying serializer or deserializer that actually performs I/O.
        pub delegate: D,
    }

    impl<D> Serialize for SingletonMap<D>
    where
        D: Serialize,
    {
        /// # Overview
        ///
        /// Allows the `SingletonMap` wrapper to implement [`Serialize`].
        /// Any nested enums encountered during serialization are also handled
        /// in the same singleton map style.
        ///
        /// # Returns
        ///
        /// `Ok` if the serialization is successful, or `Err` if the underlying
        /// serializer fails.
        ///
        /// # Errors
        ///
        /// Can fail if the delegate serializer encounters an error or if the
        /// data cannot be formatted as required.
        ///
        /// # Examples
        ///
        /// See [`crate::with::singleton_map::serialize`] for usage examples.
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.delegate.serialize(SingletonMap {
                delegate: serializer,
            })
        }
    }

    // --- The remaining implementations are private details of how the
    // --- singleton map representation is enforced for all Rust data types.
    // --- They follow the same conceptual approach: hooking into serialization
    // --- and deserialization to maintain a one-key-per-variant structure.

    impl<D> Serializer for SingletonMap<D>
    where
        D: Serializer,
    {
        type Ok = D::Ok;
        type Error = D::Error;

        type SerializeSeq = D::SerializeSeq;
        type SerializeTuple = D::SerializeTuple;
        type SerializeTupleStruct = D::SerializeTupleStruct;
        type SerializeTupleVariant =
            SerializeTupleVariantAsSingletonMap<D::SerializeMap>;
        type SerializeMap = D::SerializeMap;
        type SerializeStruct = D::SerializeStruct;
        type SerializeStructVariant =
            SerializeStructVariantAsSingletonMap<D::SerializeMap>;

        fn serialize_bool(
            self,
            v: bool,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_bool(v)
        }

        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i8(v)
        }

        fn serialize_i16(
            self,
            v: i16,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i16(v)
        }

        fn serialize_i32(
            self,
            v: i32,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i32(v)
        }

        fn serialize_i64(
            self,
            v: i64,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i64(v)
        }

        fn serialize_i128(
            self,
            v: i128,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i128(v)
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u8(v)
        }

        fn serialize_u16(
            self,
            v: u16,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u16(v)
        }

        fn serialize_u32(
            self,
            v: u32,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u32(v)
        }

        fn serialize_u64(
            self,
            v: u64,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u64(v)
        }

        fn serialize_u128(
            self,
            v: u128,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u128(v)
        }

        fn serialize_f32(
            self,
            v: f32,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_f32(v)
        }

        fn serialize_f64(
            self,
            v: f64,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_f64(v)
        }

        fn serialize_char(
            self,
            v: char,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_char(v)
        }

        fn serialize_str(
            self,
            v: &str,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_str(v)
        }

        fn serialize_bytes(
            self,
            v: &[u8],
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_bytes(v)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_unit()
        }

        fn serialize_unit_struct(
            self,
            name: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_unit_struct(name)
        }

        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_unit_variant(
                name,
                variant_index,
                variant,
            )
        }

        fn serialize_newtype_struct<T>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.delegate.serialize_newtype_struct(name, value)
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_entry(variant, value)?;
            map.end()
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_none()
        }

        fn serialize_some<V>(
            self,
            value: &V,
        ) -> Result<Self::Ok, Self::Error>
        where
            V: ?Sized + Serialize,
        {
            self.delegate
                .serialize_some(&SingletonMap { delegate: value })
        }

        fn serialize_seq(
            self,
            len: Option<usize>,
        ) -> Result<Self::SerializeSeq, Self::Error> {
            self.delegate.serialize_seq(len)
        }

        fn serialize_tuple(
            self,
            len: usize,
        ) -> Result<Self::SerializeTuple, Self::Error> {
            self.delegate.serialize_tuple(len)
        }

        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            self.delegate.serialize_tuple_struct(name, len)
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_key(variant)?;
            let sequence = Sequence::with_capacity(len);
            Ok(SerializeTupleVariantAsSingletonMap { map, sequence })
        }

        fn serialize_map(
            self,
            len: Option<usize>,
        ) -> Result<Self::SerializeMap, Self::Error> {
            self.delegate.serialize_map(len)
        }

        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            self.delegate.serialize_struct(name, len)
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_key(variant)?;
            let mapping = Mapping::with_capacity(len);
            Ok(SerializeStructVariantAsSingletonMap { map, mapping })
        }

        fn collect_str<T>(
            self,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Display,
        {
            self.delegate.collect_str(value)
        }

        fn is_human_readable(&self) -> bool {
            self.delegate.is_human_readable()
        }
    }

    /// A helper struct for serializing tuple variants as a singleton map.
    ///
    /// # Overview
    ///
    /// For a tuple variant like `Enum::TupleVariant(u32, i32)`, this structure
    /// produces a YAML map with a single key (the variant name) and a sequence
    /// of elements as the value.
    ///
    /// # Returns
    ///
    /// Upon completion, the final map entry looks like:
    ///
    /// ```yaml
    /// TupleVariant:
    ///   - ...
    ///   - ...
    /// ```
    ///
    /// # Errors
    ///
    /// Any failure to serialize an element within the tuple variant will cause
    /// an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// enum MyEnum {
    ///     Tuple(u32, i32),
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct SerializeTupleVariantAsSingletonMap<M> {
        /// The underlying map serializer.
        pub map: M,
        /// The sequence of values to serialize.
        pub sequence: Sequence,
    }

    impl<M> SerializeTupleVariant for SerializeTupleVariantAsSingletonMap<M>
    where
        M: SerializeMap,
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T>(
            &mut self,
            field: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = field
                .serialize(crate::value::Serializer)
                .map_err(ser::Error::custom)?;
            self.sequence.push(value);
            Ok(())
        }

        fn end(mut self) -> Result<Self::Ok, Self::Error> {
            self.map.serialize_value(&self.sequence)?;
            self.map.end()
        }
    }

    /// A helper struct for serializing struct variants as a singleton map.
    ///
    /// # Overview
    ///
    /// For a struct variant like `Enum::StructVariant { field1, field2 }`,
    /// this produces a YAML map with one key (the variant name) and a nested
    /// map of field names and their values.
    ///
    /// # Returns
    ///
    /// After serialization, the result looks like:
    ///
    /// ```yaml
    /// StructVariant:
    ///   field1: ...
    ///   field2: ...
    /// ```
    ///
    /// # Errors
    ///
    /// Errors occur if any field cannot be serialized properly or if the
    /// underlying serializer fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// enum MyEnum {
    ///     StructVariant { field1: i32, field2: i32 },
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct SerializeStructVariantAsSingletonMap<M> {
        /// The underlying map serializer.
        pub map: M,
        /// The mapping of field names to their values.
        pub mapping: Mapping,
    }

    impl<M> SerializeStructVariant
        for SerializeStructVariantAsSingletonMap<M>
    where
        M: SerializeMap,
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T>(
            &mut self,
            name: &'static str,
            field: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = field
                .serialize(crate::value::Serializer)
                .map_err(ser::Error::custom)?;
            self.mapping.insert(
                crate::value::Value::String(name.to_owned()),
                value,
            );
            Ok(())
        }

        fn end(mut self) -> Result<Self::Ok, Self::Error> {
            self.map.serialize_value(&self.mapping)?;
            self.map.end()
        }
    }

    // --- The Deserializer implementation enforces the same idea for reading
    // --- single-key maps as enum variants. All internal logic ensures that
    // --- we only ever process exactly one key-value pair for each enum.

    impl<'de, D> Deserializer<'de> for SingletonMap<D>
    where
        D: Deserializer<'de>,
    {
        type Error = D::Error;

        fn deserialize_any<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_any(visitor)
        }

        fn deserialize_bool<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_bool(visitor)
        }

        fn deserialize_i8<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i8(visitor)
        }

        fn deserialize_i16<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i16(visitor)
        }

        fn deserialize_i32<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i32(visitor)
        }

        fn deserialize_i64<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i64(visitor)
        }

        fn deserialize_i128<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i128(visitor)
        }

        fn deserialize_u8<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u8(visitor)
        }

        fn deserialize_u16<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u16(visitor)
        }

        fn deserialize_u32<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u32(visitor)
        }

        fn deserialize_u64<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u64(visitor)
        }

        fn deserialize_u128<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u128(visitor)
        }

        fn deserialize_f32<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_f32(visitor)
        }

        fn deserialize_f64<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_f64(visitor)
        }

        fn deserialize_char<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_char(visitor)
        }

        fn deserialize_str<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_str(visitor)
        }

        fn deserialize_string<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_string(visitor)
        }

        fn deserialize_bytes<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_bytes(visitor)
        }

        fn deserialize_byte_buf<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_byte_buf(visitor)
        }

        fn deserialize_option<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_option(SingletonMapAsEnum {
                name: "",
                delegate: visitor,
            })
        }

        fn deserialize_unit<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_unit(visitor)
        }

        fn deserialize_unit_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_unit_struct(name, visitor)
        }

        fn deserialize_newtype_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_newtype_struct(name, visitor)
        }

        fn deserialize_seq<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_seq(visitor)
        }

        fn deserialize_tuple<V>(
            self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_tuple(len, visitor)
        }

        fn deserialize_tuple_struct<V>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_tuple_struct(name, len, visitor)
        }

        fn deserialize_map<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_map(visitor)
        }

        fn deserialize_struct<V>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_struct(name, fields, visitor)
        }

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_any(SingletonMapAsEnum {
                name,
                delegate: visitor,
            })
        }

        fn deserialize_identifier<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_identifier(visitor)
        }

        fn deserialize_ignored_any<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_ignored_any(visitor)
        }

        fn is_human_readable(&self) -> bool {
            self.delegate.is_human_readable()
        }
    }

    // --- Internal helpers for interpreting singleton maps as enum variants ---
    // --- remain unchanged except for clarifying docstrings.

    struct SingletonMapAsEnum<D> {
        name: &'static str,
        delegate: D,
    }

    impl<'de, V> Visitor<'de> for SingletonMapAsEnum<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn expecting(
            &self,
            formatter: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            self.delegate.expecting(formatter)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_enum(de::value::StrDeserializer::new(v))
        }

        fn visit_borrowed_str<E>(
            self,
            v: &'de str,
        ) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate
                .visit_enum(de::value::BorrowedStrDeserializer::new(v))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate
                .visit_enum(de::value::StringDeserializer::new(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_none()
        }

        fn visit_some<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            self.delegate.visit_some(SingletonMap {
                delegate: deserializer,
            })
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_unit()
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            self.delegate.visit_enum(SingletonMapAsEnum {
                name: self.name,
                delegate: map,
            })
        }
    }

    impl<'de, D> EnumAccess<'de> for SingletonMapAsEnum<D>
    where
        D: MapAccess<'de>,
    {
        type Error = D::Error;
        type Variant = Self;

        fn variant_seed<V>(
            mut self,
            seed: V,
        ) -> Result<(V::Value, Self::Variant), Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            (self.delegate.next_key_seed(seed)?).map_or_else(
                || {
                    Err(de::Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ))
                },
                |value| Ok((value, self)),
            )
        }
    }

    impl<'de, D> VariantAccess<'de> for SingletonMapAsEnum<D>
    where
        D: MapAccess<'de>,
    {
        type Error = D::Error;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Err(de::Error::invalid_type(
                Unexpected::Map,
                &"unit variant",
            ))
        }

        fn newtype_variant_seed<T>(
            mut self,
            seed: T,
        ) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            let value = self.delegate.next_value_seed(seed)?;
            match self.delegate.next_key()? {
                None => Ok(value),
                Some(IgnoredAny) => Err(de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                )),
            }
        }

        fn tuple_variant<V>(
            mut self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let value = self
                .delegate
                .next_value_seed(TupleVariantSeed { len, visitor })?;
            match self.delegate.next_key()? {
                None => Ok(value),
                Some(IgnoredAny) => Err(de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                )),
            }
        }

        fn struct_variant<V>(
            mut self,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let value =
                self.delegate.next_value_seed(StructVariantSeed {
                    name: self.name,
                    fields,
                    visitor,
                })?;
            match self.delegate.next_key()? {
                None => Ok(value),
                Some(IgnoredAny) => Err(de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                )),
            }
        }
    }

    struct TupleVariantSeed<V> {
        len: usize,
        visitor: V,
    }

    impl<'de, V> DeserializeSeed<'de> for TupleVariantSeed<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn deserialize<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_tuple(self.len, self.visitor)
        }
    }

    struct StructVariantSeed<V> {
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    }

    impl<'de, V> DeserializeSeed<'de> for StructVariantSeed<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn deserialize<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_struct(
                self.name,
                self.fields,
                self.visitor,
            )
        }
    }
}

/// Serialize/deserialize an optional enum using a YAML map containing one entry in which
/// the key identifies the variant name.
///
/// # Overview
///
/// This module is similar to `singleton_map` but is designed for `Option<T>`.
/// If the value is `Some(...)`, it is serialized as a singleton map; if it is `None`,
/// it is serialized as `null`.
///
/// # Returns
///
/// When deserializing, a `Some(T)` is returned if a valid singleton map is found,
/// or `None` if the YAML contains `null`.
///
/// # Errors
///
/// This module returns any errors that arise from the underlying
/// `singleton_map` serialization or deserialization, such as structural
/// mismatches or unknown variants.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum MyEnum {
///     Variant1,
///     Variant2(String),
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Example {
///     #[serde(with = "serde_yml::with::singleton_map_optional")]
///     field: Option<MyEnum>,
/// }
///
/// let example = Example {
///     field: Some(MyEnum::Variant2("value".to_string())),
/// };
///
/// let yaml = serde_yml::to_string(&example).unwrap();
/// assert_eq!(yaml, "field:\n  Variant2: value\n");
///
/// let deserialized: Example = serde_yml::from_str(&yaml).unwrap();
/// assert_eq!(example, deserialized);
/// ```
pub mod singleton_map_optional {
    use super::singleton_map;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Serializes an optional value using the singleton map representation.
    ///
    /// # Overview
    ///
    /// - If `value` is `Some`, it is serialized via the `singleton_map` representation.
    /// - If `value` is `None`, `null` is emitted.
    ///
    /// # Returns
    ///
    /// Returns `Ok` if the serialization succeeded, or an error if it failed.
    ///
    /// # Errors
    ///
    /// In addition to I/O or structural errors, serialization can fail if the
    /// underlying `singleton_map::serialize` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use serde_yml::with::singleton_map_optional;
    ///
    /// #[derive(Serialize, Deserialize, Debug, PartialEq)]
    /// enum MyEnum {
    ///     Unit,
    ///     Newtype(u32),
    /// }
    ///
    /// let maybe_value = Some(MyEnum::Newtype(123));
    /// let yaml = serde_yml::to_string(&maybe_value).unwrap();
    /// assert!(yaml.contains("Newtype"));
    /// ```
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        match value {
            Some(v) => singleton_map::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes a value using the `singleton_map` representation.
    ///
    /// # Overview
    ///
    /// - If the YAML is `null`, this function returns `None`.
    /// - Otherwise, it delegates to `singleton_map::deserialize` to parse
    ///   a singleton map into `Some(T)`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(value))` if a valid singleton map was found, `Ok(None)` if
    /// the YAML was `null`, or an error.
    ///
    /// # Errors
    ///
    /// Any error from `singleton_map::deserialize` can occur here, such as:
    /// - A non-map when a map was expected.
    /// - An unknown variant name.
    /// - Malformed YAML input.
    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(singleton_map::SingletonMap {
            delegate: deserializer,
        })
    }
}

/// Serialize/deserialize an enum using a YAML map containing one entry in which
/// the key identifies the variant name, while allowing combination with other `serialize_with` attributes.
///
/// # Overview
///
/// Provides a way to apply the `singleton_map` logic in conjunction with other
/// custom `serialize_with` or `deserialize_with` attributes.
///
/// # Returns
///
/// Ensures that the resulting YAML uses a singleton map for enums, returning the
/// serialized or deserialized result as appropriate.
///
/// # Errors
///
/// Returns errors from the underlying `singleton_map` module if structural or
/// variant name mismatches occur.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum MyEnum {
///     Variant1,
///     Variant2(String),
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Example {
///     #[serde(with = "serde_yml::with::singleton_map_with")]
///     field: MyEnum,
/// }
///
/// let example = Example {
///     field: MyEnum::Variant2("value".to_string()),
/// };
///
/// let yaml = serde_yml::to_string(&example).unwrap();
/// assert_eq!(yaml, "field:\n  Variant2: value\n");
///
/// let deserialized: Example = serde_yml::from_str(&yaml).unwrap();
/// assert_eq!(example, deserialized);
/// ```
#[allow(clippy::module_name_repetitions)]
pub mod singleton_map_with {
    use super::singleton_map;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// # Overview
    ///
    /// Forwards serialization to `singleton_map::serialize`, ensuring the enum
    /// is emitted in a `{ VariantName: ... }` form.
    ///
    /// # Returns
    ///
    /// Returns the `Ok` value of the serialization if successful.
    ///
    /// # Errors
    ///
    /// Any error encountered by `singleton_map::serialize` is propagated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use serde_yml::with::singleton_map_with;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// enum MyEnum {
    ///     A,
    ///     B(u32),
    /// }
    ///
    /// let value = MyEnum::B(123);
    /// let yaml = serde_yml::to_string(&value).unwrap();
    /// assert!(yaml.contains("B"));
    /// ```
    pub fn serialize<T, S>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        singleton_map::serialize(value, serializer)
    }

    /// # Overview
    ///
    /// Forwards deserialization to `singleton_map::deserialize`, recreating
    /// the enum from a singleton map structure.
    ///
    /// # Returns
    ///
    /// Returns `Ok(deserialized_value)` if successful.
    ///
    /// # Errors
    ///
    /// Propagates any error from `singleton_map::deserialize`, for example
    /// incorrect structure or variant name issues.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde::{Serialize, Deserialize};
    /// # use serde_yml::with::singleton_map_with;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize, Deserialize, Debug, PartialEq)]
    /// enum MyEnum {
    ///     A,
    ///     B(u32),
    /// }
    ///
    /// let yaml = "B: 42\n";
    /// let recovered: MyEnum = singleton_map_with::deserialize(
    ///     serde_yml::Deserializer::from_str(yaml)
    /// )?;
    /// assert_eq!(recovered, MyEnum::B(42));
    /// # Ok(())
    /// # }
    /// ```
    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        singleton_map::deserialize(deserializer)
    }
}

/// Apply [`singleton_map`] to *all* enums contained within the data structure.
///
/// # Overview
///
/// This module recursively applies the singleton map approach to any enum, at any
/// nesting level. Enums are thus serialized as single-key maps, even if they
/// are nested inside lists, structs, or other enums.
///
/// # Returns
///
/// The standard Serde `Result` type is returned on serialization and deserialization.
/// If successful, you receive the finalized data structure; otherwise, an error
/// describing the mismatch will be returned.
///
/// # Errors
///
/// - If any nested enum cannot be encoded or decoded correctly, an error occurs.
/// - Structural mismatches or unknown variants also produce errors.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum Enum {
///     Int(i32),
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Inner {
///     a: Enum,
///     bs: Vec<Enum>,
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Outer {
///     tagged_style: Inner,
///
///     #[serde(with = "serde_yml::with::singleton_map_recursive")]
///     singleton_map_style: Inner,
/// }
///
///  let object = Outer {
///      tagged_style: Inner {
///          a: Enum::Int(0),
///          bs: vec![Enum::Int(1)],
///      },
///      singleton_map_style: Inner {
///          a: Enum::Int(2),
///          bs: vec![Enum::Int(3)],
///      },
///  };
///
///  let yaml = serde_yml::to_string(&object).unwrap();
///  print!("{}", yaml);
///
///  let deserialized: Outer = serde_yml::from_str(&yaml).unwrap();
///  assert_eq!(object, deserialized);
/// ```
///
/// The serialized output is:
///
/// ```yaml
/// tagged_style:
///   a: !Int 0
///   bs:
///   - !Int 1
/// singleton_map_style:
///   a:
///     Int: 2
///   bs:
///   - Int: 3
/// ```
///
/// You can also apply this at the top level with
/// `serde_yml::with::singleton_map_recursive::serialize` / `deserialize`.
pub mod singleton_map_recursive {
    use crate::value::{Mapping, Sequence, Value};
    use serde::de::{
        self, Deserialize, DeserializeSeed, Deserializer, EnumAccess,
        IgnoredAny, MapAccess, SeqAccess, Unexpected, VariantAccess,
        Visitor,
    };
    use serde::ser::{
        self, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
        SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant, Serializer,
    };
    use std::fmt::{self, Display};

    /// Serializes all nested enums using the singleton map representation.
    ///
    /// # Overview
    ///
    /// This function inspects all data structures recursively. Wherever it
    /// encounters an enum, it emits a single-key map with the variant name
    /// and variant data. The process repeats for nested enums, ensuring a
    /// consistent representation throughout.
    ///
    /// # Returns
    ///
    /// Returns `Ok(serializer_output)` if successful, or an error describing
    /// the failure.
    ///
    /// # Errors
    ///
    /// Possible errors include:
    /// - I/O or structural errors from the underlying `Serializer`.
    /// - Mismatch between the enum's expected format and the actual data
    ///   structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use serde_yml::with::singleton_map_recursive;
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// enum MyEnum {
    ///     A(i32),
    /// }
    ///
    /// let value = MyEnum::A(42);
    /// let yaml = serde_yml::to_string(&value).unwrap();
    ///
    /// // Top-level usage:
    /// let mut buf = Vec::new();
    /// {
    ///     let mut ser = serde_yml::Serializer::new(&mut buf);
    ///     singleton_map_recursive::serialize(&value, &mut ser).unwrap();
    /// }
    /// let out_str = String::from_utf8(buf).unwrap();
    /// assert!(out_str.contains("A"));
    /// ```
    pub fn serialize<T, S>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        value.serialize(SingletonMapRecursive {
            delegate: serializer,
        })
    }

    /// Deserializes all nested enums from the singleton map representation.
    ///
    /// # Overview
    ///
    /// Reads YAML structures recursively, interpreting any single-key maps as
    /// enum variants. This process is repeated for nested data, ensuring that
    /// all enums remain in the singleton map format.
    ///
    /// # Returns
    ///
    /// Returns the reconstructed data type `T` on success.
    ///
    /// # Errors
    ///
    /// Fails if:
    /// - The data is not a valid singleton map representation for the underlying enums.
    /// - There is an unknown variant or a structural mismatch.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use serde_yml::with::singleton_map_recursive;
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// enum MyEnum {
    ///     A(i32),
    /// }
    ///
    /// let yaml = "A: 42\n";
    /// let result: MyEnum = singleton_map_recursive::deserialize(
    ///     serde_yml::Deserializer::from_str(yaml)
    /// ).unwrap();
    /// assert_eq!(result, MyEnum::A(42));
    /// ```
    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::deserialize(SingletonMapRecursive {
            delegate: deserializer,
        })
    }

    // A wrapper that recursively applies the "singleton map" logic for both
    // serialization and deserialization of nested enums.

    struct SingletonMapRecursive<D> {
        delegate: D,
    }

    impl<D> Serialize for SingletonMapRecursive<D>
    where
        D: Serialize,
    {
        /// # Overview
        ///
        /// Wraps the delegate's `serialize` call to ensure nested enums are
        /// also converted to singleton maps.
        ///
        /// # Returns
        ///
        /// Returns any result that the underlying serializer produces, or an
        /// error if serialization fails.
        ///
        /// # Errors
        ///
        /// Bubble-up from the delegate serializer.
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.delegate.serialize(SingletonMapRecursive {
                delegate: serializer,
            })
        }
    }

    // --- The remainder of this module provides the detailed logic for
    // --- recursing through nested structures and applying the singleton map
    // --- approach to every enum encountered.

    impl<D> Serializer for SingletonMapRecursive<D>
    where
        D: Serializer,
    {
        type Ok = D::Ok;
        type Error = D::Error;

        type SerializeSeq = SingletonMapRecursive<D::SerializeSeq>;
        type SerializeTuple = SingletonMapRecursive<D::SerializeTuple>;
        type SerializeTupleStruct =
            SingletonMapRecursive<D::SerializeTupleStruct>;
        type SerializeTupleVariant =
            SerializeTupleVariantAsSingletonMapRecursive<
                D::SerializeMap,
            >;
        type SerializeMap = SingletonMapRecursive<D::SerializeMap>;
        type SerializeStruct =
            SingletonMapRecursive<D::SerializeStruct>;
        type SerializeStructVariant =
            SerializeStructVariantAsSingletonMapRecursive<
                D::SerializeMap,
            >;

        fn serialize_bool(
            self,
            v: bool,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_bool(v)
        }

        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i8(v)
        }

        fn serialize_i16(
            self,
            v: i16,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i16(v)
        }

        fn serialize_i32(
            self,
            v: i32,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i32(v)
        }

        fn serialize_i64(
            self,
            v: i64,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i64(v)
        }

        fn serialize_i128(
            self,
            v: i128,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_i128(v)
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u8(v)
        }

        fn serialize_u16(
            self,
            v: u16,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u16(v)
        }

        fn serialize_u32(
            self,
            v: u32,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u32(v)
        }

        fn serialize_u64(
            self,
            v: u64,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u64(v)
        }

        fn serialize_u128(
            self,
            v: u128,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_u128(v)
        }

        fn serialize_f32(
            self,
            v: f32,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_f32(v)
        }

        fn serialize_f64(
            self,
            v: f64,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_f64(v)
        }

        fn serialize_char(
            self,
            v: char,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_char(v)
        }

        fn serialize_str(
            self,
            v: &str,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_str(v)
        }

        fn serialize_bytes(
            self,
            v: &[u8],
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_bytes(v)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_unit()
        }

        fn serialize_unit_struct(
            self,
            name: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_unit_struct(name)
        }

        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_unit_variant(
                name,
                variant_index,
                variant,
            )
        }

        fn serialize_newtype_struct<T>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.delegate.serialize_newtype_struct(
                name,
                &SingletonMapRecursive { delegate: value },
            )
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_entry(
                variant,
                &SingletonMapRecursive { delegate: value },
            )?;
            map.end()
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.serialize_none()
        }

        fn serialize_some<V>(
            self,
            value: &V,
        ) -> Result<Self::Ok, Self::Error>
        where
            V: ?Sized + Serialize,
        {
            self.delegate.serialize_some(&SingletonMapRecursive {
                delegate: value,
            })
        }

        fn serialize_seq(
            self,
            len: Option<usize>,
        ) -> Result<Self::SerializeSeq, Self::Error> {
            Ok(SingletonMapRecursive {
                delegate: self.delegate.serialize_seq(len)?,
            })
        }

        fn serialize_tuple(
            self,
            len: usize,
        ) -> Result<Self::SerializeTuple, Self::Error> {
            Ok(SingletonMapRecursive {
                delegate: self.delegate.serialize_tuple(len)?,
            })
        }

        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Ok(SingletonMapRecursive {
                delegate: self
                    .delegate
                    .serialize_tuple_struct(name, len)?,
            })
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_key(variant)?;

            let sequence = Sequence::with_capacity(len);
            Ok(SerializeTupleVariantAsSingletonMapRecursive {
                map,
                sequence,
            })
        }

        fn serialize_map(
            self,
            len: Option<usize>,
        ) -> Result<Self::SerializeMap, Self::Error> {
            Ok(SingletonMapRecursive {
                delegate: self.delegate.serialize_map(len)?,
            })
        }

        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(SingletonMapRecursive {
                delegate: self.delegate.serialize_struct(name, len)?,
            })
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_key(variant)?;
            let mapping = Mapping::with_capacity(len);
            Ok(SerializeStructVariantAsSingletonMapRecursive {
                map,
                mapping,
            })
        }

        fn collect_str<T>(
            self,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Display,
        {
            self.delegate.collect_str(value)
        }

        fn is_human_readable(&self) -> bool {
            self.delegate.is_human_readable()
        }
    }

    impl<D> SerializeSeq for SingletonMapRecursive<D>
    where
        D: SerializeSeq,
    {
        type Ok = D::Ok;
        type Error = D::Error;

        fn serialize_element<T>(
            &mut self,
            elem: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.delegate.serialize_element(&SingletonMapRecursive {
                delegate: elem,
            })
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.end()
        }
    }

    impl<D> SerializeTuple for SingletonMapRecursive<D>
    where
        D: SerializeTuple,
    {
        type Ok = D::Ok;
        type Error = D::Error;

        fn serialize_element<T>(
            &mut self,
            elem: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.delegate.serialize_element(&SingletonMapRecursive {
                delegate: elem,
            })
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.end()
        }
    }

    impl<D> SerializeTupleStruct for SingletonMapRecursive<D>
    where
        D: SerializeTupleStruct,
    {
        type Ok = D::Ok;
        type Error = D::Error;

        fn serialize_field<V>(
            &mut self,
            value: &V,
        ) -> Result<(), Self::Error>
        where
            V: ?Sized + Serialize,
        {
            self.delegate.serialize_field(&SingletonMapRecursive {
                delegate: value,
            })
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.end()
        }
    }

    struct SerializeTupleVariantAsSingletonMapRecursive<M> {
        map: M,
        sequence: Sequence,
    }

    impl<M> SerializeTupleVariant
        for SerializeTupleVariantAsSingletonMapRecursive<M>
    where
        M: SerializeMap,
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T>(
            &mut self,
            field: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = field
                .serialize(SingletonMapRecursive {
                    delegate: crate::value::Serializer,
                })
                .map_err(ser::Error::custom)?;
            self.sequence.push(value);
            Ok(())
        }

        fn end(mut self) -> Result<Self::Ok, Self::Error> {
            self.map.serialize_value(&self.sequence)?;
            self.map.end()
        }
    }

    impl<D> SerializeMap for SingletonMapRecursive<D>
    where
        D: SerializeMap,
    {
        type Ok = D::Ok;
        type Error = D::Error;

        fn serialize_key<T>(
            &mut self,
            key: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.delegate
                .serialize_key(&SingletonMapRecursive { delegate: key })
        }

        fn serialize_value<T>(
            &mut self,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.delegate.serialize_value(&SingletonMapRecursive {
                delegate: value,
            })
        }

        fn serialize_entry<K, V>(
            &mut self,
            key: &K,
            value: &V,
        ) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            self.delegate.serialize_entry(
                &SingletonMapRecursive { delegate: key },
                &SingletonMapRecursive { delegate: value },
            )
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.end()
        }
    }

    impl<D> SerializeStruct for SingletonMapRecursive<D>
    where
        D: SerializeStruct,
    {
        type Ok = D::Ok;
        type Error = D::Error;

        fn serialize_field<V>(
            &mut self,
            key: &'static str,
            value: &V,
        ) -> Result<(), Self::Error>
        where
            V: ?Sized + Serialize,
        {
            self.delegate.serialize_field(
                key,
                &SingletonMapRecursive { delegate: value },
            )
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.delegate.end()
        }
    }

    struct SerializeStructVariantAsSingletonMapRecursive<M> {
        map: M,
        mapping: Mapping,
    }

    impl<M> SerializeStructVariant
        for SerializeStructVariantAsSingletonMapRecursive<M>
    where
        M: SerializeMap,
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T>(
            &mut self,
            name: &'static str,
            field: &T,
        ) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = field
                .serialize(SingletonMapRecursive {
                    delegate: crate::value::Serializer,
                })
                .map_err(ser::Error::custom)?;
            self.mapping.insert(Value::String(name.to_owned()), value);
            Ok(())
        }

        fn end(mut self) -> Result<Self::Ok, Self::Error> {
            self.map.serialize_value(&self.mapping)?;
            self.map.end()
        }
    }

    impl<'de, D> Deserializer<'de> for SingletonMapRecursive<D>
    where
        D: Deserializer<'de>,
    {
        type Error = D::Error;

        fn deserialize_any<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_any(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_bool<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_bool(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_i8<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i8(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_i16<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i16(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_i32<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i32(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_i64<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i64(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_i128<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_i128(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_u8<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u8(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_u16<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u16(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_u32<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u32(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_u64<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u64(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_u128<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_u128(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_f32<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_f32(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_f64<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_f64(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_char<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_char(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_str<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_str(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_string<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_string(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_bytes<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_bytes(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_byte_buf<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_byte_buf(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_option<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_option(
                SingletonMapRecursiveAsEnum {
                    name: "",
                    delegate: visitor,
                },
            )
        }

        fn deserialize_unit<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_unit(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_unit_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_unit_struct(
                name,
                SingletonMapRecursive { delegate: visitor },
            )
        }

        fn deserialize_newtype_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_newtype_struct(
                name,
                SingletonMapRecursive { delegate: visitor },
            )
        }

        fn deserialize_seq<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_seq(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_tuple<V>(
            self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_tuple(
                len,
                SingletonMapRecursive { delegate: visitor },
            )
        }

        fn deserialize_tuple_struct<V>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_tuple_struct(
                name,
                len,
                SingletonMapRecursive { delegate: visitor },
            )
        }

        fn deserialize_map<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_map(SingletonMapRecursive {
                delegate: visitor,
            })
        }

        fn deserialize_struct<V>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_struct(
                name,
                fields,
                SingletonMapRecursive { delegate: visitor },
            )
        }

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_any(SingletonMapRecursiveAsEnum {
                name,
                delegate: visitor,
            })
        }

        fn deserialize_identifier<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_identifier(
                SingletonMapRecursive { delegate: visitor },
            )
        }

        fn deserialize_ignored_any<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.delegate.deserialize_ignored_any(
                SingletonMapRecursive { delegate: visitor },
            )
        }

        fn is_human_readable(&self) -> bool {
            self.delegate.is_human_readable()
        }
    }

    impl<'de, V> Visitor<'de> for SingletonMapRecursive<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn expecting(
            &self,
            formatter: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            self.delegate.expecting(formatter)
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_bool(v)
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_i8(v)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_i16(v)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_i32(v)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_i64(v)
        }

        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_i128(v)
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_u8(v)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_u16(v)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_u32(v)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_u64(v)
        }

        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_u128(v)
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_f32(v)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_f64(v)
        }

        fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_char(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_str(v)
        }

        fn visit_borrowed_str<E>(
            self,
            v: &'de str,
        ) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_borrowed_str(v)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_string(v)
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_bytes(v)
        }

        fn visit_borrowed_bytes<E>(
            self,
            v: &'de [u8],
        ) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_borrowed_bytes(v)
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_byte_buf(v)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_none()
        }

        fn visit_some<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            self.delegate.visit_some(SingletonMapRecursive {
                delegate: deserializer,
            })
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_unit()
        }

        fn visit_newtype_struct<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            self.delegate.visit_newtype_struct(SingletonMapRecursive {
                delegate: deserializer,
            })
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            self.delegate
                .visit_seq(SingletonMapRecursive { delegate: seq })
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            self.delegate
                .visit_map(SingletonMapRecursive { delegate: map })
        }
    }

    impl<'de, T> DeserializeSeed<'de> for SingletonMapRecursive<T>
    where
        T: DeserializeSeed<'de>,
    {
        type Value = T::Value;

        fn deserialize<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            self.delegate.deserialize(SingletonMapRecursive {
                delegate: deserializer,
            })
        }
    }

    impl<'de, S> SeqAccess<'de> for SingletonMapRecursive<S>
    where
        S: SeqAccess<'de>,
    {
        type Error = S::Error;

        fn next_element_seed<T>(
            &mut self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            self.delegate.next_element_seed(SingletonMapRecursive {
                delegate: seed,
            })
        }
    }

    impl<'de, M> MapAccess<'de> for SingletonMapRecursive<M>
    where
        M: MapAccess<'de>,
    {
        type Error = M::Error;

        fn next_key_seed<K>(
            &mut self,
            seed: K,
        ) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            self.delegate
                .next_key_seed(SingletonMapRecursive { delegate: seed })
        }

        fn next_value_seed<V>(
            &mut self,
            seed: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            self.delegate.next_value_seed(SingletonMapRecursive {
                delegate: seed,
            })
        }
    }

    struct SingletonMapRecursiveAsEnum<D> {
        name: &'static str,
        delegate: D,
    }

    impl<'de, V> Visitor<'de> for SingletonMapRecursiveAsEnum<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn expecting(
            &self,
            formatter: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            self.delegate.expecting(formatter)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_enum(de::value::StrDeserializer::new(v))
        }

        fn visit_borrowed_str<E>(
            self,
            v: &'de str,
        ) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate
                .visit_enum(de::value::BorrowedStrDeserializer::new(v))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate
                .visit_enum(de::value::StringDeserializer::new(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_none()
        }

        fn visit_some<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            self.delegate.visit_some(SingletonMapRecursive {
                delegate: deserializer,
            })
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.delegate.visit_unit()
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            self.delegate.visit_enum(SingletonMapRecursiveAsEnum {
                name: self.name,
                delegate: map,
            })
        }
    }

    impl<'de, D> EnumAccess<'de> for SingletonMapRecursiveAsEnum<D>
    where
        D: MapAccess<'de>,
    {
        type Error = D::Error;
        type Variant = Self;

        fn variant_seed<V>(
            mut self,
            seed: V,
        ) -> Result<(V::Value, Self::Variant), Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            (self.delegate.next_key_seed(seed)?).map_or_else(
                || {
                    Err(de::Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ))
                },
                |value| Ok((value, self)),
            )
        }
    }

    impl<'de, D> VariantAccess<'de> for SingletonMapRecursiveAsEnum<D>
    where
        D: MapAccess<'de>,
    {
        type Error = D::Error;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Err(de::Error::invalid_type(
                Unexpected::Map,
                &"unit variant",
            ))
        }

        fn newtype_variant_seed<T>(
            mut self,
            seed: T,
        ) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            let value = self.delegate.next_value_seed(
                SingletonMapRecursive { delegate: seed },
            )?;
            match self.delegate.next_key()? {
                None => Ok(value),
                Some(IgnoredAny) => Err(de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                )),
            }
        }

        fn tuple_variant<V>(
            mut self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let value =
                self.delegate.next_value_seed(TupleVariantSeed {
                    len,
                    visitor: SingletonMapRecursive {
                        delegate: visitor,
                    },
                })?;
            match self.delegate.next_key()? {
                None => Ok(value),
                Some(IgnoredAny) => Err(de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                )),
            }
        }

        fn struct_variant<V>(
            mut self,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let value =
                self.delegate.next_value_seed(StructVariantSeed {
                    name: self.name,
                    fields,
                    visitor: SingletonMapRecursive {
                        delegate: visitor,
                    },
                })?;
            match self.delegate.next_key()? {
                None => Ok(value),
                Some(IgnoredAny) => Err(de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                )),
            }
        }
    }

    struct TupleVariantSeed<V> {
        len: usize,
        visitor: V,
    }

    impl<'de, V> DeserializeSeed<'de> for TupleVariantSeed<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn deserialize<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_tuple(self.len, self.visitor)
        }
    }

    struct StructVariantSeed<V> {
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    }

    impl<'de, V> DeserializeSeed<'de> for StructVariantSeed<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn deserialize<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_struct(
                self.name,
                self.fields,
                self.visitor,
            )
        }
    }
}

/// Serialize/deserialize nested enums using a YAML map containing one entry in which
/// the key identifies the variant name.
///
/// # Overview
///
/// This module is nearly identical to `singleton_map`, except it applies the
/// singleton map layout recursively to any nested enums. When an enum contains
/// other enums, all are represented in a consistent, single-key map style.
///
/// # Returns
///
/// On success, returns `Ok(T)` during deserialization, or the serialized YAML
/// data structure during serialization.
///
/// # Errors
///
/// Errors arise if the input does not match the expected nested singleton map
/// format or if an invalid variant is encountered in nested data.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum InnerEnum {
///     Variant1,
///     Variant2(String),
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// enum OuterEnum {
///     Variant1(InnerEnum),
///     Variant2 { inner: InnerEnum },
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Example {
///     #[serde(with = "serde_yml::with::nested_singleton_map")]
///     field: OuterEnum,
/// }
///
/// let example = Example {
///     field: OuterEnum::Variant2 {
///         inner: InnerEnum::Variant2("value".to_string()),
///     },
/// };
///
/// let yaml = serde_yml::to_string(&example).unwrap();
/// assert_eq!(yaml, "field:\n  Variant2:\n    inner:\n      Variant2: value\n");
///
/// let deserialized: Example = serde_yml::from_str(&yaml).unwrap();
/// assert_eq!(example, deserialized);
/// ```
pub mod nested_singleton_map {
    use super::singleton_map_recursive;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Serializes a value using the nested singleton map representation.
    ///
    /// # Overview
    ///
    /// Any enum encountered within the data structure is converted into a
    /// single-key map, where the key is the variant name. This transformation
    /// happens recursively, so even nested enums will follow the same format.
    ///
    /// # Returns
    ///
    /// `Ok` if serialization succeeds, or an error if it fails.
    ///
    /// # Errors
    ///
    /// This function returns errors from the underlying
    /// `singleton_map_recursive::serialize` if data cannot be serialized
    /// or does not fit the expected structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde::{Serialize, Deserialize};
    /// use serde_yml::with::nested_singleton_map;
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// enum InnerEnum {
    ///     Variant1,
    ///     Variant2(String),
    /// }
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// enum OuterEnum {
    ///     Variant1(InnerEnum),
    ///     Variant2 { inner: InnerEnum },
    /// }
    ///
    /// let value = OuterEnum::Variant2 {
    ///     inner: InnerEnum::Variant2("value".to_string()),
    /// };
    ///
    /// let yaml = serde_yml::to_string(&value).unwrap();
    /// assert!(yaml.contains("Variant2"));
    /// ```
    pub fn serialize<T, S>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        singleton_map_recursive::serialize(value, serializer)
    }

    /// Deserializes a value using the nested singleton map representation.
    ///
    /// # Overview
    ///
    /// Expects a recursively nested singleton map structure, where each enum
    /// is represented as a single-key map. Attempts to parse all nested enums
    /// accordingly.
    ///
    /// # Returns
    ///
    /// On success, returns an instance of the type `T`.
    ///
    /// # Errors
    ///
    /// - If the structure does not match the nested singleton map pattern,
    ///   deserialization fails.
    /// - Unknown enum variants or I/O errors cause deserialization to fail.
    ///
    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        singleton_map_recursive::deserialize(deserializer)
    }
}
