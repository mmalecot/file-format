//! Convenient macros.

/// Generates [`FileFormat`] enum with methods.
macro_rules! formats {
    {
        $(
            format = $format:ident
            name = $name:literal
            media_type = $media_type:literal
            extension = $extension:literal
            kind = $kind:ident
        )*
    } => {
        /// A file format.
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum FileFormat {
            $(
                #[doc=concat!($name, ".")]
                #[doc=""]
                #[doc=concat!("**Media type**: `", $media_type, "`")]
                #[doc=""]
                #[doc=concat!("**Extension**: `", $extension, "`")]
                $format,
            )*
        }

        impl crate::FileFormat {
            /// Returns the name of the [`FileFormat`].
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

            /// Returns the media type (formerly known as MIME type) of the [`FileFormat`].
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

            /// Returns the extension of the [`FileFormat`].
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

            /// Returns the [`Kind`] of the [`FileFormat`].
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::{FileFormat, Kind};
            ///
            /// let format = FileFormat::FreeLosslessAudioCodec;
            /// assert_eq!(format.kind(), Kind::Audio);
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

/// Generates [`FileFormat::from_signature`] function.
macro_rules! signatures {
    {
        $(
            format = $format:ident
            $(value = $($value:literal $(offset = $offset:literal)?),+)+
        )*
    } => {
        impl crate::FileFormat {
            /// Determines [`FileFormat`] by checking its signature.
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
