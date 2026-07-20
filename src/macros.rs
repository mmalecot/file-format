//! Internal macros for generating the [`FileFormat`](crate::FileFormat) enum and its associated
//! methods from declarative format and signature definitions.
//!
//! These macros are not part of the public API.

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
        /// Represents a specific file format identified by its magic-byte signature or
        /// internal structure.
        ///
        /// Each variant corresponds to a single file format and provides associated metadata
        /// accessible via [`name`](Self::name), [`short_name`](Self::short_name),
        /// [`media_type`](Self::media_type), [`extension`](Self::extension), and
        /// [`kind`](Self::kind).
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[non_exhaustive]
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

            /// Returns the short name (abbreviation) of the file format, if one exists.
            ///
            /// Returns `None` for formats that do not have a widely recognized abbreviation.
            ///
            /// Note: this value is not necessarily unique, as multiple file formats may share
            /// the same short name.
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

            /// Returns the common file extension of the file format (without the leading dot).
            ///
            /// Note: this value is never empty.
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

            /// Returns all file formats of the given [`Kind`](crate::Kind).
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::{FileFormat, Kind};
            ///
            /// let formats = FileFormat::from_kind(Kind::Image);
            /// assert!(formats.contains(&FileFormat::PortableNetworkGraphics));
            /// assert!(formats.contains(&FileFormat::JointPhotographicExpertsGroup));
            ///```
            pub fn from_kind(kind: crate::Kind) -> &'static [crate::FileFormat] {
                static KIND: std::sync::OnceLock<std::collections::HashMap<crate::Kind, Vec<crate::FileFormat>>> =
                    std::sync::OnceLock::new();
                KIND.get_or_init(|| {
                    let mut map: std::collections::HashMap<crate::Kind, Vec<crate::FileFormat>> =
                        std::collections::HashMap::new();
                    $(
                        map.entry(crate::Kind::$kind).or_default().push(crate::FileFormat::$format);
                    )*
                    map
                })
                .get(&kind)
                .map(Vec::as_slice)
                .unwrap_or_default()
            }

            /// Returns all file formats that use the given file extension.
            ///
            /// The lookup is case-insensitive and a leading `.` is stripped automatically.
            ///
            /// Multiple file formats can share the same extension, so this returns a slice.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let formats = FileFormat::from_extension("jpg");
            /// assert!(formats.contains(&FileFormat::JointPhotographicExpertsGroup));
            ///
            /// // Case-insensitive and leading dot is stripped.
            /// assert_eq!(FileFormat::from_extension(".JPG"), FileFormat::from_extension("jpg"));
            ///```
            pub fn from_extension(extension: &str) -> &'static [crate::FileFormat] {
                static EXTENSION: std::sync::OnceLock<std::collections::HashMap<String, Vec<crate::FileFormat>>> =
                    std::sync::OnceLock::new();
                let extension = extension.strip_prefix('.').unwrap_or(extension);
                let lower = extension.to_ascii_lowercase();
                EXTENSION.get_or_init(|| {
                    let mut map: std::collections::HashMap<String, Vec<crate::FileFormat>> =
                        std::collections::HashMap::new();
                    $(
                        map.entry($extension.to_ascii_lowercase()).or_default().push(Self::$format);
                    )*
                    map
                })
                .get(&lower)
                .map(Vec::as_slice)
                .unwrap_or_default()
            }

            /// Returns all file formats that use the given media type.
            ///
            /// The lookup is case-insensitive.
            ///
            /// Multiple file formats can share the same media type, so this returns a slice.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use file_format::FileFormat;
            ///
            /// let formats = FileFormat::from_media_type("image/jpeg");
            /// assert!(formats.contains(&FileFormat::JointPhotographicExpertsGroup));
            ///
            /// // Case-insensitive.
            /// assert_eq!(FileFormat::from_media_type("IMAGE/JPEG"), FileFormat::from_media_type("image/jpeg"));
            ///```
            pub fn from_media_type(media_type: &str) -> &'static [crate::FileFormat] {
                static MEDIA_TYPE: std::sync::OnceLock<std::collections::HashMap<&str, Vec<crate::FileFormat>>> =
                    std::sync::OnceLock::new();
                let lower = media_type.to_ascii_lowercase();
                MEDIA_TYPE.get_or_init(|| {
                    let mut map: std::collections::HashMap<&str, Vec<crate::FileFormat>> =
                        std::collections::HashMap::new();
                    $(
                        map.entry($media_type).or_default().push(Self::$format);
                    )*
                    map
                })
                .get(lower.as_str())
                .map(Vec::as_slice)
                .unwrap_or_default()
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
            /// Maximum number of bytes needed to match any known signature.
            pub(crate) const SIGNATURE_MAX_LEN: usize = {
                let sizes: &[usize] = &[
                    $($($(
                        $($offset +)? $value.len(),
                    )+)+)*
                ];
                let mut max = 0;
                let mut i = 0;
                while i < sizes.len() {
                    if sizes[i] > max {
                        max = sizes[i];
                    }
                    i += 1;
                }
                max
            };

            /// Attempts to identify the file format by matching `bytes` against all known
            /// magic-byte signatures.
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
