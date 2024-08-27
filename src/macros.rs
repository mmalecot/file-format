//! Macros for generating the [`FileFormat`](crate::FileFormat) enum and associated methods.

/// Generates the [`FileFormat`](crate::FileFormat) enum with methods for retrieving information.
///
/// # Parameters
///
/// - `format`: Variant representing the file format.
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
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "extended-enums", derive(strum::EnumIter, strum::AsRefStr, strum::FromRepr))]
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
            /// Basic usage:
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let fmt = FileFormat::Mpeg12AudioLayer3;
            /// assert_eq!(fmt.name(), "MPEG-1/2 Audio Layer 3");
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
            /// Note: this information is not necessarily unique, as multiple file formats might
            /// share the same short name.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let fmt = FileFormat::MusicalInstrumentDigitalInterface;
            /// assert_eq!(fmt.short_name(), Some("MIDI"));
            ///```
            pub const fn short_name(&self) -> Option<&str> {
                match self {
                    $(
                        $(Self::$format => Some($short_name),)?
                    )*
                    _ => None,
                }
            }

            /// Returns the common media type (formerly known as MIME type) of the file format as
            /// defined in [IETF RFC 6838](https://tools.ietf.org/html/rfc6838).
            ///
            /// Note: some media types may not be defined in the
            /// [IANA registry](https://www.iana.org/assignments/media-types/media-types.xhtml).
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let fmt = FileFormat::Zstandard;
            /// assert_eq!(fmt.media_type(), "application/zstd");
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
            /// Note: this information is never empty.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let fmt = FileFormat::WindowsMediaVideo;
            /// assert_eq!(fmt.extension(), "wmv");
            ///```
            pub const fn extension(&self) -> &str {
                match self {
                    $(
                        Self::$format => $extension,
                    )*
                }
            }

            /// Returns the [`Kind`](crate::Kind) of the file format.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::{FileFormat, Kind};
            ///
            /// let fmt = FileFormat::Zip;
            /// assert_eq!(fmt.kind(), Kind::Archive);
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

/// Generates the [`FileFormat::from_signature`](crate::FileFormat::from_signature) function.
///
/// # Parameters
///
/// - `format`: Variant representing the file format.
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


macro_rules! media_type_mapping {
    {
        $(
            media_type = $media_type:literal
            $(format = $format:ident)+
        )*
    } => {
        #[cfg(feature = "from-media-type")]
        impl crate::FileFormat {
            /// Determines the file format from the media type
            pub fn from_media_type(media_type: impl AsRef<str>) -> Option<&'static [Self]> {
                match media_type.as_ref() {
                    $($media_type => Some(&[$(Self::$format,)+]),)*
                    _ => None
                }
            }
        }
    };
}

macro_rules! extension_mapping {
    {
        $(
            extension = $extension:literal
            $(format = $format:ident)+
        )*
    } => {
        #[cfg(feature = "from-extension")]
        impl crate::FileFormat {
            /// Determines the file format from the extension
            pub fn from_extension(extension: impl AsRef<str>) -> Option<&'static [Self]> {
                match extension.as_ref().trim_start_matches('.') {
                    $($extension => Some(&[$(Self::$format,)+]),)*
                    _ => None
                }
            }
        }
    };
}

