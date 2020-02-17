//! File format library for Rust.
//!
//! # Supported file formats
//!
//! | Name                              | Media type                | Extensions    |
//! |-----------------------------------|---------------------------|---------------|
//! | Binary                            | application/octet-stream  | bin           |
//! | Bzip2                             | application/x-bzip2       | bz2           |
//! | GNU zip                           | application/gzip          | gz            |
//! | Long Range ZIP                    | application/x-lrzip       | lrz           |
//! | Lzip                              | application/x-lzip        | lz            |
//! | LZ4                               | application/x-lz4         | lz4           |
//! | Lempel-Ziv-Oberhumer              | application/x-lzop        | lzo           |
//! | XZ                                | application/x-xz          | xz            |
//! | Compress                          | application/x-compress    | Z             |
//! | Zstandard                         | application/zstd          | zst           |
//! | Audio Interchange File Format     | audio/aiff                | aif, aiff     |
//! | Au                                | audio/basic               | au, snd       |
//! | Free Lossless Audio Codec         | audio/x-flac              | flac          |
//! | M4V                               | audio/x-m4a               | m4a           |
//! | MPEG-1 Audio Layer III            | audio/mpeg                | mp3           |
//! | Ogg                               | audio/ogg                 | ogg, oga, spx |
//! | Waveform Audio File Format        | audio/vnd.wave            | wav           |
//! | WavPack                           | audio/wavpack             | wv            |
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
//! | GL Transmission Format binary     | model/gltf-binary         | glb           |
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
//! | Windows Media Video               | video/x-ms-asf            | wmv, wm       |
//! | UTF-8 text                        | text/plain                | txt           |
//!

use std::{fs, io::Error, path::Path, str};

/// Creates a boolean expression from a pattern.
macro_rules! pattern {
    ($bytes:expr, $offset:literal => $signature:literal) => {
        $bytes.len() >= $offset + $signature.len() && &$bytes[$offset..$offset + $signature.len()] == $signature
    };
    ($bytes:expr, $(($($literal1:literal => $literal2:literal)&&+)),+) => {
        $($(pattern!($bytes, $literal1 => $literal2))&&*)||*
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
        $($kind:ident, $media_type:literal, $($extension:literal),+, $(($($pattern:tt)*)),+),*
    } => {
        impl FileFormat {
            #[inline]
            fn from_bytes_impl(bytes: &[u8]) -> FileFormat {
                $(if pattern!(bytes, $(($($pattern)*)),*) {
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
    Application, "application/x-bzip2",       "bz2",               (0 => b"BZh"),
    Application, "application/gzip",          "gz",                (0 => b"\x1F\x8B"),
    Application, "application/x-lrzip",       "lrz",               (0 => b"LRZI"),
    Application, "application/x-lzip",        "lz",                (0 => b"LZIP"),
    Application, "application/x-lz4",         "lz4",               (0 => b"\x04\x22\x4D\x18"),
    Application, "application/x-lzop",        "lzo",               (0 => b"\x89\x4C\x5A\x4F\x00\x0D\x0A\x1A\x0A"),
    Application, "application/x-xz",          "xz",                (0 => b"\xFD\x37\x7A\x58\x5A\x00"),
    Application, "application/x-compress",    "Z",                 (0 => b"\x1F\xA0"),
                                                                   (0 => b"\x1F\x9D"),
    Application, "application/zstd",          "zst",               (0 => b"\x28\xB5\x2F\xFD"),
    Audio,       "audio/aiff",                "aif", "aiff",       (0 => b"FORM"),
    Audio,       "audio/basic",               "au", "snd",         (0 => b".snd"),
    Audio,       "audio/x-flac",              "flac",              (0 => b"fLaC"),
    Audio,       "audio/x-m4a",               "m4a",               (4 => b"ftypM4A"),
    Audio,       "audio/mpeg",                "mp3",               (0 => b"ID3"),
    Audio,       "audio/ogg",                 "ogg", "oga", "spx", (0 => b"OggS" && 29 => b"vorbis"),
                                                                   (0 => b"OggS" && 29 => b"FLAC"),
                                                                   (0 => b"OggS" && 28 => b"Speex"),
    Audio,       "audio/vnd.wave",            "wav",               (0 => b"\x52\x49\x46\x46" && 8 => b"\x57\x41\x56\x45"),
    Audio,       "audio/wavpack",             "wv",                (0 => b"wvpk"),
    Font,        "font/otf",                  "otf",               (0 => b"\x4F\x54\x54\x4F\x00"),
    Font,        "font/ttf",                  "ttf",               (0 => b"\x00\x01\x00\x00\x00"),
    Font,        "font/woff",                 "woff",              (0 => b"wOFF"),
    Font,        "font/woff2",                "woff2",             (0 => b"wOF2"),
    Image,       "image/bmp",                 "bmp", "dlib",       (0 => b"BM"),
    Image,       "image/bpg",                 "bpg",               (0 => b"\x42\x50\x47\xFB"),
    Image,       "image/flif",                "flif",              (0 => b"FLIF"),
    Image,       "image/gif",                 "gif",               (0 => b"GIF"),
    Image,       "image/heic",                "heic",              (4 => b"ftypheic"),
                                                                   (4 => b"ftypheix"),
    Image,       "image/x-icon",              "ico",               (0 => b"\x00\x00\x01\x00"),
    Image,       "image/jp2",                 "jp2",               (16 => b"ftypjp2"),
    Image,       "image/jpeg",                "jpg", "jpeg",       (0 => b"\xFF\xD8\xFF"),
    Image,       "image/jxr",                 "jxr", "hdp", "wdp", (0 => b"\x49\x49\xBC"),
    Image,       "image/png",                 "png",               (0 => b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"),
    Image,       "image/vnd.adobe.photoshop", "psd",               (0 => b"8BPS"),
    Image,       "image/tiff",                "tiff", "tif",       (0 => b"\x49\x49\x2A\x00"),
    Image,       "image/webp",                "webp",              (8 => b"WEBP"),
    Model,       "model/gltf-binary",         "glb",               (0 => b"glTF"),
    Video,       "video/3gpp",                "3gp",               (4 => b"ftyp3gp"),
    Video,       "video/avi",                 "avi",               (0 => b"\x52\x49\x46\x46" && 8 => b"\x41\x56\x49"),
    Video,       "video/x-flv",               "flv",               (0 => b"\x46\x4C\x56\x01"),
    Video,       "video/x-m4v",               "m4v",               (4 => b"ftypM4V"),
    Video,       "video/x-matroska",          "mkv",               (0 => b"\x1A\x45\xDF\xA3" && 24 => b"matroska"),
    Video,       "video/quicktime",           "mov", "qt",         (0 => b"\x00\x00\x00\x14" && 4 => b"ftypqt"),
    Video,       "video/mp4",                 "mp4",               (4 => b"ftypavc1"),
                                                                   (4 => b"ftypdash"),
                                                                   (4 => b"ftypiso2"),
                                                                   (4 => b"ftypiso3"),
                                                                   (4 => b"ftypiso4"),
                                                                   (4 => b"ftypiso5"),
                                                                   (4 => b"ftypiso6"),
                                                                   (4 => b"ftypisom"),
                                                                   (4 => b"ftypmmp4"),
                                                                   (4 => b"ftypmp41"),
                                                                   (4 => b"ftypmp42"),
                                                                   (4 => b"ftypmp4v"),
                                                                   (4 => b"ftypmp71"),
                                                                   (4 => b"ftypMSNV"),
                                                                   (4 => b"ftypNDAS"),
                                                                   (4 => b"ftypNDSC"),
                                                                   (4 => b"ftypNDSH"),
                                                                   (4 => b"ftypNDSM"),
                                                                   (4 => b"ftypNDSP"),
                                                                   (4 => b"ftypNDSS"),
                                                                   (4 => b"ftypNDXC"),
                                                                   (4 => b"ftypNDXH"),
                                                                   (4 => b"ftypNDXM"),
                                                                   (4 => b"ftypNDXP"),
                                                                   (4 => b"ftypF4V "),
                                                                   (4 => b"ftypF4P "),
    Video,       "video/mpeg",                "mpg",               (0 => b"\x00\x00\x01\xBA"),
                                                                   (0 => b"\x00\x00\x01\xB3"),
    Video,       "video/ogg",                 "ogv",               (0 => b"OggS" && 29 => b"theora"),
    Video,       "video/webm",                "webm",              (0 => b"\x1A\x45\xDF\xA3" && 24 => b"webm"),
    Video,       "video/x-ms-asf",            "wmv", "wm",         (0 => b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9")
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
