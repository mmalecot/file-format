/*!
Crate for determining the file format of a [given file](`FileFormat::from_file`) or stream.

It provides a variety of functions for identifying a wide range of file formats, including
[ZIP](`FileFormat::Zip`), [Compound File Binary (CFB)](`FileFormat::CompoundFileBinary`),
[Extensible Markup Language (XML)](`FileFormat::ExtensibleMarkupLanguage`) and [more](`FileFormat`).

It checks the signature of the file to determine its format and intelligently employs specific
readers when available for accurate identification. If the signature is not recognized, the crate
falls back to the [default](`FileFormat::default`) file format, which is
[Arbitrary Binary Data (BIN)](`FileFormat::ArbitraryBinaryData`).

# Examples

Determines from a file:

```no_run
use file_format::{FileFormat, Kind};

let fmt = FileFormat::from_file("fixtures/document/sample.pdf")?;
assert_eq!(fmt, FileFormat::PortableDocumentFormat);
assert_eq!(fmt.name(), "Portable Document Format");
assert_eq!(fmt.short_name(), Some("PDF"));
assert_eq!(fmt.media_type(), "application/pdf");
assert_eq!(fmt.extension(), "pdf");
assert_eq!(fmt.kind(), Kind::Document);
# Ok::<(), std::io::Error>(())
```

Determines from bytes:

```
use file_format::{FileFormat, Kind};

let fmt = FileFormat::from_bytes(&[0xFF, 0xD8, 0xFF]);
assert_eq!(fmt, FileFormat::JointPhotographicExpertsGroup);
assert_eq!(fmt.name(), "Joint Photographic Experts Group");
assert_eq!(fmt.short_name(), Some("JPEG"));
assert_eq!(fmt.media_type(), "image/jpeg");
assert_eq!(fmt.extension(), "jpg");
assert_eq!(fmt.kind(), Kind::Image);
```

# Crate features

All features below are disabled by default.

## Reader features

These features enable the detection of file formats that require a specific reader for
identification.

- `reader` - Enables all reader features.
- `reader-asf` - Enables [Advanced Systems Format (ASF)](`FileFormat::AdvancedSystemsFormat`) based
  file formats detection.
  * [Microsoft Digital Video Recording (DVR-MS)](`FileFormat::MicrosoftDigitalVideoRecording`)
  * [Windows Media Audio (WMA)](`FileFormat::WindowsMediaAudio`)
  * [Windows Media Video (WMV)](`FileFormat::WindowsMediaVideo`)
- `reader-cfb` - Enables [Compound File Binary (CFB)](`FileFormat::CompoundFileBinary`) based file
  formats detection.
  * [3D Studio Max (MAX)](`FileFormat::ThreeDimensionalStudioMax`)
  * [Autodesk Inventor Assembly (IAM)](`FileFormat::AutodeskInventorAssembly`)
  * [Autodesk Inventor Drawing (IDW)](`FileFormat::AutodeskInventorDrawing`)
  * [Autodesk Inventor Part (IPT)](`FileFormat::AutodeskInventorPart`)
  * [Autodesk Inventor Presentation (IPN)](`FileFormat::AutodeskInventorPresentation`)
  * [Corel Presentations 7 (SHW)](`FileFormat::CorelPresentations7`)
  * [Flash Project (FLA)](`FileFormat::FlashProject`)
  * [Microsoft Excel Spreadsheet (XLS)](`FileFormat::MicrosoftExcelSpreadsheet`)
  * [Microsoft PowerPoint Presentation (PPT)](`FileFormat::MicrosoftPowerpointPresentation`)
  * [Microsoft Project Plan (MPP)](`FileFormat::MicrosoftProjectPlan`)
  * [Microsoft Publisher Document (PUB)](`FileFormat::MicrosoftPublisherDocument`)
  * [Microsoft Software Installer (MSI)](`FileFormat::MicrosoftSoftwareInstaller`)
  * [Microsoft Visio Drawing (VSD)](`FileFormat::MicrosoftVisioDrawing`)
  * [Microsoft Word Document (DOC)](`FileFormat::MicrosoftWordDocument`)
  * [Microsoft Works 6 Spreadsheet (XLR)](`FileFormat::MicrosoftWorks6Spreadsheet`)
  * [Microsoft Works Database (WDB)](`FileFormat::MicrosoftWorksDatabase`)
  * [Microsoft Works Word Processor (WPS)](`FileFormat::MicrosoftWorksWordProcessor`)
  * [SolidWorks Assembly (SLDASM)](`FileFormat::SolidworksAssembly`)
  * [SolidWorks Drawing (SLDDRW)](`FileFormat::SolidworksDrawing`)
  * [SolidWorks Part (SLDPRT)](`FileFormat::SolidworksPart`)
  * [StarCalc (SDC)](`FileFormat::Starcalc`)
  * [StarChart (SDS)](`FileFormat::Starchart`)
  * [StarDraw (SDA)](`FileFormat::Stardraw`)
  * [StarImpress (SDD)](`FileFormat::Starimpress`)
  * [StarMath (SMF)](`FileFormat::Starmath`)
  * [StarWriter (SDW)](`FileFormat::Starwriter`)
  * [WordPerfect Document (WPD)](`FileFormat::WordperfectDocument`)
  * [WordPerfect Graphics (WPG)](`FileFormat::WordperfectGraphics`)
- `reader-ebml` - Enables [Extensible Binary Meta Language (EBML)](`FileFormat::ExtensibleBinaryMetaLanguage`)
  based file formats detection.
  * [Matroska 3D Video (MK3D)](`FileFormat::Matroska3dVideo`)
  * [Matroska Audio (MKA)](`FileFormat::MatroskaAudio`)
  * [Matroska Subtitles (MKS)](`FileFormat::MatroskaSubtitles`)
  * [Matroska Video (MKV)](`FileFormat::MatroskaVideo`)
  * [WebM](`FileFormat::Webm`)
- `reader-exe` - Enables [MS-DOS Executable (EXE)](`FileFormat::MsDosExecutable`) based file formats
  detection.
  * [Dynamic Link Library (DLL)](`FileFormat::DynamicLinkLibrary`)
  * [Linear Executable (LE)](`FileFormat::LinearExecutable`)
  * [New Executable (NE)](`FileFormat::NewExecutable`)
  * [Portable Executable (PE)](`FileFormat::PortableExecutable`)
- `reader-mp4` - Enables [MPEG-4 Part 14 (MP4)](`FileFormat::Mpeg4Part14`) based file formats
  detection.
  * [MPEG-4 Part 14 Audio (MP4)](`FileFormat::Mpeg4Part14Audio`)
  * [MPEG-4 Part 14 Subtitles (MP4)](`FileFormat::Mpeg4Part14Subtitles`)
  * [MPEG-4 Part 14 Video (MP4)](`FileFormat::Mpeg4Part14Video`)
- `reader-pdf` - Enables [Portable Document Format (PDF)](`FileFormat::PortableDocumentFormat`)
  based file formats detection.
  * [Adobe Illustrator Artwork (AI)](`FileFormat::AdobeIllustratorArtwork`)
- `reader-rm` - Enables [RealMedia (RM)](`FileFormat::Realmedia`) based file formats detection.
  * [RealAudio (RA)](`FileFormat::Realaudio`)
  * [RealVideo (RV)](`FileFormat::Realvideo`)
- `reader-txt` - Enables [Plain Text (TXT)](`FileFormat::PlainText`) detection when the file format
  is not recognized by its signature. Please note that this feature only detects files containing
  ASCII/UTF-8-encoded text.
- `reader-xml` - Enables [Extensible Markup Language (XML)](`FileFormat::ExtensibleMarkupLanguage`)
  based file formats detection. Please note that these file formats may be detected without the
  feature in certain cases.
  * [AbiWord (ABW)](`FileFormat::Abiword`)
  * [AbiWord Template (AWT)](`FileFormat::AbiwordTemplate`)
  * [Additive Manufacturing Format (AMF)](`FileFormat::AdditiveManufacturingFormat`)
  * [Advanced Stream Redirector (ASX)](`FileFormat::AdvancedStreamRedirector`)
  * [Atom](`FileFormat::Atom`)
  * [Collaborative Design Activity (COLLADA)](`FileFormat::CollaborativeDesignActivity`)
  * [Extensible 3D (X3D)](`FileFormat::Extensible3d`)
  * [Extensible Stylesheet Language Transformations (XSLT)](`FileFormat::ExtensibleStylesheetLanguageTransformations`)
  * [FictionBook (FB2)](`FileFormat::Fictionbook`)
  * [GPS Exchange Format (GPX)](`FileFormat::GpsExchangeFormat`)
  * [Geography Markup Language (GML)](`FileFormat::GeographyMarkupLanguage`)
  * [Keyhole Markup Language (KML)](`FileFormat::KeyholeMarkupLanguage`)
  * [MPEG-DASH MPD (MPD)](`FileFormat::MpegDashMpd`)
  * [Mathematical Markup Language (MathML)](`FileFormat::MathematicalMarkupLanguage`)
  * [MusicXML](`FileFormat::Musicxml`)
  * [Really Simple Syndication (RSS)](`FileFormat::ReallySimpleSyndication`)
  * [Scalable Vector Graphics (SVG)](`FileFormat::ScalableVectorGraphics`)
  * [Simple Object Access Protocol (SOAP)](`FileFormat::SimpleObjectAccessProtocol`)
  * [Tiled Map XML (TMX)](`FileFormat::TiledMapXml`)
  * [Tiled Tileset XML (TSX)](`FileFormat::TiledTilesetXml`)
  * [Timed Text Markup Language (TTML)](`FileFormat::TimedTextMarkupLanguage`)
  * [Training Center XML (TCX)](`FileFormat::TrainingCenterXml`)
  * [Uniform Office Format Presentation (UOP)](`FileFormat::UniformOfficeFormatPresentation`)
  * [Uniform Office Format Spreadsheet (UOS)](`FileFormat::UniformOfficeFormatSpreadsheet`)
  * [Uniform Office Format Text (UOT)](`FileFormat::UniformOfficeFormatText`)
  * [Universal Subtitle Format (USF)](`FileFormat::UniversalSubtitleFormat`)
  * [XML Localization Interchange File Format (XLIFF)](`FileFormat::XmlLocalizationInterchangeFileFormat`)
  * [XML Shareable Playlist Format (XSPF)](`FileFormat::XmlShareablePlaylistFormat`)
  * [draw.io (DRAWIO)](`FileFormat::Drawio`)
- `reader-zip` - Enables [ZIP](`FileFormat::Zip`)-based file formats detection.
  * [3D Manufacturing Format (3MF)](`FileFormat::ThreeDimensionalManufacturingFormat`)
  * [Adobe Integrated Runtime (AIR)](`FileFormat::AdobeIntegratedRuntime`)
  * [Android App Bundle (AAB)](`FileFormat::AndroidAppBundle`)
  * [Android Package (APK)](`FileFormat::AndroidPackage`)
  * [Autodesk 123D (123DX)](`FileFormat::Autodesk123d`)
  * [Circuit Diagram Document (CDDX)](`FileFormat::CircuitDiagramDocument`)
  * [Design Web Format XPS (DWFX)](`FileFormat::DesignWebFormatXps`)
  * [Electronic Publication (EPUB)](`FileFormat::ElectronicPublication`)
  * [Enterprise Application Archive (EAR)](`FileFormat::EnterpriseApplicationArchive`)
  * [FictionBook ZIP (FBZ)](`FileFormat::FictionbookZip`)
  * [Flash CS5 Project (FLA)](`FileFormat::FlashCs5Project`)
  * [Fusion 360 (F3D)](`FileFormat::Fusion360`)
  * [InDesign Markup Language (IDML)](`FileFormat::IndesignMarkupLanguage`)
  * [Java Archive (JAR)](`FileFormat::JavaArchive`)
  * [Keyhole Markup Language ZIP (KMZ)](`FileFormat::KeyholeMarkupLanguageZip`)
  * [Microsoft Visual Studio Extension (VSIX)](`FileFormat::MicrosoftVisualStudioExtension`)
  * [MusicXML ZIP (MXL)](`FileFormat::MusicxmlZip`)
  * [Office Open XML Document (DOCX)](`FileFormat::OfficeOpenXmlDocument`)
  * [Office Open XML Drawing (VSDX)](`FileFormat::OfficeOpenXmlDrawing`)
  * [Office Open XML Presentation (PPTX)](`FileFormat::OfficeOpenXmlPresentation`)
  * [Office Open XML Spreadsheet (XLSX)](`FileFormat::OfficeOpenXmlSpreadsheet`)
  * [OpenDocument Database (ODB)](`FileFormat::OpendocumentDatabase`)
  * [OpenDocument Formula (ODF)](`FileFormat::OpendocumentFormula`)
  * [OpenDocument Formula Template (OTF)](`FileFormat::OpendocumentFormulaTemplate`)
  * [OpenDocument Graphics (ODG)](`FileFormat::OpendocumentGraphics`)
  * [OpenDocument Graphics Template (OTG)](`FileFormat::OpendocumentGraphicsTemplate`)
  * [OpenDocument Presentation (ODP)](`FileFormat::OpendocumentPresentation`)
  * [OpenDocument Presentation Template (OTP)](`FileFormat::OpendocumentPresentationTemplate`)
  * [OpenDocument Spreadsheet (ODS)](`FileFormat::OpendocumentSpreadsheet`)
  * [OpenDocument Spreadsheet Template (OTS)](`FileFormat::OpendocumentSpreadsheetTemplate`)
  * [OpenDocument Text (ODT)](`FileFormat::OpendocumentText`)
  * [OpenDocument Text Master (ODM)](`FileFormat::OpendocumentTextMaster`)
  * [OpenDocument Text Master Template (OTM)](`FileFormat::OpendocumentTextMasterTemplate`)
  * [OpenDocument Text Template (OTT)](`FileFormat::OpendocumentTextTemplate`)
  * [OpenRaster (ORA)](`FileFormat::Openraster`)
  * [OpenXPS (OXPS)](`FileFormat::Openxps`)
  * [SpaceClaim Document (SCDOC)](`FileFormat::SpaceclaimDocument`)
  * [Sun XML Calc (SXC)](`FileFormat::SunXmlCalc`)
  * [Sun XML Calc Template (STC)](`FileFormat::SunXmlCalcTemplate`)
  * [Sun XML Draw (SXD)](`FileFormat::SunXmlDraw`)
  * [Sun XML Draw Template (STD)](`FileFormat::SunXmlDrawTemplate`)
  * [Sun XML Impress (SXI)](`FileFormat::SunXmlImpress`)
  * [Sun XML Impress Template (STI)](`FileFormat::SunXmlImpressTemplate`)
  * [Sun XML Math (SXM)](`FileFormat::SunXmlMath`)
  * [Sun XML Writer (SXW)](`FileFormat::SunXmlWriter`)
  * [Sun XML Writer Global (SGW)](`FileFormat::SunXmlWriterGlobal`)
  * [Sun XML Writer Template (STW)](`FileFormat::SunXmlWriterTemplate`)
  * [Universal Scene Description ZIP (USDZ)](`FileFormat::UniversalSceneDescriptionZip`)
  * [Web Application Archive (WAR)](`FileFormat::WebApplicationArchive`)
  * [Windows App Bundle (APPXBUNDLE)](`FileFormat::WindowsAppBundle`)
  * [Windows App Package (APPX)](`FileFormat::WindowsAppPackage`)
  * [XAP](`FileFormat::Xap`)
  * [XPInstall (XPI)](`FileFormat::Xpinstall`)
  * [iOS App Store Package (IPA)](`FileFormat::IosAppStorePackage`)
*/

#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[macro_use]
mod macros;

mod formats;
mod readers;
mod signatures;

use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{Cursor, Read, Result, Seek},
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
    /// ```
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_bytes(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");
    /// assert_eq!(fmt, FileFormat::PortableNetworkGraphics);
    ///```
    ///
    /// Detects from a zeroed buffer:
    ///
    /// ```
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_bytes(&[0; 1000]);
    /// assert_eq!(fmt, FileFormat::ArbitraryBinaryData);
    ///```
    ///
    /// [default value]: FileFormat::default
    #[inline]
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Self {
        Self::from(bytes.as_ref())
    }

    /// Determines file format from a file.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_file("fixtures/video/sample.avi")?;
    /// assert_eq!(fmt, FileFormat::AudioVideoInterleave);
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
    /// Basic usage:
    ///
    /// ```
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_reader(std::io::empty())?;
    /// assert_eq!(fmt, FileFormat::Empty);
    /// # Ok::<(), std::io::Error>(())
    ///```
    pub fn from_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // Creates and fills a buffer.
        let mut buf = [0; 36_870];
        let nread = reader.read(&mut buf)?;

        // Determines file format.
        Ok(if nread == 0 {
            Self::Empty
        } else if let Some(fmt) = Self::from_signature(&buf[..nread]) {
            Self::from_fmt_reader(fmt, &mut reader)
                .unwrap_or_else(|_| Self::from_generic_reader(&mut reader))
        } else {
            Self::from_generic_reader(&mut reader)
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
        write!(formatter, "{}", self.name())
    }
}

impl From<&[u8]> for FileFormat {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self::from_reader(Cursor::new(value)).unwrap_or_default()
    }
}

/// A kind of file format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Files and directories stored in a single, possibly compressed, archive.
    Archive,
    /// Musics, sound effects, and spoken audio recordings.
    Audio,
    /// Compressed single files or streams.
    Compressed,
    /// Organized collections of data.
    Database,
    /// Visual information using graphics and spatial relationships.
    Diagram,
    /// Floppy disk images, optical disc images and virtual machine disks.
    Disk,
    /// Word processing and desktop publishing documents.
    Document,
    /// Electronic books.
    Ebook,
    /// Machine-executable code, virtual machine code and shared libraries.
    Executable,
    /// Typefaces used for displaying text on screen or in print.
    Font,
    /// Mathematical formulas.
    Formula,
    /// Collections of geospatial features, GPS tracks and other location-related files.
    Geospatial,
    /// Animated images, icons, cursors, raster graphics and vector graphics.
    Image,
    /// Data that provides information about other data.
    Metadata,
    /// 3D models, CAD drawings, and other types of files used for creating or displaying 3D images.
    Model,
    /// Data which do not fit in any of the other kinds.
    Other,
    /// Collections of files bundled together for software distribution.
    Package,
    /// Lists of audio or video files, organized in a specific order for sequential playback.
    Playlist,
    /// Slide shows.
    Presentation,
    /// Copies of a read-only memory chip of computers, cartridges, or other electronic devices.
    Rom,
    /// Data in tabular form.
    Spreadsheet,
    /// Subtitles and captions.
    Subtitle,
    /// Moving images, possibly with color and coordinated sound.
    Video,
}
