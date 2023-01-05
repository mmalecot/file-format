/*!
Crate for determining the file format of a given file or stream.

It provides a variety of functions for identifying a wide range of file formats, including
[ZIP](`FileFormat::Zip`), [Compound File Binary (CFB)](`FileFormat::CompoundFileBinary`),
[Extensible Markup Language (XML)](`FileFormat::ExtensibleMarkupLanguage`) and [more](`FileFormat`).

It checks the signature of the file to determine its format. If the file format is not recognized by
its signature, it checks if it is [Plain Text (TXT)](`FileFormat::PlainText`). Otherwise, it returns
the default file format which is [Arbitrary Binary Data (BIN)](`FileFormat::ArbitraryBinaryData`).

## Examples

Determines from a file:

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_file("fixtures/text/sample.html")?;
assert_eq!(format, FileFormat::HypertextMarkupLanguage);
assert_eq!(format.name(), "HyperText Markup Language");
assert_eq!(format.short_name(), "HTML");
assert_eq!(format.display_name(), "HyperText Markup Language (HTML)");
assert_eq!(format.media_type(), "text/html");
assert_eq!(format.extension(), "html");
assert_eq!(format.kind(), Kind::Text);
# Ok::<(), std::io::Error>(())
```

Determines from bytes:

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_bytes(&[0xFF, 0xD8, 0xFF]);
assert_eq!(format, FileFormat::JointPhotographicExpertsGroup);
assert_eq!(format.name(), "Joint Photographic Experts Group");
assert_eq!(format.short_name(), "JPEG");
assert_eq!(format.display_name(), "Joint Photographic Experts Group (JPEG)");
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.extension(), "jpg");
assert_eq!(format.kind(), Kind::Image);
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
file-format = "0.13"
```

## Features

- `accuracy` - Improves the accuracy but may increase the processing time and memory usage.
- `cfb` - Enables [CFB](`FileFormat::CompoundFileBinary`)-based formats support:
    - [Microsoft Excel Spreadsheet (XLS)](`FileFormat::MicrosoftExcelSpreadsheet`)
    - [Microsoft PowerPoint Presentation (PPT)](`FileFormat::MicrosoftPowerpointPresentation`)
    - [Microsoft Project Plan (MPP)](`FileFormat::MicrosoftProjectPlan`)
    - [Microsoft Publisher Document (PUB)](`FileFormat::MicrosoftPublisherDocument`)
    - [Microsoft Software Installer (MSI)](`FileFormat::MicrosoftSoftwareInstaller`)
    - [Microsoft Visio Drawing (VSD)](`FileFormat::MicrosoftVisioDrawing`)
    - [Microsoft Word Document (DOC)](`FileFormat::MicrosoftWordDocument`)
- `zip` - Enables [ZIP](`FileFormat::Zip`)-based formats support:
    - [3D Manufacturing Format (3MF)](`FileFormat::ThreeDimensionalManufacturingFormat`)
    - [Android Package (APK)](`FileFormat::AndroidPackage`)
    - [Circuit Diagram Document (CDDX)](`FileFormat::CircuitDiagramDocument`)
    - [Design Web Format XPS (DWFX)](`FileFormat::DesignWebFormatXps`)
    - [Electronic Publication (EPUB)](`FileFormat::ElectronicPublication`)
    - [Enterprise Application Archive (EAR)](`FileFormat::EnterpriseApplicationArchive`)
    - [Java Archive (JAR)](`FileFormat::JavaArchive`)
    - [Keyhole Markup Language Zipped (KMZ)](`FileFormat::KeyholeMarkupLanguageZipped`)
    - [Microsoft Visual Studio Extension (VSIX)](`FileFormat::MicrosoftVisualStudioExtension`)
    - [MusicXML Zipped (MXL)](`FileFormat::MusicxmlZipped`)
    - [Office Open XML Document (DOCX)](`FileFormat::OfficeOpenXmlDocument`)
    - [Office Open XML Drawing (VSDX)](`FileFormat::OfficeOpenXmlDrawing`)
    - [Office Open XML Presentation (PPTX)](`FileFormat::OfficeOpenXmlPresentation`)
    - [Office Open XML Spreadsheet (XLSX)](`FileFormat::OfficeOpenXmlSpreadsheet`)
    - [OpenDocument Graphics (ODG)](`FileFormat::OpenDocumentGraphics`)
    - [OpenDocument Presentation (ODP)](`FileFormat::OpenDocumentPresentation`)
    - [OpenDocument Spreadsheet (ODS)](`FileFormat::OpenDocumentSpreadsheet`)
    - [OpenDocument Text (ODT)](`FileFormat::OpenDocumentText`)
    - [OpenRaster (ORA)](`FileFormat::Openraster`)
    - [Web Application Archive (WAR)](`FileFormat::WebApplicationArchive`)
    - [Windows App Package (APPX)](`FileFormat::WindowsAppPackage`)
    - [XAP](`FileFormat::Xap`)
    - [XPInstall (XPI)](`FileFormat::Xpinstall`)
    - [iOS App Store Package (IPA)](`FileFormat::IosAppStorePackage`)

All these features are disabled by default.
*/

#![deny(missing_docs)]

#[macro_use]
mod macros;

mod formats;
mod readers;
mod signatures;
mod utils;

use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Result, Seek},
    path::Path,
};

pub use formats::FileFormat;

impl FileFormat {
    /// Determines file format from bytes.
    ///
    /// # Examples
    ///
    /// Detects from the first bytes of a
    /// [Portable Network Graphics (PNG)](`FileFormat::PortableNetworkGraphics`) file:
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_bytes(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");
    /// assert_eq!(format, FileFormat::PortableNetworkGraphics);
    ///```
    ///
    /// Detects from a zeroed buffer:
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_bytes(&[0; 1000]);
    /// assert_eq!(format, FileFormat::ArbitraryBinaryData);
    ///```
    ///
    /// [default value]: FileFormat::default
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from(bytes)
    }

    /// Determines file format from a file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_file("fixtures/text/sample.txt")?;
    /// assert_eq!(format, FileFormat::PlainText);
    /// # Ok::<(), std::io::Error>(())
    ///```
    #[inline]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_reader(File::open(path)?)
    }

    /// Determines file format from a reader.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_reader(std::io::empty())?;
    /// assert_eq!(format, FileFormat::default());
    /// # Ok::<(), std::io::Error>(())
    ///```
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        let mut reader = BufReader::with_capacity(36870, reader);
        Ok(if reader.fill_buf()?.is_empty() {
            Self::default()
        } else if let Some(format) = Self::from_signature(reader.buffer()) {
            Self::from_format_reader(format, &mut reader)
                .unwrap_or_else(|_| Self::from_txt_reader(&mut reader).unwrap_or_default())
        } else {
            Self::from_txt_reader(&mut reader).unwrap_or_default()
        })
    }
}

impl Default for FileFormat {
    /// Returns the default file format which is
    /// [Arbitrary Binary Data (BIN)](`FileFormat::ArbitraryBinaryData`).
    #[inline]
    fn default() -> Self {
        Self::ArbitraryBinaryData
    }
}

impl Display for FileFormat {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.display_name())
    }
}

impl From<&[u8]> for FileFormat {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self::from_reader(Cursor::new(value)).unwrap_or_default()
    }
}

/// A kind of [`FileFormat`] according to the media type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Data to be processed by some type of application program.
    Application,
    /// Audio.
    Audio,
    /// Font.
    Font,
    /// One or more individual images.
    Image,
    /// 3D model.
    Model,
    /// Text.
    Text,
    /// Video.
    Video,
}
