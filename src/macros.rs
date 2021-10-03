/// Checks if the specified bytes match the signature.
macro_rules! check_signature {
    ($bytes:expr, $offset:literal, $signature:literal) => {
        $bytes.len() >= $offset + $signature.len() && &$bytes[$offset..$offset + $signature.len()] == $signature
    };
    ($bytes:expr, $([$([$offset:literal, $signature:literal]),*]),*) => {
        $($(check_signature!($bytes, $offset, $signature))&&*)||*
    };
}

/// Generates `FileFormat` enum using a database described in YAML-like format.
macro_rules! file_format {
    {
        $(
            -   format: $format:ident
                description: $description:literal
                media_type: $media_type:literal
                extensions: [$($extension:literal),+]
                signatures:
                $(
                    -   parts:
                    $(
                        -   offset: $offset:literal
                            value: $signature:literal
                    )+
                )+
        )+
    } => {
        use std::{fs, io::Error, path::Path, str};

        /// Represents a file format.
        #[derive(Clone, Debug, PartialEq)]
        pub enum FileFormat {
            $(
                #[doc=$description]
                $format,
            )*
            /// UTF-8 encoded text
            Text,
            /// Arbitrary binary data
            Binary,
        }

        impl FileFormat {
            /// Determines file format from bytes.
            ///
            /// If the signature of the file is not recognized,
            /// it will try to detect if the file is an UTF-8 encoded text and if so return [`FileFormat::Text`].
            /// Otherwise, it will return [`FileFormat::Binary`] which corresponds to arbitrary binary data.
            ///
            /// # Examples
            ///
            /// Detects from the first bytes of a PNG file:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_bytes(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");
            /// assert_eq!(format, FileFormat::Png);
            /// assert_eq!(format.media_type(), "image/png");
            /// assert_eq!(format.preferred_extension(), "png");
            ///```
            ///
            /// Detects from an UTF-8 text:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_bytes("岁月不留人".as_bytes());
            /// assert_eq!(format, FileFormat::Text);
            /// assert_eq!(format.media_type(), "text/plain");
            /// assert_eq!(format.preferred_extension(), "txt");
            ///```
            ///
            /// Detects from a zeroed buffer:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_bytes(&[0; 1000]);
            /// assert_eq!(format, FileFormat::Binary);
            /// assert_eq!(format.media_type(), "application/octet-stream");
            /// assert_eq!(format.preferred_extension(), "bin");
            ///```
            pub fn from_bytes(bytes: &[u8]) -> FileFormat {
                $(if check_signature!(bytes, $([$([$offset, $signature]),*]),*) {
                    return FileFormat::$format;
                })*
                if FileFormat::is_utf8_text(bytes) {
                    FileFormat::Text
                } else {
                    FileFormat::Binary
                }
            }

            /// Determines file format from an extension.
            ///
            /// # Examples
            ///
            /// Detects from an extension:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_extension("jpeg");
            /// assert_eq!(format, Some(FileFormat::Jpeg));
            ///```
            ///
            /// Detects from an unknown extension:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_extension("foobar");
            /// assert_eq!(format, None);
            ///```
            pub fn from_extension(extension: &str) -> Option<FileFormat> {
                match extension {
                    $($($extension)|* => Some(FileFormat::$format),)*
                    "txt" => Some(FileFormat::Text),
                    "bin" => Some(FileFormat::Binary),
                    _ => None,
                }
            }

            /// Determines file format from a media type.
            ///
            /// # Examples
            ///
            /// Detects from a media type:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_media_type("image/png");
            /// assert_eq!(format, Some(FileFormat::Png));
            ///```
            ///
            /// Detects from an unknown media type:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_media_type("foo/bar");
            /// assert_eq!(format, None);
            ///```
            pub fn from_media_type(media_type: &str) -> Option<FileFormat> {
                match media_type {
                    $($media_type => Some(FileFormat::$format),)*
                    "text/plain" => Some(FileFormat::Text),
                    "application/octet-stream" => Some(FileFormat::Binary),
                    _ => None,
                }
            }

            /// Determines file format from a file.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::from_file("Cargo.toml")?;
            /// assert_eq!(format, FileFormat::Text);
            /// # Ok::<(), std::io::Error>(())
            ///```
            pub fn from_file<P: AsRef<Path>>(path: P) -> Result<FileFormat, Error> {
                Ok(FileFormat::from_bytes(&fs::read(&path)?))
            }

            /// Returns the media type (formerly known as MIME type) of the file format.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::Zstandard;
            /// assert_eq!(format.media_type(), "application/zstd");
            ///```
            pub fn media_type(&self) -> &str {
               match self {
                   $(FileFormat::$format => $media_type,)*
                   FileFormat::Text => "text/plain",
                   FileFormat::Binary => "application/octet-stream"
               }
            }

            /// Returns the extensions list of the file format.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::Midi;
            /// assert_eq!(format.extensions(), vec!["mid", "midi"]);
            ///```
            pub fn extensions(&self) -> Vec<&str> {
                match self {
                   $(FileFormat::$format => vec!($($extension),*),)*
                   FileFormat::Text => vec!("txt"),
                   FileFormat::Binary => vec!("bin")
               }
            }

            /// Returns the preferred extension of the file format.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::Wmv;
            /// assert_eq!(format.preferred_extension(), "wmv");
            ///```
            pub fn preferred_extension(&self) -> &str {
                self.extensions().first().unwrap()
            }

            /// Returns `true` if the specified bytes can be considered as human readable UTF-8 text.
            /// Except `\t`, `\n` and `\r`, we consider all control characters as non human readable.
            fn is_utf8_text(bytes: &[u8]) -> bool {
                str::from_utf8(bytes).map_or(false, |string| {
                    string
                        .chars()
                        .find(|character| {
                            character.is_control()
                                && *character != '\t'
                                && *character != '\n'
                                && *character != '\r'
                        })
                        .is_none()
                })
            }
        }
    };
}
