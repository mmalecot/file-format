//! File format library for Rust.
//!
//! # Supported file formats
//!
//! | Name                              | Media type                | Extensions    |
//! |-----------------------------------|---------------------------|---------------|
//! | Binary                            | application/octet-stream  | bin           |
//! | Microsoft Windows bitmap          | image/bmp                 | bmp, dlib     |
//! | Better Portable Graphics          | image/bpg                 | bpg           |
//! | Free Lossless Image Format        | image/flif                | flif          |
//! | Graphics Interchange Format       | image/gif                 | gif           |
//! | High Efficiency Image File Format | image/heic                | heic          |
//! | Microsoft icon                    | image/x-icon              | ico           |
//! | JPEG-2000 JP2                     | image/jp2                 | jp2           |
//! | Joint Photographic Experts Group  | image/jpeg                | jpg, jpeg     |
//! | JPEG extended range               | image/jxr                 | jxr, hdp, wdp |
//! | Portable Network Graphics         | image/png                 | png           |
//! | Adobe Photoshop bitmap            | image/vnd.adobe.photoshop | psd           |
//! | Tagged Image File Format          | image/tiff                | tiff, tif     |
//! | Weppy image format                | image/webp                | webp          |
//! | UTF-8 text                        | text/plain                | txt           |
//!

use std::{fs, io::Error, path::Path, str};

/// Creates a file format.
macro_rules! file_format {
    ($kind:expr, $media_type:literal, $($extension:literal),+) => {
        FileFormat {
            kind: $kind,
            media_type: String::from($media_type),
            extensions: vec![$(String::from($extension)),*],
        }
    };
}

/// Generates `from_bytes_impl`, `from_extension_impl` and `from_media_type_impl` functions
/// for `FileFormat` struct with a declaration of file formats and the way to detect them.
macro_rules! file_formats {
    {
        $($kind:expr, $media_type:literal,
            [$($extension:literal),+],
            [$([$($start:literal..=$end:literal == $bytes:literal),+]),+]
        ),*
    } => {
        impl FileFormat {
            #[inline]
            fn from_bytes_impl(bytes: &[u8]) -> FileFormat {
                $(
                    if $($(bytes.len() > $end && &bytes[$start..=$end] == $bytes) && *) || * {
                        return file_format!($kind, $media_type, $($extension),*);
                    }
                )*
                if FileFormat::is_utf8_text(bytes) {
                    file_format!(Kind::Text, "text/plain", "txt")
                } else {
                    file_format!(Kind::Application, "application/octet-stream", "bin")
                }
            }

            #[inline]
            fn from_extension_impl(extension: &str) -> Option<FileFormat> {
                match extension {
                    $(
                        $($extension) | * => Some(file_format!($kind, $media_type, $($extension),*)),
                    )*
                    "txt" => Some(file_format!(Kind::Text, "text/plain", "txt")),
                    "bin" => Some(file_format!(Kind::Application, "application/octet-stream", "bin")),
                    _ => None,
                }
            }

            #[inline]
            fn from_media_type_impl(media_type: &str) -> Option<FileFormat> {
                match media_type {
                    $(
                        $media_type => Some(file_format!($kind, $media_type, $($extension),*)),
                    )*
                    "text/plain" => Some(file_format!(Kind::Text, "text/plain", "txt")),
                    "application/octet-stream" => Some(file_format!(Kind::Application, "application/octet-stream", "bin")),
                    _ => None,
                }
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

/// Represents a kind of file format.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// Multi-purpose.
    Application,
    /// Audio.
    Audio,
    /// Font.
    Font,
    /// Image.
    Image,
    /// 3D model.
    Model,
    /// Human readable text or source code.
    Text,
    /// Video.
    Video,
}

/// Represents a file format.
#[derive(Clone, Debug, PartialEq)]
pub struct FileFormat {
    kind: Kind,
    media_type: String,
    extensions: Vec<String>,
}

file_formats! {
    Kind::Image, "image/bmp", ["bmp", "dlib"], [[ 0..=1 == b"BM" ]],
    Kind::Image, "image/bpg", ["bpg"], [[ 0..=3 == b"\x42\x50\x47\xFB" ]],
    Kind::Image, "image/flif", ["flif"], [[ 0..=3 == b"FLIF" ]],
    Kind::Image, "image/gif", ["gif"], [[ 0..=2 == b"GIF" ]],
    Kind::Image, "image/heic", ["heic"], [[ 4..=7 == b"ftyp", 8..=11 == b"heic" ],
                                          [ 4..=7 == b"ftyp", 8..=11 == b"heix" ]],
    Kind::Image, "image/x-icon", ["ico"], [[ 0..=3 == b"\x00\x00\x01\x00" ]],
    Kind::Image, "image/jp2", ["jp2"], [[ 16..=19 == b"ftyp", 20..=22 == b"jp2" ]],
    Kind::Image, "image/jpeg", ["jpg", "jpeg"], [[ 0..=2 == b"\xFF\xD8\xFF" ]],
    Kind::Image, "image/jxr", ["jxr", "hdp", "wdp"], [[ 0..=2 == b"\x49\x49\xBC" ]],
    Kind::Image, "image/png", ["png"], [[ 0..=7 == b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A" ]],
    Kind::Image, "image/vnd.adobe.photoshop", ["psd"], [[ 0..=3 == b"8BPS" ]],
    Kind::Image, "image/tiff", ["tiff", "tif"], [[ 0..=3 == b"\x49\x49\x2A\x00" ]],
    Kind::Image, "image/webp", ["webp"], [[ 8..=11 == b"WEBP" ]]
}

impl FileFormat {
    /// Determines file format from a file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_file("Cargo.toml").unwrap();
    /// assert_eq!(format.kind(), Kind::Text);
    /// assert_eq!(format.media_type(), "text/plain");
    ///```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<FileFormat, Error> {
        Ok(FileFormat::from_bytes(&fs::read(&path)?))
    }

    /// Determines file format from bytes.
    ///
    /// If the signature of the file is not recognized,
    /// it will try to detect if the file is an UTF-8 encoded text and if so return `text/plain`.
    /// Otherwise, it will return `application/octet-stream` which corresponds to a generic binary type.
    ///
    /// # Examples
    ///
    /// ## Firsts bytes of a PNG
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_bytes(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");
    /// assert_eq!(format.kind(), Kind::Image);
    /// assert_eq!(format.media_type(), "image/png");
    /// assert_eq!(format.preferred_extension(), Some("png"));
    ///```
    ///
    /// ## UTF-8 text
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_bytes("Hello 😊!".as_bytes());
    /// assert_eq!(format.kind(), Kind::Text);
    /// assert_eq!(format.media_type(), "text/plain");
    /// assert_eq!(format.preferred_extension(), Some("txt"));
    ///```
    ///
    /// ## 512 bytes
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_bytes(&[0xFF; 512]);
    /// assert_eq!(format.kind(), Kind::Application);
    /// assert_eq!(format.media_type(), "application/octet-stream");
    /// assert_eq!(format.preferred_extension(), Some("bin"));
    ///```
    pub fn from_bytes(bytes: &[u8]) -> FileFormat {
        FileFormat::from_bytes_impl(bytes)
    }

    /// Determines file format from an extension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_extension("jpeg").unwrap();
    /// assert_eq!(format.kind(), Kind::Image);
    /// assert_eq!(format.media_type(), "image/jpeg");
    /// assert_eq!(format.preferred_extension(), Some("jpg"));
    ///```
    pub fn from_extension(extension: &str) -> Option<FileFormat> {
        FileFormat::from_extension_impl(extension)
    }

    /// Determines file format from a media type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_media_type("image/png").unwrap();
    /// assert_eq!(format.kind(), Kind::Image);
    /// assert_eq!(format.media_type(), "image/png");
    /// assert_eq!(format.preferred_extension(), Some("png"));
    ///```
    pub fn from_media_type(media_type: &str) -> Option<FileFormat> {
        FileFormat::from_media_type_impl(media_type)
    }

    /// Returns the kind of the file format.
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns the media type (formerly known as MIME type) of the file format according to
    /// the [IANA registy](https://www.iana.org/assignments/media-types/media-types.xhtml).
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    /// Returns the extensions list of the file format.
    pub fn extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    /// Returns the preferred extension of the file format.
    /// It can return `None` given that some file formats have no extension.
    pub fn preferred_extension(&self) -> Option<&str> {
        self.extensions.first().map(|string| string.as_str())
    }
}
