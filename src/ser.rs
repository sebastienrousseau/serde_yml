//! YAML Serialization – This module provides YAML serialization via the `Serializer` type.

use crate::libyml;
use crate::libyml::emitter::{
    Emitter, Event, Mapping, Scalar, ScalarStyle, Sequence,
};
use crate::{
    modules::error::{self, Error, ErrorImpl},
    value::tagged::{self, MaybeTag},
};
use serde::{
    de::Visitor,
    ser::{self, Serializer as _},
};
use std::{
    fmt::{self, Display},
    io,
    marker::PhantomData,
    mem, num, str,
};

/// The result type returned by most serialization functions in this module.
///
/// # Overview
/// We alias `std::result::Result<T, Error>` to reduce verbosity. Any YAML
/// serialization call may yield an [`Error`], describing I/O issues or
/// structural mismatches.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A YAML serializer that implements [`serde::ser::Serializer`].
///
/// # Overview
/// This struct wraps an internal [`Emitter`], writing the YAML events
/// (like `MappingStart`, `SequenceStart`, etc.) into an `io::Write`
/// target. The [`SerializerConfig`] determines whether to tag unit
/// variants.
///
/// # Returns
/// Each `serialize_XXX` method returns a [`Result`] indicating success
/// or a YAML/IO error.
///
/// # Errors
/// - Writing invalid YAML events or encountering nested unit variant tags
///   triggers an error (`ErrorImpl::SerializeNestedEnum`).
/// - I/O failures writing to the underlying stream cause errors.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// use serde_yml::ser::{Serializer, SerializerConfig};
/// use std::collections::HashMap;
///
/// fn main() -> serde_yml::Result<()> {
///     let mut buffer = Vec::new();
///     let config = SerializerConfig { tag_unit_variants: true };
///     let mut ser = Serializer::new_with_config(&mut buffer, config);
///
///     let mut object = HashMap::new();
///     object.insert("hello", "world");
///
///     // Serialize a map into YAML
///     object.serialize(&mut ser)?;
///
///     // Print the final output
///     println!("{}", String::from_utf8_lossy(&buffer));
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Serializer<W> {
    /// Configuration for how certain variants are emitted.
    pub config: SerializerConfig,
    /// Recursion depth for the current serialization process.
    pub depth: usize,
    /// Tracks the current tagging state or in-progress mapping state.
    pub state: State,
    /// The internal YAML emitter handling event emission.
    pub emitter: Emitter<'static>,
    /// Marker to ensure the type `W: io::Write` is not dropped prematurely.
    pub writer: PhantomData<W>,
}

/// Configuration affecting how the [`Serializer`] emits YAML.
///
/// # Overview
/// Currently, we only have a single toggle, `tag_unit_variants`.
/// This affects whether unit variants (e.g., `MyEnum::Unit`) appear
/// as `Unit` or `!Unit` in the output.
///
/// # Examples
/// ```
/// use serde_yml::ser::{Serializer, SerializerConfig};
///
/// let config = SerializerConfig {
///     tag_unit_variants: true,
/// };
/// // Then pass `config` to `Serializer::new_with_config`...
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct SerializerConfig {
    /// When `true`, unit variants become YAML tags (`!Unit`).
    /// When `false`, they remain as plain strings (`Unit`).
    pub tag_unit_variants: bool,
}

/// Tracks the current state of the [`Serializer`].
///
/// # Overview
/// Certain YAML constructs (e.g., tag-based enums) require a multi-step
/// approach. `State` indicates whether we are about to start a mapping,
/// have encountered a tag, or have completed it.
///
/// - [`State::CheckForTag`]: We anticipate a single-key map for an enum variant.
/// - [`State::FoundTag(String)`]: The variant name is recognized, but not yet fully emitted.
///
/// # Errors
/// If a nested or duplicate tag is encountered incorrectly, an error
/// may be triggered.
///
/// # Examples
/// Not typically interacted with directly by library users, but
/// helps in advanced debugging if custom events are used.
#[derive(Debug)]
pub enum State {
    /// No special handling is in progress.
    NothingInParticular,
    /// Next step is to check if a single-key map is an enum variant tag.
    CheckForTag,
    /// Next step is to see if we encountered a repeated or nested enum tag.
    CheckForDuplicateTag,
    /// A tag was found (unit variant name). We store it here.
    FoundTag(String),
    /// A tag was already applied (avoid duplicates).
    AlreadyTagged,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    // ----------------------------------------------------------------------------
    // CONSTRUCTORS
    // ----------------------------------------------------------------------------

    /// Creates a new YAML serializer with default [`SerializerConfig`].
    ///
    /// # Overview
    /// Wraps a writer in an [`Emitter`], beginning a YAML `StreamStart`.
    ///
    /// # Returns
    /// A fresh [`Serializer`] ready to process Serde data.
    ///
    /// # Errors
    /// While unlikely during creation, I/O might fail if the emitter
    /// cannot write the initial `StreamStart`.
    ///
    /// # Examples
    /// ```
    /// use serde::{Serialize, Serializer as _};
    /// use serde_yml::ser::{Serializer, SerializerConfig, to_string};
    ///
    /// #[derive(Serialize)]
    /// struct MyData {
    ///     number: i32,
    /// }
    ///
    /// fn main() -> serde_yml::Result<()> {
    ///     // Create an in-memory buffer
    ///     let buffer = Vec::new();
    ///
    ///     // Construct a serializer with the default config
    ///     let mut serializer = Serializer::new(buffer);
    ///
    ///     // Create some data to serialize
    ///     let data = MyData { number: 42 };
    ///
    ///     // Serialize it into the YAML serializer
    ///     data.serialize(&mut serializer)?;
    ///
    ///     // Convert the resulting buffer to a string for display
    ///     let yaml_str = to_string(&data)?;
    ///     println!("Serialized YAML:\n{}", yaml_str);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(writer: W) -> Self {
        Self::new_with_config(writer, SerializerConfig::default())
    }

    /// Creates a new YAML serializer with a custom [`SerializerConfig`].
    ///
    /// # Overview
    /// Similar to [`Serializer::new`], but you can enable or disable certain
    /// behaviors (like tagging unit variants).
    ///
    /// # Returns
    /// A configured [`Serializer`] instance.
    ///
    /// # Errors
    /// Same as [`Serializer::new`], I/O errors can occur when initializing the
    /// emitter.
    ///
    /// # Examples
    /// ```
    /// use serde::{Serialize, Serializer as _};
    /// use serde_yml::ser::{Serializer, SerializerConfig, to_string};
    ///
    /// #[derive(Serialize)]
    /// struct MyData {
    ///     message: String,
    ///     value: i32,
    /// }
    ///
    /// fn main() -> serde_yml::Result<()> {
    ///     // Create an in-memory buffer
    ///     let buffer = Vec::new();
    ///
    ///     // Configure serializer to tag unit variants as `!VariantName`
    ///     let config = SerializerConfig {
    ///         tag_unit_variants: true,
    ///     };
    ///
    ///     // Build the serializer with the custom configuration
    ///     let mut serializer = Serializer::new_with_config(buffer, config);
    ///
    ///     // Prepare some data to serialize
    ///     let data = MyData {
    ///         message: "Hello, world!".to_owned(),
    ///         value: 42,
    ///     };
    ///
    ///     // Serialize it into the YAML serializer
    ///     data.serialize(&mut serializer)?;
    ///
    ///     // Convert the resulting buffer to a YAML string
    ///     let yaml_str = to_string(&data)?;
    ///     println!("Serialized YAML:\n{}", yaml_str);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new_with_config(
        writer: W,
        config: SerializerConfig,
    ) -> Self {
        let mut emitter = Emitter::new({
            let writer = Box::new(writer);
            // Safety note: Transmute is used to unify `Box<dyn io::Write>` with
            // a `'static` lifetime for the emitter. This is carefully managed so
            // that `writer` is never dropped prematurely.
            unsafe {
                mem::transmute::<Box<dyn io::Write>, Box<dyn io::Write>>(
                    writer,
                )
            }
        });

        // Emit the start of a YAML stream immediately.
        emitter
            .emit(Event::StreamStart)
            .expect("Failed to write StreamStart");

        Serializer {
            config,
            depth: 0,
            state: State::NothingInParticular,
            emitter,
            writer: PhantomData,
        }
    }

    // ----------------------------------------------------------------------------
    // PUBLIC METHODS
    // ----------------------------------------------------------------------------

    /// Flushes any buffered events to the underlying `io::Write`.
    ///
    /// # Overview
    /// Ensures that all pending YAML events have been physically written.
    ///
    /// # Returns
    /// `Ok(())` on success, or an [`Error`] if the emitter fails.
    ///
    /// # Examples
    /// ```
    /// use serde::Serialize;
    /// use serde_yml::ser::{Serializer, to_string};
    ///
    /// #[derive(Serialize)]
    /// struct MyData {
    ///     name: String,
    ///     count: usize,
    /// }
    ///
    /// fn main() -> serde_yml::Result<()> {
    ///     let mut buffer = Vec::new();
    ///     let mut ser = Serializer::new(&mut buffer);
    ///
    ///     // Create some data to serialize
    ///     let data = MyData {
    ///         name: "example".into(),
    ///         count: 42,
    ///     };
    ///
    ///     // Serialize the data into YAML
    ///     data.serialize(&mut ser)?;
    ///
    ///     // Flush ensures everything is written out
    ///     ser.flush()?;
    ///
    ///     // Now you can inspect the YAML output in `buffer`
    ///     println!("YAML so far:\n{}", String::from_utf8_lossy(&buffer));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn flush(&mut self) -> Result<()> {
        self.emitter.flush()?;
        Ok(())
    }

    /// Consumes `self`, finalizing the YAML stream and returning
    /// the underlying writer `W`.
    ///
    /// # Overview
    /// After writing `StreamEnd`, this function flushes remaining events and
    /// returns the `io::Write` so you can continue using it. Any future usage
    /// of this `Serializer` is invalid.
    ///
    /// # Returns
    /// The original writer `W` on success.
    ///
    /// # Errors
    /// - Fails if `StreamEnd` or final flush cannot be emitted successfully.
    /// - Future usage of `Serializer` after this call is undefined.
    ///
    /// # Examples
    /// ```rust
    /// use serde::Serialize;
    /// use serde_yml::ser::{Serializer, to_string};
    ///
    /// #[derive(Serialize)]
    /// struct MyData {
    ///     key: String,
    ///     value: i32,
    /// }
    ///
    /// fn main() -> serde_yml::Result<()> {
    ///     // Create an in-memory buffer for YAML output
    ///     let buffer = Vec::new();
    ///
    ///     // Initialize the YAML serializer
    ///     let mut ser = Serializer::new(buffer);
    ///
    ///     // Serialize some data
    ///     let data = MyData {
    ///         key: "example".into(),
    ///         value: 123,
    ///     };
    ///     data.serialize(&mut ser)?;
    ///
    ///     // Finalize the YAML stream and retrieve the writer
    ///     let buffer = ser.into_inner()?;
    ///
    ///     // Now `ser` is invalid; we can use `buffer` directly
    ///     println!("Final YAML output:\n{}", String::from_utf8_lossy(&buffer));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn into_inner(mut self) -> Result<W> {
        self.emitter.emit(Event::StreamEnd)?;
        self.emitter.flush()?;
        let writer = self.emitter.into_inner();
        Ok(*unsafe { Box::from_raw(Box::into_raw(writer).cast::<W>()) })
    }

    // ----------------------------------------------------------------------------
    // LOW-LEVEL EMISSION HELPERS
    // ----------------------------------------------------------------------------

    /// Emit a single YAML scalar value, optionally applying any stored tag.
    ///
    /// # Overview
    /// Typically used internally by numeric/string `serialize_XXX` calls.
    ///
    /// # Returns
    /// `Ok(())` on success.
    ///
    /// # Errors
    /// - Emits an error if there is a nested tag mismatch or an I/O failure.
    ///
    /// # Examples
    /// ```rust
    /// # use serde_yml::ser::{Serializer, SerializerConfig};
    /// # use serde_yml::libyml::emitter::{Scalar, ScalarStyle, Event};
    /// # use std::io;
    ///
    /// // Create an in-memory buffer for YAML output
    /// let mut buffer = Vec::new();
    ///
    /// // Initialize the YAML serializer with default configuration
    /// let mut ser = Serializer::new_with_config(&mut buffer, SerializerConfig::default());
    ///
    /// // Emit a scalar value "true" with a plain style
    /// ser.emit_scalar(Scalar {
    ///     tag: None,
    ///     value: "true",
    ///     style: ScalarStyle::Plain,
    /// })?;
    ///
    /// // Flush any buffered events
    /// ser.flush()?;
    ///
    /// // Convert buffer to UTF-8 string and print
    /// println!("{}", String::from_utf8_lossy(&buffer));
    ///
    /// # Ok::<(), serde_yml::Error>(())
    /// ```
    pub fn emit_scalar(
        &mut self,
        mut scalar: Scalar<'_>,
    ) -> Result<()> {
        self.flush_mapping_start()?;
        if let Some(tag) = self.take_tag() {
            scalar.tag = Some(tag);
        }
        self.value_start()?;
        self.emitter.emit(Event::Scalar(scalar))?;
        self.value_end()
    }

    /// Begins a YAML sequence by emitting `SequenceStart`.
    ///
    /// # Overview
    /// Called by tuple or slice serialization logic. If there’s a queued tag,
    /// it is applied here.
    ///
    /// # Returns
    /// `Ok(())` if emission succeeds.
    ///
    /// # Errors
    /// - Errors if I/O fails.
    /// - Tagging conflicts may also raise errors (rare).
    pub fn emit_sequence_start(&mut self) -> Result<()> {
        self.flush_mapping_start()?;
        self.value_start()?;
        let tag = self.take_tag();
        self.emitter.emit(Event::SequenceStart(Sequence { tag }))?;
        Ok(())
    }

    /// Ends a YAML sequence with `SequenceEnd`.
    ///
    /// # Overview
    /// Typically used at the end of list/tuple serialization.
    ///
    /// # Errors
    /// - I/O or structural errors if something is out of place.
    pub fn emit_sequence_end(&mut self) -> Result<()> {
        self.emitter.emit(Event::SequenceEnd)?;
        self.value_end()
    }

    /// Begins a YAML mapping with `MappingStart`.
    ///
    /// # Overview
    /// Often called when serializing a struct or map. If we have a pending tag
    /// for an enum variant, it is applied here.
    ///
    /// # Errors
    /// - If nested tags are encountered.
    pub fn emit_mapping_start(&mut self) -> Result<()> {
        self.flush_mapping_start()?;
        self.value_start()?;
        let tag = self.take_tag();
        self.emitter.emit(Event::MappingStart(Mapping { tag }))?;
        Ok(())
    }

    /// Ends a YAML mapping with `MappingEnd`.
    ///
    /// # Overview
    /// Typically used at the end of struct or map serialization.
    ///
    /// # Errors
    /// - If the `MappingEnd` is out of place or I/O fails.
    pub fn emit_mapping_end(&mut self) -> Result<()> {
        self.emitter.emit(Event::MappingEnd)?;
        self.value_end()
    }

    /// Emits a "value start," used to potentially begin a new YAML document
    /// if `depth == 0`.
    ///
    /// # Overview
    /// - If `depth` is zero, emits `DocumentStart`.
    /// - Increments `depth`.
    ///
    /// # Errors
    /// - I/O or event-based errors if out of place.
    pub fn value_start(&mut self) -> Result<()> {
        if self.depth == 0 {
            self.emitter.emit(Event::DocumentStart)?;
        }
        self.depth += 1;
        Ok(())
    }

    /// Emits a "value end," finishing the current YAML element.
    ///
    /// # Overview
    /// - Decrements `depth`.
    /// - If `depth` hits zero, emits `DocumentEnd`.
    ///
    /// # Errors
    /// - I/O or event-based errors if out of place.
    pub fn value_end(&mut self) -> Result<()> {
        self.depth = self.depth.saturating_sub(1);
        if self.depth == 0 {
            self.emitter.emit(Event::DocumentEnd)?;
        }
        Ok(())
    }

    /// Returns any stored tag, converting it to a YAML `!Tag`.
    ///
    /// # Overview
    /// If [`State::FoundTag`] is active, we prefix with `!` unless
    /// already present. Resets the internal state to no tagging.
    ///
    /// # Errors
    /// - None directly. Called by other methods (e.g., `emit_scalar`).
    pub fn take_tag(&mut self) -> Option<String> {
        let state =
            mem::replace(&mut self.state, State::NothingInParticular);
        if let State::FoundTag(mut tag) = state {
            if !tag.starts_with('!') {
                tag.insert(0, '!');
            }
            Some(tag)
        } else {
            self.state = state;
            None
        }
    }

    /// If necessary, flushes the start of a mapping if we’re in certain states.
    ///
    /// # Overview
    /// Called before writing a map’s key-value pair to ensure
    /// `[State::CheckForTag]` or `[State::CheckForDuplicateTag]` is handled.
    ///
    /// # Errors
    /// - If `emit_mapping_start()` fails.
    pub fn flush_mapping_start(&mut self) -> Result<()> {
        match self.state {
            State::CheckForTag => {
                self.state = State::NothingInParticular;
                self.emit_mapping_start()?;
            }
            State::CheckForDuplicateTag => {
                self.state = State::NothingInParticular;
            }
            _ => {}
        }
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// SERDE’S SERIALIZER IMPLEMENTATION
// ----------------------------------------------------------------------------

impl<W> ser::Serializer for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // -- BASIC DATA TYPES --

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: if v { "true" } else { "false" },
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        let mut buffer = ryu::Buffer::new();
        self.emit_scalar(Scalar {
            tag: None,
            value: match v.classify() {
                num::FpCategory::Infinite if v.is_sign_positive() => {
                    ".inf"
                }
                num::FpCategory::Infinite => "-.inf",
                num::FpCategory::Nan => ".nan",
                _ => buffer.format_finite(v),
            },
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        let mut buffer = ryu::Buffer::new();
        self.emit_scalar(Scalar {
            tag: None,
            value: match v.classify() {
                num::FpCategory::Infinite if v.is_sign_positive() => {
                    ".inf"
                }
                num::FpCategory::Infinite => "-.inf",
                num::FpCategory::Nan => ".nan",
                _ => buffer.format_finite(v),
            },
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: value.encode_utf8(&mut [0u8; 4]),
            style: ScalarStyle::SingleQuoted,
        })
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        // This nested visitor approach mimics how we handle ambiguous strings
        // in `crate::de`. If the string looks like a YAML boolean, we single-quote it.
        struct InferScalarStyle;

        impl Visitor<'_> for InferScalarStyle {
            type Value = ScalarStyle;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str("scalar style inference")
            }

            fn visit_bool<E>(self, _v: bool) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_i64<E>(self, _v: i64) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_i128<E>(self, _v: i128) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_u128<E>(self, _v: u128) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_f64<E>(self, _v: f64) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                if crate::de::ambiguous_string(v) {
                    Ok(ScalarStyle::SingleQuoted)
                } else {
                    Ok(ScalarStyle::Any)
                }
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }
        }

        let style = match value {
            // Certain keywords that can be misread as booleans or something else
            "y" | "Y" | "yes" | "Yes" | "YES" | "n" | "N" | "no"
            | "No" | "NO" | "true" | "True" | "TRUE" | "false"
            | "False" | "FALSE" | "on" | "On" | "ON" | "off"
            | "Off" | "OFF" => ScalarStyle::SingleQuoted,
            _ if value.contains('\n') => ScalarStyle::Literal,
            _ => {
                let result = crate::de::visit_untagged_scalar(
                    InferScalarStyle,
                    value,
                    None,
                    libyml::parser::ScalarStyle::Plain,
                );
                result.unwrap_or(ScalarStyle::Any)
            }
        };

        self.emit_scalar(Scalar {
            tag: None,
            value,
            style,
        })
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(error::new(ErrorImpl::BytesUnsupported))
    }

    fn serialize_unit(self) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: "null",
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        if !self.config.tag_unit_variants {
            self.serialize_str(variant)
        } else {
            if let State::FoundTag(_) = self.state {
                return Err(error::new(ErrorImpl::SerializeNestedEnum));
            }
            self.state = State::FoundTag(variant.to_owned());
            self.emit_scalar(Scalar {
                tag: None,
                value: "",
                style: ScalarStyle::Plain,
            })
        }
    }

    // -- NEWTYPE --

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        if let State::FoundTag(_) = self.state {
            return Err(error::new(ErrorImpl::SerializeNestedEnum));
        }
        self.state = State::FoundTag(variant.to_owned());
        value.serialize(&mut *self)
    }

    // -- OPTION TYPES --

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<V>(self, value: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    // -- COLLECTIONS --

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq> {
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple> {
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _enm: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        if let State::FoundTag(_) = self.state {
            return Err(error::new(ErrorImpl::SerializeNestedEnum));
        }
        self.state = State::FoundTag(variant.to_owned());
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        if len == Some(1) {
            // Single entry map might be an enum variant: check or duplicate
            self.state = if let State::FoundTag(_) = self.state {
                self.emit_mapping_start()?;
                State::CheckForDuplicateTag
            } else {
                State::CheckForTag
            };
        } else {
            self.emit_mapping_start()?;
        }
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.emit_mapping_start()?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _enm: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        if let State::FoundTag(_) = self.state {
            return Err(error::new(ErrorImpl::SerializeNestedEnum));
        }
        self.state = State::FoundTag(variant.to_owned());
        self.emit_mapping_start()?;
        Ok(self)
    }

    // -- MISCELLANEOUS --

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        let string = if matches!(
            self.state,
            State::CheckForTag | State::CheckForDuplicateTag
        ) {
            match tagged::check_for_tag(value) {
                MaybeTag::NotTag(string) => string,
                MaybeTag::Tag(string) => {
                    return if let State::CheckForDuplicateTag =
                        self.state
                    {
                        Err(error::new(ErrorImpl::SerializeNestedEnum))
                    } else {
                        self.state = State::FoundTag(string);
                        Ok(())
                    };
                }
            }
        } else {
            value.to_string()
        };

        self.serialize_str(&string)
    }
}

// ----------------------------------------------------------------------------
// SEQ, TUPLE, MAP, STRUCT, etc. HELPERS
// ----------------------------------------------------------------------------

impl<W> ser::SerializeSeq for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        elem.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<W> ser::SerializeTuple for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        elem.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<W> ser::SerializeTupleStruct for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<W> ser::SerializeTupleVariant for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, v: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<W> ser::SerializeMap for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.flush_mapping_start()?;
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_entry<K, V>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        key.serialize(&mut **self)?;
        let tagged = matches!(self.state, State::FoundTag(_));
        value.serialize(&mut **self)?;
        if tagged {
            self.state = State::AlreadyTagged;
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        if matches!(self.state, State::CheckForTag) {
            self.emit_mapping_start()?;
        }
        if !matches!(self.state, State::AlreadyTagged) {
            self.emit_mapping_end()?;
        }
        self.state = State::NothingInParticular;
        Ok(())
    }
}

impl<W> ser::SerializeStruct for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(
        &mut self,
        key: &'static str,
        value: &V,
    ) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        self.serialize_str(key)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_mapping_end()
    }
}

impl<W> ser::SerializeStructVariant for &mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(
        &mut self,
        field: &'static str,
        v: &V,
    ) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        self.serialize_str(field)?;
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_mapping_end()
    }
}

// ----------------------------------------------------------------------------
// TOP-LEVEL CONVENIENCE FUNCTIONS
// ----------------------------------------------------------------------------

/// Serialize the given data structure as YAML into the provided IO writer.
///
/// # Overview
/// This is the simplest entry point if you already have a `Writer`.
///
/// # Returns
/// `Ok(())` on success; an [`Error`] if serialization fails.
///
/// # Errors
/// - If `T` cannot be represented (e.g., nested enum issues).
/// - If I/O fails to write the YAML data.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// use serde_yml::ser::to_writer;
/// use std::collections::BTreeMap;
///
/// fn main() -> serde_yml::Result<()> {
///     let mut buffer = Vec::new();
///     let mut object = BTreeMap::new();
///     object.insert("answer", 42);
///
///     to_writer(&mut buffer, &object)?;
///     println!("{}", String::from_utf8_lossy(&buffer));
///     Ok(())
/// }
/// ```
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + ser::Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}

/// Serialize the given data structure into a YAML `String`.
///
/// # Overview
/// This convenience method allocates an in-memory `Vec<u8>` to collect
/// the YAML output and then converts it to a `String`.
///
/// # Returns
/// A `String` containing the YAML representation of `value`.
///
/// # Errors
/// - Fails if serialization fails for `T`.
/// - Fails if the resulting bytes are not valid UTF-8.
///
/// # Examples
/// ```
/// use serde::Serialize;
/// use serde_yml::ser::to_string;
///
/// #[derive(Serialize)]
/// struct Example {
///     foo: u32,
///     bar: String,
/// }
///
/// fn main() -> serde_yml::Result<()> {
///     let example = Example { foo: 123, bar: "hello".into() };
///     let yaml_str = to_string(&example)?;
///     println!("YAML Output:\n{}", yaml_str);
///     Ok(())
/// }
/// ```
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + ser::Serialize,
{
    let mut vec = Vec::with_capacity(128);
    to_writer(&mut vec, value)?;
    String::from_utf8(vec)
        .map_err(|error| error::new(ErrorImpl::FromUtf8(error)))
}
