/*!
Crate for determining the file format of a given file or stream.

It provides a variety of functions for identifying a wide range of file formats, including
[ZIP](`FileFormat::Zip`), [CFB](`FileFormat::CompoundFileBinary`),
[XML](`FileFormat::ExtensibleMarkupLanguage`) and [more](`FileFormat`).

It checks the signature of the file to determine its format. If the file format is not recognized by
its signature, it checks if it is [Plain Text](`FileFormat::PlainText`). Otherwise, it returns the
default file format which is [Arbitrary Binary Data](`FileFormat::ArbitraryBinaryData`).

## Examples

Determines from a file:

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_file("fixtures/application/sample.zip")?;
assert_eq!(format, FileFormat::Zip);
assert_eq!(format.name(), "ZIP");
assert_eq!(format.media_type(), "application/zip");
assert_eq!(format.extension(), "zip");
assert_eq!(format.kind(), Kind::Application);
# Ok::<(), std::io::Error>(())
```

Determines from bytes:

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_bytes(&[0xFF, 0xD8, 0xFF, 0xEE]);
assert_eq!(format, FileFormat::JointPhotographicExpertsGroup);
assert_eq!(format.name(), "Joint Photographic Experts Group");
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.extension(), "jpg");
assert_eq!(format.kind(), Kind::Image);
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
file-format = "0.12"
```

## Features

- `accuracy` - Improves the accuracy but may increase the processing time and memory usage.
- `cfb` - Enables [CFB](`FileFormat::CompoundFileBinary`)-based formats support:
    - [Microsoft Excel Spreadsheet](`FileFormat::MicrosoftExcelSpreadsheet`)
    - [Microsoft PowerPoint Presentation](`FileFormat::MicrosoftPowerpointPresentation`)
    - [Microsoft Project Plan](`FileFormat::MicrosoftProjectPlan`)
    - [Microsoft Publisher Document](`FileFormat::MicrosoftPublisherDocument`)
    - [Microsoft Software Installer](`FileFormat::MicrosoftSoftwareInstaller`)
    - [Microsoft Visio Drawing](`FileFormat::MicrosoftVisioDrawing`)
    - [Microsoft Word Document](`FileFormat::MicrosoftWordDocument`)
- `zip` - Enables [ZIP](`FileFormat::Zip`)-based formats support:
    - [3D Manufacturing Format](`FileFormat::ThreeDimensionalManufacturingFormat`)
    - [Android Package](`FileFormat::AndroidPackage`)
    - [Circuit Diagram Document](`FileFormat::CircuitDiagramDocument`)
    - [Design Web Format XPS](`FileFormat::DesignWebFormatXps`)
    - [Electronic Publication](`FileFormat::ElectronicPublication`)
    - [Enterprise Application Archive](`FileFormat::EnterpriseApplicationArchive`)
    - [Java Archive](`FileFormat::JavaArchive`)
    - [Keyhole Markup Language Zipped](`FileFormat::KeyholeMarkupLanguageZipped`)
    - [Microsoft Visual Studio Extension](`FileFormat::MicrosoftVisualStudioExtension`)
    - [MusicXML Zipped](`FileFormat::MusicxmlZipped`)
    - [Office Open XML Document](`FileFormat::OfficeOpenXmlDocument`)
    - [Office Open XML Drawing](`FileFormat::OfficeOpenXmlDrawing`)
    - [Office Open XML Presentation](`FileFormat::OfficeOpenXmlPresentation`)
    - [Office Open XML Spreadsheet](`FileFormat::OfficeOpenXmlSpreadsheet`)
    - [OpenDocument Graphics](`FileFormat::OpenDocumentGraphics`)
    - [OpenDocument Presentation](`FileFormat::OpenDocumentPresentation`)
    - [OpenDocument Spreadsheet](`FileFormat::OpenDocumentSpreadsheet`)
    - [OpenDocument Text](`FileFormat::OpenDocumentText`)
    - [OpenRaster](`FileFormat::Openraster`)
    - [Web Application Archive](`FileFormat::WebApplicationArchive`)
    - [Windows App Package](`FileFormat::WindowsAppPackage`)
    - [XAP](`FileFormat::Xap`)
    - [XPInstall](`FileFormat::Xpinstall`)
    - [iOS App Store Package](`FileFormat::IosAppStorePackage`)

All these features are disabled by default.
*/

#![deny(missing_docs)]

use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Result, Seek},
    path::Path,
    str,
};

#[macro_use]
mod macros;

mod read;
mod signatures;

formats! {
    format = AdaptiveMultiRate
    name = "Adaptive Multi-Rate"
    media_type = "audio/amr"
    extension = "amr"
    kind = Audio

    format = AdobeFlashPlayerAudio
    name = "Adobe Flash Player Audio"
    media_type = "audio/mp4"
    extension = "f4a"
    kind = Audio

    format = AdobeFlashPlayerAudiobook
    name = "Adobe Flash Player Audiobook"
    media_type = "audio/mp4"
    extension = "f4b"
    kind = Audio

    format = AdobeFlashPlayerProtectedVideo
    name = "Adobe Flash Player Protected Video"
    media_type = "video/mp4"
    extension = "f4p"
    kind = Video

    format = AdobeFlashPlayerVideo
    name = "Adobe Flash Player Video"
    media_type = "video/mp4"
    extension = "f4v"
    kind = Video

    format = AdobeIllustratorArtwork
    name = "Adobe Illustrator Artwork"
    media_type = "application/pdf"
    extension = "ai"
    kind = Application

    format = AdobeIndesignDocument
    name = "Adobe InDesign Document"
    media_type = "application/x-indesign"
    extension = "indd"
    kind = Application

    format = AdobePhotoshopDocument
    name = "Adobe Photoshop Document"
    media_type = "image/vnd.adobe.photoshop"
    extension = "psd"
    kind = Image

    format = AdvancedAudioCoding
    name = "Advanced Audio Coding"
    media_type = "audio/aac"
    extension = "aac"
    kind = Audio

    format = Alz
    name = "ALZ"
    media_type = "application/x-alz-compressed"
    extension = "alz"
    kind = Application

    format = AndroidBinaryXml
    name = "Android Binary XML"
    media_type = "application/vnd.android.axml"
    extension = "xml"
    kind = Application

    format = AndroidCompiledResources
    name = "Android Compiled Resources"
    media_type = "application/vnd.android.arsc"
    extension = "arsc"
    kind = Application

    format = AndroidPackage
    name = "Android Package"
    media_type = "application/vnd.android.package-archive"
    extension = "apk"
    kind = Application

    format = AnimatedPortableNetworkGraphics
    name = "Animated Portable Network Graphics"
    media_type = "image/apng"
    extension = "apng"
    kind = Image

    format = ApacheArrowColumnar
    name = "Apache Arrow Columnar"
    media_type = "application/x-apache-arrow"
    extension = "arrow"
    kind = Application

    format = AppleDiskImage
    name = "Apple Disk Image"
    media_type = "application/x-apple-diskimage"
    extension = "dmg"
    kind = Application

    format = AppleIconImage
    name = "Apple Icon Image"
    media_type = "image/x-icns"
    extension = "icns"
    kind = Image

    format = AppleItunesAudio
    name = "Apple iTunes Audio"
    media_type = "audio/x-m4a"
    extension = "m4a"
    kind = Audio

    format = AppleItunesAudiobook
    name = "Apple iTunes Audiobook"
    media_type = "audio/mp4"
    extension = "m4b"
    kind = Audio

    format = AppleItunesProtectedAudio
    name = "Apple iTunes Protected Audio"
    media_type = "audio/mp4"
    extension = "m4p"
    kind = Audio

    format = AppleItunesVideo
    name = "Apple iTunes Video"
    media_type = "video/x-m4v"
    extension = "m4v"
    kind = Video

    format = AppleQuicktime
    name = "Apple QuickTime"
    media_type = "video/quicktime"
    extension = "mov"
    kind = Video

    format = ArbitraryBinaryData
    name = "Arbitrary Binary Data"
    media_type = "application/octet-stream"
    extension = "bin"
    kind = Application

    format = ArchivedByRobertJung
    name = "Archived by Robert Jung"
    media_type = "application/x-arj"
    extension = "arj"
    kind = Application

    format = Au
    name = "Au"
    media_type = "audio/basic"
    extension = "au"
    kind = Audio

    format = AudioCodec3
    name = "Audio Codec 3"
    media_type = "audio/vnd.dolby.dd-raw"
    extension = "ac3"
    kind = Audio

    format = AudioInterchangeFileFormat
    name = "Audio Interchange File Format"
    media_type = "audio/aiff"
    extension = "aiff"
    kind = Audio

    format = AudioVideoInterleave
    name = "Audio Video Interleave"
    media_type = "video/avi"
    extension = "avi"
    kind = Video

    format = Av1ImageFileFormat
    name = "AV1 Image File Format"
    media_type = "image/avif"
    extension = "avif"
    kind = Image

    format = Av1ImageFileFormatSequence
    name = "AV1 Image File Format Sequence"
    media_type = "image/avif-sequence"
    extension = "avifs"
    kind = Image

    format = BetterPortableGraphics
    name = "Better Portable Graphics"
    media_type = "image/bpg"
    extension = "bpg"
    kind = Image

    format = Blender
    name = "Blender"
    media_type = "application/x-blender"
    extension = "blend"
    kind = Application

    format = Bzip2
    name = "bzip2"
    media_type = "application/x-bzip2"
    extension = "bz2"
    kind = Application

    format = Cabinet
    name = "Cabinet"
    media_type = "application/vnd.ms-cab-compressed"
    extension = "cab"
    kind = Application

    format = CanonRaw2
    name = "Canon Raw 2"
    media_type = "image/x-canon-cr2"
    extension = "cr2"
    kind = Image

    format = CanonRaw3
    name = "Canon Raw 3"
    media_type = "image/x-canon-cr3"
    extension = "cr3"
    kind = Image

    format = Cineon
    name = "Cineon"
    media_type = "image/cineon"
    extension = "cin"
    kind = Image

    format = CircuitDiagramDocument
    name = "Circuit Diagram Document"
    media_type = "application/vnd.circuitdiagram.document.main+xml"
    extension = "cddx"
    kind = Application

    format = ClojureScript
    name = "Clojure Script"
    media_type = "text/x-clojure"
    extension = "clj"
    kind = Text

    format = CommonObjectFileFormat
    name = "Common Object File Format"
    media_type = "application/x-coff"
    extension = "coff"
    kind = Application

    format = CompoundFileBinary
    name = "Compound File Binary"
    media_type = "application/x-cfb"
    extension = "cfb"
    kind = Application

    format = Cpio
    name = "cpio"
    media_type = "application/x-cpio"
    extension = "cpio"
    kind = Application

    format = CreativeVoice
    name = "Creative Voice"
    media_type = "audio/x-voc"
    extension = "voc"
    kind = Audio

    format = DalvikExecutable
    name = "Dalvik Executable"
    media_type = "application/vnd.android.dex"
    extension = "dex"
    kind = Application

    format = DebianBinaryPackage
    name = "Debian Binary Package"
    media_type = "application/vnd.debian.binary-package"
    extension = "deb"
    kind = Application

    format = DerCertificate
    name = "DER Certificate"
    media_type = "application/x-x509-ca-cert"
    extension = "der"
    kind = Application

    format = DesignWebFormatXps
    name = "Design Web Format XPS"
    media_type = "model/vnd.dwfx+xps"
    extension = "dwfx"
    kind = Model

    format = DigitalImagingAndCommunicationsInMedicine
    name = "Digital Imaging and Communications in Medicine"
    media_type = "application/dicom"
    extension = "dcm"
    kind = Application

    format = DigitalPictureExchange
    name = "Digital Picture Exchange"
    media_type = "image/x-dpx"
    extension = "dpx"
    kind = Image

    format = Djvu
    name = "DjVu"
    media_type = "image/vnd.djvu"
    extension = "djvu"
    kind = Image

    format = DynamicLinkLibrary
    name = "Dynamic Link Library"
    media_type = "application/vnd.microsoft.portable-executable"
    extension = "dll"
    kind = Application

    format = ElectronicPublication
    name = "Electronic Publication"
    media_type = "application/epub+zip"
    extension = "epub"
    kind = Application

    format = EmbeddedOpentype
    name = "Embedded OpenType"
    media_type = "application/vnd.ms-fontobject"
    extension = "eot"
    kind = Application

    format = EncapsulatedPostscript
    name = "Encapsulated PostScript"
    media_type = "application/eps"
    extension = "eps"
    kind = Application

    format = EnterpriseApplicationArchive
    name = "Enterprise Application Archive"
    media_type = "application/java-archive"
    extension = "ear"
    kind = Application

    format = ExecutableAndLinkableFormat
    name = "Executable and Linkable Format"
    media_type = "application/x-executable"
    extension = "elf"
    kind = Application

    format = ExperimentalComputingFacility
    name = "Experimental Computing Facility"
    media_type = "image/x-xcf"
    extension = "xcf"
    kind = Image

    format = ExtensibleArchive
    name = "Extensible Archive"
    media_type = "application/x-xar"
    extension = "xar"
    kind = Application

    format = ExtensibleMarkupLanguage
    name = "Extensible Markup Language"
    media_type = "text/xml"
    extension = "xml"
    kind = Text

    format = ExtensibleStylesheetLanguageTransformations
    name = "Extensible Stylesheet Language Transformations"
    media_type = "application/xslt+xml"
    extension = "xsl"
    kind = Application

    format = FastTracker2ExtendedModule
    name = "FastTracker 2 Extended Module"
    media_type = "audio/x-xm"
    extension = "xm"
    kind = Audio

    format = FlashVideo
    name = "Flash Video"
    media_type = "video/x-flv"
    extension = "flv"
    kind = Video

    format = FlexibleImageTransportSystem
    name = "Flexible Image Transport System"
    media_type = "image/fits"
    extension = "fits"
    kind = Image

    format = FreeLosslessAudioCodec
    name = "Free Lossless Audio Codec"
    media_type = "audio/x-flac"
    extension = "flac"
    kind = Audio

    format = FreeLosslessImageFormat
    name = "Free Lossless Image Format"
    media_type = "image/flif"
    extension = "flif"
    kind = Image

    format = FujifilmRaw
    name = "Fujifilm Raw"
    media_type = "image/x-fuji-raf"
    extension = "raf"
    kind = Image

    format = GameBoyAdvanceRom
    name = "Game Boy Advance ROM"
    media_type = "application/x-gba-rom"
    extension = "gba"
    kind = Application

    format = GameBoyColorRom
    name = "Game Boy Color ROM"
    media_type = "application/x-gameboy-color-rom"
    extension = "gbc"
    kind = Application

    format = GameBoyRom
    name = "Game Boy ROM"
    media_type = "application/x-gameboy-rom"
    extension = "gb"
    kind = Application

    format = GeographyMarkupLanguage
    name = "Geography Markup Language"
    media_type = "application/gml+xml"
    extension = "gml"
    kind = Application

    format = GlTransmissionFormatBinary
    name = "GL Transmission Format Binary"
    media_type = "model/gltf-binary"
    extension = "glb"
    kind = Model

    format = GoogleChromeExtension
    name = "Google Chrome Extension"
    media_type = "application/x-google-chrome-extension"
    extension = "crx"
    kind = Application

    format = GoogleDraco
    name = "Google Draco"
    media_type = "model/x-draco"
    extension = "drc"
    kind = Model

    format = GraphicsInterchangeFormat
    name = "Graphics Interchange Format"
    media_type = "image/gif"
    extension = "gif"
    kind = Image

    format = Gzip
    name = "gzip"
    media_type = "application/gzip"
    extension = "gz"
    kind = Application

    format = HighEfficiencyImageCoding
    name = "High Efficiency Image Coding"
    media_type = "image/heic"
    extension = "heic"
    kind = Image

    format = HighEfficiencyImageCodingSequence
    name = "High Efficiency Image Coding Sequence"
    media_type = "image/heic-sequence"
    extension = "heics"
    kind = Image

    format = HighEfficiencyImageFileFormat
    name = "High Efficiency Image File Format"
    media_type = "image/heif"
    extension = "heif"
    kind = Image

    format = HighEfficiencyImageFileFormatSequence
    name = "High Efficiency Image File Format Sequence"
    media_type = "image/heif-sequence"
    extension = "heifs"
    kind = Image

    format = HypertextMarkupLanguage
    name = "HyperText Markup Language"
    media_type = "text/html"
    extension = "html"
    kind = Text

    format = IccProfile
    name = "ICC Profile"
    media_type = "application/vnd.iccprofile"
    extension = "icc"
    kind = Application

    format = ImpulseTrackerModule
    name = "Impulse Tracker Module"
    media_type = "audio/x-it"
    extension = "it"
    kind = Audio

    format = IosAppStorePackage
    name = "iOS App Store Package"
    media_type = "application/x-ios-app"
    extension = "ipa"
    kind = Application

    format = Iso9660
    name = "ISO 9660"
    media_type = "application/x-iso9660-image"
    extension = "iso"
    kind = Application

    format = JavaArchive
    name = "Java Archive"
    media_type = "application/java-archive"
    extension = "jar"
    kind = Application

    format = JavaClass
    name = "Java Class"
    media_type = "application/java-vm"
    extension = "class"
    kind = Application

    format = JavaKeystore
    name = "Java KeyStore"
    media_type = "application/x-java-keystore"
    extension = "jks"
    kind = Application

    format = JointPhotographicExpertsGroup
    name = "Joint Photographic Experts Group"
    media_type = "image/jpeg"
    extension = "jpg"
    kind = Image

    format = Jpeg2000Part1
    name = "JPEG 2000 Part 1"
    media_type = "image/jp2"
    extension = "jp2"
    kind = Image

    format = Jpeg2000Part2
    name = "JPEG 2000 Part 2"
    media_type = "image/jpx"
    extension = "jpx"
    kind = Image

    format = Jpeg2000Part3
    name = "JPEG 2000 Part 3"
    media_type = "image/mj2"
    extension = "mj2"
    kind = Image

    format = Jpeg2000Part6
    name = "JPEG 2000 Part 6"
    media_type = "image/jpm"
    extension = "jpm"
    kind = Image

    format = JpegExtendedRange
    name = "PEG Extended Range"
    media_type = "image/jxr"
    extension = "jxr"
    kind = Image

    format = JpegXl
    name = "JPEG XL"
    media_type = "image/jxl"
    extension = "jxl"
    kind = Image

    format = KeyholeMarkupLanguage
    name = "Keyhole Markup Language"
    media_type = "application/vnd.google-earth.kml+xml"
    extension = "kml"
    kind = Application

    format = KeyholeMarkupLanguageZipped
    name = "Keyhole Markup Language Zipped"
    media_type = "application/vnd.google-earth.kmz"
    extension = "kmz"
    kind = Application

    format = KhronosTexture
    name = "Khronos Texture"
    media_type = "image/ktx"
    extension = "ktx"
    kind = Image

    format = KhronosTexture2
    name = "Khronos Texture 2"
    media_type = "image/ktx2"
    extension = "ktx2"
    kind = Image

    format = Latex
    name = "LaTeX"
    media_type = "text/x-tex"
    extension = "tex"
    kind = Text

    format = LempelZivFiniteStateEntropy
    name = "Lempel–Ziv Finite State Entropy"
    media_type = "application/x-lzfse"
    extension = "lzfse"
    kind = Application

    format = Lha
    name = "LHA"
    media_type = "application/x-lzh-compressed"
    extension = "lzh"
    kind = Application

    format = LlvmBitcode
    name = "LLVM Bitcode"
    media_type = "application/x-llvm"
    extension = "bc"
    kind = Application

    format = LongRangeZip
    name = "Long Range ZIP"
    media_type = "application/x-lrzip"
    extension = "lrz"
    kind = Application

    format = LuaBytecode
    name = "Lua Bytecode"
    media_type = "application/x-lua-bytecode"
    extension = "luac"
    kind = Application

    format = LuaScript
    name = "Lua Script"
    media_type = "text/x-lua"
    extension = "lua"
    kind = Text

    format = Lz4
    name = "LZ4"
    media_type = "application/x-lz4"
    extension = "lz4"
    kind = Application

    format = Lzip
    name = "lzip"
    media_type = "application/x-lzip"
    extension = "lz"
    kind = Application

    format = Lzop
    name = "lzop"
    media_type = "application/x-lzop"
    extension = "lzo"
    kind = Application

    format = MachO
    name = "Mach-O"
    media_type = "application/x-mach-binary"
    extension = "mach"
    kind = Application

    format = MacosAlias
    name = "macOS Alias"
    media_type = "application/x-apple-alias"
    extension = "alias"
    kind = Application

    format = MaterialExchangeFormat
    name = "Material Exchange Format"
    media_type = "application/mxf"
    extension = "mxf"
    kind = Application

    format = MatroskaVideo
    name = "Matroska Video"
    media_type = "video/x-matroska"
    extension = "mkv"
    kind = Video

    format = MetaInformationEncapsulation
    name = "Meta Information Encapsulation"
    media_type = "application/x-mie"
    extension = "mie"
    kind = Application

    format = MicrosoftAccess2007Database
    name = "Microsoft Access 2007 Database"
    media_type = "application/x-msaccess"
    extension = "accdb"
    kind = Application

    format = MicrosoftAccessDatabase
    name = "Microsoft Access Database"
    media_type = "application/x-msaccess"
    extension = "mdb"
    kind = Application

    format = MicrosoftCompiledHtmlHelp
    name = "Microsoft Compiled HTML Help"
    media_type = "application/vnd.ms-htmlhelp"
    extension = "chm"
    kind = Application

    format = MicrosoftDirectDrawSurface
    name = "Microsoft DirectDraw Surface"
    media_type = "image/vnd-ms.dds"
    extension = "dds"
    kind = Image

    format = MicrosoftExcelSpreadsheet
    name = "Microsoft Excel Spreadsheet"
    media_type = "application/vnd.ms-excel"
    extension = "xls"
    kind = Application

    format = MicrosoftPowerpointPresentation
    name = "Microsoft PowerPoint Presentation"
    media_type = "application/vnd.ms-powerpoint"
    extension = "ppt"
    kind = Application

    format = MicrosoftProjectPlan
    name = "Microsoft Project Plan"
    media_type = "application/vnd.ms-project"
    extension = "mpp"
    kind = Application

    format = MicrosoftPublisherDocument
    name = "Microsoft Publisher Document"
    media_type = "application/vnd.ms-publisher"
    extension = "pub"
    kind = Application

    format = MicrosoftSoftwareInstaller
    name = "Microsoft Software Installer"
    media_type = "application/x-msi"
    extension = "msi"
    kind = Application

    format = MicrosoftVirtualHardDisk
    name = "Microsoft Virtual Hard Disk"
    media_type = "application/x-vhd"
    extension = "vhd"
    kind = Application

    format = MicrosoftVirtualHardDisk2
    name = "Microsoft Virtual Hard Disk 2"
    media_type = "application/x-vhdx"
    extension = "vhdx"
    kind = Application

    format = MicrosoftVisioDrawing
    name = "Microsoft Visio Drawing"
    media_type = "application/vnd.visio"
    extension = "vsd"
    kind = Application

    format = MicrosoftVisualStudioExtension
    name = "Microsoft Visual Studio Extension"
    media_type = "application/vsix"
    extension = "vsix"
    kind = Application

    format = MicrosoftWordDocument
    name = "Microsoft Word Document"
    media_type = "application/msword"
    extension = "doc"
    kind = Application

    format = Mobipocket
    name = "Mobipocket"
    media_type = "application/x-mobipocket-ebook"
    extension = "mobi"
    kind = Application

    format = MonkeysAudio
    name = "Monkey's Audio"
    media_type = "audio/x-ape"
    extension = "ape"
    kind = Audio

    format = Mpeg12AudioLayer3
    name = "MPEG-1/2 Audio Layer 3"
    media_type = "audio/mpeg"
    extension = "mp3"
    kind = Audio

    format = Mpeg1AudioLayer1
    name = "MPEG-1 Audio Layer 1"
    media_type = "audio/mpeg"
    extension = "mp1"
    kind = Audio

    format = Mpeg1AudioLayer2
    name = "MPEG-1 Audio Layer 2"
    media_type = "audio/mpeg"
    extension = "mp2"
    kind = Audio

    format = Mpeg1Video
    name = "MPEG-1 Video"
    media_type = "video/mpeg"
    extension = "mpg"
    kind = Video

    format = Mpeg2TransportStream
    name = "MPEG-2 Transport Stream"
    media_type = "video/mp2t"
    extension = "mts"
    kind = Video

    format = Mpeg4Part14Video
    name = "MPEG-4 Part 14 Video"
    media_type = "video/mp4"
    extension = "mp4"
    kind = Video

    format = MsDosExecutable
    name = "MS-DOS Executable"
    media_type = "application/x-dosexec"
    extension = "exe"
    kind = Application

    format = Musepack
    name = "Musepack"
    media_type = "audio/x-musepack"
    extension = "mpc"
    kind = Audio

    format = MusicalInstrumentDigitalInterface
    name = "Musical Instrument Digital Interface"
    media_type = "audio/midi"
    extension = "mid"
    kind = Audio

    format = Musicxml
    name = "MusicXML"
    media_type = "application/vnd.recordare.musicxml+xml"
    extension = "musicxml"
    kind = Application

    format = MusicxmlZipped
    name = "MusicXML Zipped"
    media_type = "application/vnd.recordare.musicxml"
    extension = "mxl"
    kind = Application

    format = NikonElectronicFile
    name = "Nikon Electronic File"
    media_type = "image/x-nikon-nef"
    extension = "nef"
    kind = Image

    format = Nintendo64Rom
    name = "Nintendo 64 ROM"
    media_type = "application/x-n64-rom"
    extension = "z64"
    kind = Application

    format = NintendoDsRom
    name = "Nintendo DS ROM"
    media_type = "application/x-nintendo-ds-rom"
    extension = "nds"
    kind = Application

    format = NintendoEntertainmentSystemRom
    name = "Nintendo Entertainment System ROM"
    media_type = "application/x-nintendo-nes-rom"
    extension = "nes"
    kind = Application

    format = OfficeOpenXmlDocument
    name = "Office Open XML Document"
    media_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    extension = "docx"
    kind = Application

    format = OfficeOpenXmlDrawing
    name = "Office Open XML Drawing"
    media_type = "application/vnd.ms-visio.drawing.main+xml"
    extension = "vsdx"
    kind = Application

    format = OfficeOpenXmlPresentation
    name = "Office Open XML Presentation"
    media_type = "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    extension = "pptx"
    kind = Application

    format = OfficeOpenXmlSpreadsheet
    name = "Office Open XML Spreadsheet"
    media_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    extension = "xlsx"
    kind = Application

    format = OggFlac
    name = "Ogg FLAC"
    media_type = "audio/ogg"
    extension = "oga"
    kind = Audio

    format = OggMedia
    name = "Ogg Media"
    media_type = "video/ogg"
    extension = "ogm"
    kind = Video

    format = OggMultiplexedMedia
    name = "Ogg Multiplexed Media"
    media_type = "application/ogg"
    extension = "ogx"
    kind = Application

    format = OggOpus
    name = "Ogg Opus"
    media_type = "audio/opus"
    extension = "opus"
    kind = Audio

    format = OggSpeex
    name = "Ogg Speex"
    media_type = "audio/ogg"
    extension = "spx"
    kind = Audio

    format = OggTheora
    name = "Ogg Theora"
    media_type = "video/ogg"
    extension = "ogv"
    kind = Video

    format = OggVorbis
    name = "Ogg Vorbis"
    media_type = "audio/ogg"
    extension = "ogg"
    kind = Audio

    format = OlympusRawFormat
    name = "Olympus Raw Format"
    media_type = "image/x-olympus-orf"
    extension = "orf"
    kind = Image

    format = OpenDocumentGraphics
    name = "OpenDocument Graphics"
    media_type = "application/vnd.oasis.opendocument.graphics"
    extension = "odg"
    kind = Application

    format = OpenDocumentPresentation
    name = "OpenDocument Presentation"
    media_type = "application/vnd.oasis.opendocument.presentation"
    extension = "odp"
    kind = Application

    format = OpenDocumentSpreadsheet
    name = "OpenDocument Spreadsheet"
    media_type = "application/vnd.oasis.opendocument.spreadsheet"
    extension = "ods"
    kind = Application

    format = OpenDocumentText
    name = "OpenDocument Text"
    media_type = "application/vnd.oasis.opendocument.text"
    extension = "odt"
    kind = Application

    format = Openexr
    name = "OpenEXR"
    media_type = "image/x-exr"
    extension = "exr"
    kind = Image

    format = Openraster
    name = "OpenRaster"
    media_type = "image/openraster"
    extension = "ora"
    kind = Image

    format = Opentype
    name = "OpenType"
    media_type = "font/otf"
    extension = "otf"
    kind = Font

    format = OptimizedDalvikExecutable
    name = "Optimized Dalvik Executable"
    media_type = "application/vnd.android.dey"
    extension = "dey"
    kind = Application

    format = PanasonicRaw
    name = "Panasonic Raw"
    media_type = "image/x-panasonic-rw2"
    extension = "rw2"
    kind = Image

    format = PcapDump
    name = "PCAP Dump"
    media_type = "application/vnd.tcpdump.pcap"
    extension = "pcap"
    kind = Application

    format = PcapNextGenerationDump
    name = "PCAP Next Generation Dump"
    media_type = "application/x-pcapng"
    extension = "pcapng"
    kind = Application

    format = PemCertificate
    name = "PEM Certificate"
    media_type = "application/x-pem-file"
    extension = "crt"
    kind = Application

    format = PemCertificateSigningRequest
    name = "PEM Certificate Signing Request"
    media_type = "application/x-pem-file"
    extension = "csr"
    kind = Application

    format = PemPrivateKey
    name = "PEM Private Key"
    media_type = "application/x-pem-file"
    extension = "key"
    kind = Application

    format = PerlScript
    name = "Perl Script"
    media_type = "text/x-perl"
    extension = "pl"
    kind = Text

    format = PgpMessage
    name = "PGP Message"
    media_type = "application/pgp"
    extension = "asc"
    kind = Application

    format = PgpPrivateKeyBlock
    name = "PGP Private Key Block"
    media_type = "application/pgp-keys"
    extension = "asc"
    kind = Application

    format = PgpPublicKeyBlock
    name = "PGP Public Key Block"
    media_type = "application/pgp-keys"
    extension = "asc"
    kind = Application

    format = PgpSignature
    name = "PGP Signature"
    media_type = "application/pgp-signature"
    extension = "asc"
    kind = Application

    format = PgpSignedMessage
    name = "PGP Signed Message"
    media_type = "application/pgp"
    extension = "asc"
    kind = Application

    format = PlainText
    name = "Plain Text"
    media_type = "text/plain"
    extension = "txt"
    kind = Text

    format = PortableDocumentFormat
    name = "Portable Document Format"
    media_type = "application/pdf"
    extension = "pdf"
    kind = Application

    format = PortableExecutable
    name = "Portable Executable"
    media_type = "application/vnd.microsoft.portable-executable"
    extension = "exe"
    kind = Application

    format = PortableNetworkGraphics
    name = "Portable Network Graphics"
    media_type = "image/png"
    extension = "png"
    kind = Image

    format = Postscript
    name = "PostScript"
    media_type = "application/postscript"
    extension = "ps"
    kind = Application

    format = PythonScript
    name = "Python Script"
    media_type = "text/x-script.python"
    extension = "py"
    kind = Text

    format = QualcommPureVoice
    name = "Qualcomm PureVoice"
    media_type = "audio/qcelp"
    extension = "qcp"
    kind = Audio

    format = RadianceHdr
    name = "Radiance HDR"
    media_type = "image/vnd.radiance"
    extension = "hdr"
    kind = Image

    format = ReallySimpleSyndication
    name = "Really Simple Syndication"
    media_type = "application/rss+xml"
    extension = "rss"
    kind = Application

    format = RedHatPackageManager
    name = "Red Hat Package Manager"
    media_type = "application/x-rpm"
    extension = "rpm"
    kind = Application

    format = RichTextFormat
    name = "Rich Text Format"
    media_type = "application/rtf"
    extension = "rtf"
    kind = Application

    format = RoshalArchive
    name = "Roshal Archive"
    media_type = "application/vnd.rar"
    extension = "rar"
    kind = Application

    format = RubyScript
    name = "Ruby Script"
    media_type = "text/x-ruby"
    extension = "rb"
    kind = Text

    format = ScalableVectorGraphics
    name = "Scalable Vector Graphics"
    media_type = "image/svg+xml"
    extension = "svg"
    kind = Image

    format = Screamtracker3Module
    name = "ScreamTracker 3 Module"
    media_type = "audio/x-s3m"
    extension = "s3m"
    kind = Audio

    format = Seqbox
    name = "SeqBox"
    media_type = "application/x-sbx"
    extension = "sbx"
    kind = Application

    format = SevenZip
    name = "7-Zip"
    media_type = "application/x-7z-compressed"
    extension = "7z"
    kind = Application

    format = Shapefile
    name = "Shapefile"
    media_type = "application/x-esri-shape"
    extension = "shp"
    kind = Application

    format = ShellScript
    name = "Shell Script"
    media_type = "text/x-shellscript"
    extension = "sh"
    kind = Text

    format = SimpleObjectAccessProtocol
    name = "Simple Object Access Protocol"
    media_type = "application/soap+xml"
    extension = "soap"
    kind = Application

    format = Sketchup
    name = "SketchUp"
    media_type = "application/vnd.sketchup.skp"
    extension = "skp"
    kind = Application

    format = SmallWebFormat
    name = "Small Web Format"
    media_type = "application/x-shockwave-flash"
    extension = "swf"
    kind = Application

    format = Snappy
    name = "Snappy"
    media_type = "application/x-snappy-framed"
    extension = "sz"
    kind = Application

    format = SonyDsdStreamFile
    name = "Sony DSD Stream File"
    media_type = "audio/x-dsf"
    extension = "dsf"
    kind = Audio

    format = SonyMovie
    name = "Sony Movie"
    media_type = "video/quicktime"
    extension = "mqv"
    kind = Video

    format = Sqlite3
    name = "SQLite 3"
    media_type = "application/vnd.sqlite3"
    extension = "sqlite"
    kind = Application

    format = TagImageFileFormat
    name = "Tag Image File Format"
    media_type = "image/tiff"
    extension = "tiff"
    kind = Image

    format = TapeArchive
    name = "Tape Archive"
    media_type = "application/x-tar"
    extension = "tar"
    kind = Application

    format = Tasty
    name = "TASTy"
    media_type = "application/x-tasty"
    extension = "tasty"
    kind = Application

    format = ThirdGenerationPartnershipProject
    name = "3rd Generation Partnership Project"
    media_type = "video/3gpp"
    extension = "3gp"
    kind = Video

    format = ThirdGenerationPartnershipProject2
    name = "3rd Generation Partnership Project 2"
    media_type = "video/3gpp2"
    extension = "3g2"
    kind = Video

    format = ThreeDimensionalManufacturingFormat
    name = "3D Manufacturing Format"
    media_type = "application/vnd.ms-package.3dmanufacturing-3dmodel+xml"
    extension = "3mf"
    kind = Application

    format = ToolCommandLanguageScript
    name = "Tool Command Language Script"
    media_type = "text/x-tcl"
    extension = "tcl"
    kind = Text

    format = Truetype
    name = "TrueType"
    media_type = "font/ttf"
    extension = "ttf"
    kind = Font

    format = UnixArchiver
    name = "UNIX archiver"
    media_type = "application/x-archive"
    extension = "a"
    kind = Application

    format = UnixCompress
    name = "UNIX compress"
    media_type = "application/x-compress"
    extension = "Z"
    kind = Application

    format = Vcalendar
    name = "vCalendar"
    media_type = "text/calendar"
    extension = "ics"
    kind = Text

    format = Vcard
    name = "vCard"
    media_type = "text/vcard"
    extension = "vcf"
    kind = Text

    format = VirtualboxVirtualDiskImage
    name = "VirtualBox Virtual Disk Image"
    media_type = "application/x-virtualbox-vdi"
    extension = "vdi"
    kind = Application

    format = WaveformAudio
    name = "Waveform Audio"
    media_type = "audio/vnd.wave"
    extension = "wav"
    kind = Audio

    format = Wavpack
    name = "WavPack"
    media_type = "audio/wavpack"
    extension = "wv"
    kind = Audio

    format = WebApplicationArchive
    name = "Web Application Archive"
    media_type = "application/java-archive"
    extension = "war"
    kind = Application

    format = WebOpenFontFormat
    name = "Web Open Font Format"
    media_type = "font/woff"
    extension = "woff"
    kind = Font

    format = WebOpenFontFormat2
    name = "Web Open Font Format 2"
    media_type = "font/woff2"
    extension = "woff2"
    kind = Font

    format = WebassemblyBinary
    name = "WebAssembly Binary"
    media_type = "application/wasm"
    extension = "wasm"
    kind = Application

    format = Webm
    name = "WebM"
    media_type = "video/webm"
    extension = "webm"
    kind = Video

    format = Webp
    name = "WebP"
    media_type = "image/webp"
    extension = "webp"
    kind = Image

    format = WindowsAnimatedCursor
    name = "Windows Animated Cursor"
    media_type = "application/x-navi-animation"
    extension = "ani"
    kind = Application

    format = WindowsAppPackage
    name = "Windows App Package"
    media_type = "application/vnd.ms-appx"
    extension = "appx"
    kind = Application

    format = WindowsBitmap
    name = "Windows Bitmap"
    media_type = "image/bmp"
    extension = "bmp"
    kind = Image

    format = WindowsCursor
    name = "Windows Cursor"
    media_type = "image/x-icon"
    extension = "cur"
    kind = Image

    format = WindowsIcon
    name = "Windows Icon"
    media_type = "image/x-icon"
    extension = "ico"
    kind = Image

    format = WindowsMediaVideo
    name = "Windows Media Video"
    media_type = "video/x-ms-asf"
    extension = "wmv"
    kind = Video

    format = WindowsMetafile
    name = "Windows Metafile"
    media_type = "image/wmf"
    extension = "wmf"
    kind = Image

    format = WindowsShortcut
    name = "Windows Shortcut"
    media_type = "application/x-ms-shortcut"
    extension = "lnk"
    kind = Application

    format = Xap
    name = "XAP"
    media_type = "application/x-silverlight-app"
    extension = "xap"
    kind = Application

    format = Xpinstall
    name = "XPInstall"
    media_type = "application/x-xpinstall"
    extension = "xpi"
    kind = Application

    format = Xz
    name = "XZ"
    media_type = "application/x-xz"
    extension = "xz"
    kind = Application

    format = Zip
    name = "ZIP"
    media_type = "application/zip"
    extension = "zip"
    kind = Application

    format = Zoo
    name = "zoo"
    media_type = "application/x-zoo"
    extension = "zoo"
    kind = Application

    format = Zstandard
    name = "Zstandard"
    media_type = "application/zstd"
    extension = "zst"
    kind = Application
}

impl FileFormat {
    /// Determines file format from bytes.
    ///
    /// # Examples
    ///
    /// Detects from the first bytes of a [PNG](`FileFormat::PortableNetworkGraphics`) file:
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
            match format {
                #[cfg(feature = "cfb")]
                Self::CompoundFileBinary => Self::from_cfb(&mut reader).unwrap_or_default(),
                Self::ExtensibleMarkupLanguage => Self::from_xml(&mut reader).unwrap_or_default(),
                Self::MatroskaVideo => Self::from_mkv(&mut reader).unwrap_or_default(),
                Self::MsDosExecutable => Self::from_ms_dos_exe(&mut reader).unwrap_or_default(),
                Self::PortableDocumentFormat => Self::from_pdf(&mut reader).unwrap_or_default(),
                #[cfg(feature = "zip")]
                Self::Zip => Self::from_zip(&mut reader).unwrap_or_default(),
                _ => format,
            }
        } else {
            Self::from_plain_text(&mut reader).unwrap_or_default()
        })
    }
}

impl Default for FileFormat {
    /// Returns the default file format which is
    /// [Arbitrary Binary Data](`FileFormat::ArbitraryBinaryData`).
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

/// A kind of `FileFormat` according to the media type.
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
