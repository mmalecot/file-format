//! File format library for Rust.
//!
//! # Supported file formats
//!
//! | Name                              | Media type                | Extensions    |
//! |-----------------------------------|---------------------------|---------------|
//! | Binary                            | application/octet-stream  | bin           |
//! | OpenType                          | font/otf                  | otf           |
//! | TrueType                          | font/ttf                  | ttf           |
//! | Web Open Font Format              | font/woff                 | woff          |
//! | Web Open Font Format 2            | font/woff2                | woff2         |
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
//! | 3GPP                              | video/3gpp                | 3gp           |
//! | Audio Video Interleave            | video/avi                 | avi           |
//! | Flash Video                       | video/x-flv               | flv           |
//! | M4V                               | video/x-m4v               | m4v           |
//! | Matroska Multimedia Container     | video/x-matroska          | mkv           |
//! | QuickTime File Format             | video/quicktime           | mov, qt       |
//! | MPEG                              | video/mpeg                | mpg           |
//! | MPEG-4 Part 14                    | video/mp4                 | mp4           |
//! | Ogg                               | video/ogv                 | ogv           |
//! | Weppy video format                | video/webp                | webp          |
//! | Windows Media Video               | x-ms-asf                  | wmv, wm       |
//! | UTF-8 text                        | text/plain                | txt           |
//!

use std::{fs, io::Error, path::Path, str};

/// Creates a boolean expression from a pattern.
macro_rules! pattern {
    ($bytes1:expr, $offset:literal => $bytes2:literal) => {
        $bytes1.len() >= $offset + $bytes2.len() && &$bytes1[$offset..$offset + $bytes2.len()] == $bytes2
    };
    ($bytes1:expr, $(($($inner:tt)*))&&*) => {
        $(pattern!($bytes1, $($inner)*))&&*
    };
    ($bytes1:expr, $(($($inner:tt)*))||*) => {
        $(pattern!($bytes1, $($inner)*))||*
    };
}

/// Creates a file format.
macro_rules! file_format {
    ($kind:expr, $media_type:literal, $($extension:literal),+) => {
        FileFormat {
            kind: $kind,
            media_type: String::from($media_type),
            extensions: vec![$(String::from($extension)),*],
        }
    };
    (text) => {
        file_format!(Kind::Text, "text/plain", "txt")
    };
    (binary) => {
        file_format!(Kind::Application, "application/octet-stream", "bin")
    };
}

/// Generates `from_bytes_impl`, `from_extension_impl` and `from_media_type_impl` functions
/// for `FileFormat` struct with a declaration of file formats and the way to detect them.
macro_rules! file_formats {
    {
        $($kind:ident, $media_type:literal, $($extension:literal),+, ($($patterns:tt)*)),*
    } => {
        impl FileFormat {
            #[inline]
            fn from_bytes_impl(bytes: &[u8]) -> FileFormat {
                $(if pattern!(bytes, $($patterns)*) {
                    return file_format!(Kind::$kind, $media_type, $($extension),*);
                })*
                if FileFormat::is_utf8_text(bytes) {
                    file_format!(text)
                } else {
                    file_format!(binary)
                }
            }

            #[inline]
            fn from_extension_impl(extension: &str) -> Option<FileFormat> {
                match extension {
                    $($($extension)|* => Some(file_format!(Kind::$kind, $media_type, $($extension),*)),)*
                    "txt" => Some(file_format!(text)),
                    "bin" => Some(file_format!(binary)),
                    _ => None,
                }
            }

            #[inline]
            fn from_media_type_impl(media_type: &str) -> Option<FileFormat> {
                match media_type {
                    $($media_type => Some(file_format!(Kind::$kind, $media_type, $($extension),*)),)*
                    "text/plain" => Some(file_format!(text)),
                    "application/octet-stream" => Some(file_format!(binary)),
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
    Font,  "font/otf",                  "otf",               (0 => b"\x4F\x54\x54\x4F\x00"),
    Font,  "font/ttf",                  "ttf",               (0 => b"\x00\x01\x00\x00\x00"),
    Font,  "font/woff",                 "woff",              (0 => b"wOFF"),
    Font,  "font/woff2",                "woff2",             (0 => b"wOF2"),
    Image, "image/bmp",                 "bmp", "dlib",       (0 => b"BM"),
    Image, "image/bpg",                 "bpg",               (0 => b"\x42\x50\x47\xFB"),
    Image, "image/flif",                "flif",              (0 => b"FLIF"),
    Image, "image/gif",                 "gif",               (0 => b"GIF"),
    Image, "image/heic",                "heic",              ((4 => b"ftyp") && ((8 => b"heic") || (8 => b"heix"))),
    Image, "image/x-icon",              "ico",               (0 => b"\x00\x00\x01\x00"),
    Image, "image/jp2",                 "jp2",               ((16 => b"ftyp") && (20 => b"jp2")),
    Image, "image/jpeg",                "jpg", "jpeg",       (0 => b"\xFF\xD8\xFF"),
    Image, "image/jxr",                 "jxr", "hdp", "wdp", (0 => b"\x49\x49\xBC"),
    Image, "image/png",                 "png",               (0 => b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"),
    Image, "image/vnd.adobe.photoshop", "psd",               (0 => b"8BPS"),
    Image, "image/tiff",                "tiff", "tif",       (0 => b"\x49\x49\x2A\x00"),
    Image, "image/webp",                "webp",              (8 => b"WEBP"),
    Video, "video/3gpp",                "3gp",               ((4 => b"ftyp") && (8 => b"3gp")),
    Video, "video/avi",                 "avi",               ((0 => b"\x52\x49\x46\x46") && (8 => b"\x41\x56\x49")),
    Video, "video/x-flv",               "flv",               (0 => b"\x46\x4C\x56\x01"),
    Video, "video/x-m4v",               "m4v",               ((4 => b"ftyp") && (8 => b"M4V")),
    Video, "video/x-matroska",          "mkv",               ((0 => b"\x1A\x45\xDF\xA3") && (24 => b"matroska")),
    Video, "video/quicktime",           "mov", "qt",         ((0 => b"\x00\x00\x00\x14") && (4 => b"ftyp") && (8 => b"qt")),
    Video, "video/mp4",                 "mp4",               ((4 => b"ftyp")
                                                              && ((8 => b"avc1")
                                                                  || (8 => b"dash")
                                                                  || (8 => b"iso2")
                                                                  || (8 => b"iso3")
                                                                  || (8 => b"iso4")
                                                                  || (8 => b"iso5")
                                                                  || (8 => b"iso6")
                                                                  || (8 => b"isom")
                                                                  || (8 => b"mmp4")
                                                                  || (8 => b"mp41")
                                                                  || (8 => b"mp42")
                                                                  || (8 => b"mp4v")
                                                                  || (8 => b"mp71")
                                                                  || (8 => b"MSNV")
                                                                  || (8 => b"NDAS")
                                                                  || (8 => b"NDSC")
                                                                  || (8 => b"NDSH")
                                                                  || (8 => b"NDSM")
                                                                  || (8 => b"NDSP")
                                                                  || (8 => b"NDSS")
                                                                  || (8 => b"NDXC")
                                                                  || (8 => b"NDXH")
                                                                  || (8 => b"NDXM")
                                                                  || (8 => b"NDXP")
                                                                  || (8 => b"F4V ")
                                                                  || (8 => b"F4P "))),
    Video, "video/mpeg",                "mpg",               ((0 => b"\x00\x00\x01\xBA") || (0 => b"\x00\x00\x01\xB3")),
    Video, "video/ogg",                 "ogv",               ((0 => b"OggS") && (29 => b"theora")),
    Video, "video/webm",                "webm",              ((0 => b"\x1A\x45\xDF\xA3") && (24 => b"webm")),
    Video, "video/x-ms-asf",            "wmv", "wm",         (0 => b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9")
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
    /// # Examples
    ///
    /// ## From jpeg extension
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_extension("jpeg").unwrap();
    /// assert_eq!(format.kind(), Kind::Image);
    /// assert_eq!(format.media_type(), "image/jpeg");
    /// assert_eq!(format.preferred_extension(), Some("jpg"));
    ///```
    ///
    /// ## From txt extension
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_extension("txt").unwrap();
    /// assert_eq!(format.kind(), Kind::Text);
    /// assert_eq!(format.media_type(), "text/plain");
    /// assert_eq!(format.preferred_extension(), Some("txt"));
    ///```
    ///
    /// ## Unknown extension
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::from_extension("foobar");
    /// assert_eq!(format, None);
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
