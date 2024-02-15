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
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

            /// Determines the file formats associated with the given extension.
            /// If the extension is not recognized, `None` is returned.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let fmts = FileFormat::from_extension("pdf");
            /// assert_eq!(fmts, vec![FileFormat::PortableDocumentFormat]);
            ///
            /// let fmts = FileFormat::from_extension("tar.gz");
            /// assert_eq!(fmts, vec![FileFormat::Gzip]);
            /// ```
            pub fn from_extension(extension: &str) -> Vec<Self> {
                #[cfg(not(feature = "rust-1_70"))]
                {
                    let extensions = crate::formats::EXTENSIONS_TO_FORMATS
                        .iter()
                        .filter_map(|(ext, fmt)| if *ext == extension { Some(*fmt) } else { None })
                        .collect::<Vec<_>>();
                    if extensions.is_empty() {
                        if let Some((_, extension)) = extension.split_once('.') {
                            return Self::from_extension(extension);
                        } else {
                            return vec![Self::ArbitraryBinaryData];
                        }
                    }

                    extensions
                }
                #[cfg(feature = "rust-1_70")]
                {
                    static EXTENSIONS_TO_FORMATS: std::sync::OnceLock<std::collections::BTreeMap<&str, Vec<FileFormat>>> = std::sync::OnceLock::new();
                    let map = EXTENSIONS_TO_FORMATS.get_or_init(|| {
                        let mut map = std::collections::BTreeMap::new();
                        $(
                            map.entry($extension)
                                .or_insert_with(Vec::new)
                                .push(FileFormat::$format);
                        )*
                        map
                    });

                    let extensions = map.get(extension).cloned().unwrap_or_default();
                    if extensions.is_empty() {
                        if let Some((_, extension)) = extension.split_once('.') {
                            return Self::from_extension(extension);
                        } else {
                            return vec![Self::ArbitraryBinaryData];
                        }
                    }

                    extensions
                }
            }
        }

        #[cfg(not(feature = "rust-1_70"))]
        pub(crate) static EXTENSIONS_TO_FORMATS: &[(&str, FileFormat)] = &[
            $(
                ($extension, FileFormat::$format),
            )*
        ];
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
