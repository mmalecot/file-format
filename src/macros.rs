//! Macros for generating [FileFormat](crate::FileFormat) enum and associated methods.

/// Generates the [FileFormat](crate::FileFormat) enum with methods for retrieving information.
///
/// # Parameters
///
/// - `format`: Variant name representing the file format.
/// - `name`: Full name of the file format.
/// - `short_name`: Abbreviated name of the file format (optional).
/// - `media_type`: Common media type associated with the file format.
/// - `extension`: Common file extension used for the file format.
/// - `kind`: Type or category of the file format.
macro_rules! formats {
    {
        $(
            format = $format:ident
            name = $name:literal
            $(short_name = $short_name:literal)?
            media_type = $media_type:literal
            extension = $extension:literal
            kind = $kind:ident
        )*
    } => {
        /// A file format.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub enum FileFormat {
            $(
                #[doc=concat!($name, $(" (", $short_name, ")",)? ".")]
                #[doc=concat!("- Media type: `", $media_type, "`")]
                #[doc=concat!("- Extension: `.", $extension, "`")]
                #[doc=concat!("- Kind: [", stringify!($kind), "](crate::Kind::", stringify!($kind), ")")]
                $format,
            )*
        }

        impl crate::FileFormat {
            /// Returns the full name of the file format.
            ///
            /// # Examples
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::Mpeg12AudioLayer3;
            /// assert_eq!(format.name(), "MPEG-1/2 Audio Layer 3");
            ///```
            pub const fn name(&self) -> &str {
                match self {
                    $(
                        Self::$format => $name,
                    )*
                }
            }

            /// Returns the short name of the file format.
            ///
            /// Note: This information is not necessarily unique, as multiple file formats might
            /// share the same short name.
            ///
            /// # Examples
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::MusicalInstrumentDigitalInterface;
            /// assert_eq!(format.short_name(), Some("MIDI"));
            ///```
            pub const fn short_name(&self) -> Option<&str> {
                match self {
                    $(
                        $(Self::$format => Some($short_name),)?
                    )*
                    _ => None,
                }
            }

            /// Returns the common media type (formerly known as MIME type) of the file format.
            ///
            /// Note: Some media types may not be defined in the IANA registry.
            ///
            /// # Examples
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::Zstandard;
            /// assert_eq!(format.media_type(), "application/zstd");
            ///```
            pub const fn media_type(&self) -> &str {
                match self {
                    $(
                        Self::$format => $media_type,
                    )*
                }
            }

            /// Returns the common extension of the file format.
            ///
            /// Note: This information is never empty.
            ///
            /// # Examples
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::WindowsMediaVideo;
            /// assert_eq!(format.extension(), "wmv");
            ///```
            pub const fn extension(&self) -> &str {
                match self {
                    $(
                        Self::$format => $extension,
                    )*
                }
            }

            /// Returns the [Kind](crate::Kind) of the file format.
            ///
            /// # Examples
            ///
            /// ```
            /// use file_format::{FileFormat, Kind};
            ///
            /// let format = FileFormat::Zip;
            /// assert_eq!(format.kind(), Kind::Archive);
            ///```
            pub const fn kind(&self) -> crate::Kind {
                match self {
                    $(
                        Self::$format => crate::Kind::$kind,
                    )*
                }
            }
        }
    };
}

/// Generates the [FileFormat::from_signature](crate::FileFormat::from_signature) function.
///
/// # Parameters
///
/// - `format`: Variant name representing the file format.
/// - `value`: Signature value associated with the format (can be repeated).
/// - `offset`: Offset to start matching the signature value (defaults to 0 if not specified).
macro_rules! signatures {
    {
        $(
            format = $format:ident
            $(value = $($value:literal $(offset = $offset:literal)?),+)+
        )*
    } => {
        impl crate::FileFormat {
            /// Determines file format by checking its signature.
            #[allow(clippy::int_plus_one)]
            pub(crate) fn from_signature(bytes: &[u8]) -> Option<Self> {
                $(
                    if $($(bytes.len() >= $($offset +)? $value.len()
                        && &bytes[$($offset)?..$($offset +)? $value.len()] == $value)&&*)||* {
                        return Some(Self::$format);
                    }
                )*
                None
            }
        }
    };
}
