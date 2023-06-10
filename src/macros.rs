//! Macros.

/// Generates `FileFormat` enum with methods.
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
                #[doc=concat!("- Extension: `", $extension, "`")]
                #[doc=concat!("- Kind: [", stringify!($kind), "](crate::Kind::", stringify!($kind), ")")]
                $format,
            )*
        }

        impl crate::FileFormat {
            /// Returns the full name of the file format.
            ///
            /// # Examples
            ///
            /// ```rust
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
            /// # Examples
            ///
            /// ```rust
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
            /// ```rust
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
            /// # Examples
            ///
            /// ```rust
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
            /// ```rust
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
        /// Generate a std::cell:OnceCell that maintains a hashmap of media types to kinds.
        /// This is used to speed up the `Kind::from_media_type` function.
        #[cfg(feature = "map-media-to-kind")]
        static MEDIA_TYPE_TO_KIND:
            ::std::sync::OnceLock<std::collections::HashMap<&'static str, Vec<crate::Kind>>> = ::std::sync::OnceLock::new();
        #[cfg(feature = "map-media-to-kind")]
        impl crate::Kind {
        
            /// Returns the kind for a given media type.
            /// Returns `None` if the media type is unknown.
            pub fn from_media_type(media_type: &str) -> Option<Vec<Self>> {
                MEDIA_TYPE_TO_KIND.get_or_init(|| {
                    let mut map = std::collections::HashMap::new();
                    $(
                        let entry = map.entry($media_type).or_insert_with(Vec::new);
                        if !entry.contains(&Self::$kind) {
                            entry.push(Self::$kind);
                        }
                    )*
                    map
                }).get(media_type).cloned()
            }
        }
    };
}

/// Generates `FileFormat::from_signature` function.
macro_rules! signatures {
    {
        $(
            format = $format:ident
            $(value = $($value:literal $(offset = $offset:literal)?),+)+
        )*
    } => {
        impl crate::FileFormat {
            /// Determines file format by checking its signature.
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
