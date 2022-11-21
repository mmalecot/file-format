#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Result, Seek},
    path::Path,
    str,
};

#[macro_use]
mod macros;

mod binary;
mod signatures;

/// A file format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileFormat {
    /// Adaptive Multi-Rate - `amr`
    AdaptiveMultiRate,
    /// Adobe Flash Player Audio - `f4a`
    AdobeFlashPlayerAudio,
    /// Adobe Flash Player Audiobook - `f4b`
    AdobeFlashPlayerAudiobook,
    /// Adobe Flash Player Protected Video - `f4p`
    AdobeFlashPlayerProtectedVideo,
    /// Adobe Flash Player Video - `f4v`
    AdobeFlashPlayerVideo,
    /// Adobe Illustrator Artwork - `ai`
    AdobeIllustratorArtwork,
    /// Adobe InDesign Document - `indd`
    AdobeIndesignDocument,
    /// Adobe Photoshop Document - `psd`
    AdobePhotoshopDocument,
    /// Advanced Audio Coding - `aac`
    AdvancedAudioCoding,
    /// ALZ - `alz`
    Alz,
    /// Android Binary XML - `xml`
    AndroidBinaryXml,
    /// Android Compiled Resources - `arsc`
    AndroidCompiledResources,
    /// Android Package - `apk`
    #[cfg(feature = "zip")]
    AndroidPackage,
    /// ANI - `ani`
    Ani,
    /// Animated Portable Network Graphics - `apng`
    AnimatedPortableNetworkGraphics,
    /// Apache Arrow Columnar - `arrow`
    ApacheArrowColumnar,
    /// Apple Disk Image - `dmg`
    AppleDiskImage,
    /// Apple Icon Image - `icns`
    AppleIconImage,
    /// Apple iTunes Audio - `m4a`
    AppleItunesAudio,
    /// Apple iTunes Audiobook - `m4b`
    AppleItunesAudiobook,
    /// Apple iTunes Protected Audio - `m4p`
    AppleItunesProtectedAudio,
    /// Apple iTunes Video - `m4v`
    AppleItunesVideo,
    /// Apple QuickTime - `mov`
    AppleQuicktime,
    /// Arbitrary Binary Data - `bin`
    ArbitraryBinaryData,
    /// Archived by Robert Jung - `arj`
    ArchivedByRobertJung,
    /// Au - `au`
    Au,
    /// Audio Codec 3 - `ac3`
    AudioCodec3,
    /// Audio Interchange File Format - `aiff`
    AudioInterchangeFileFormat,
    /// Audio Video Interleave - `avi`
    AudioVideoInterleave,
    /// AV1 Image File Format - `avif`
    Av1ImageFileFormat,
    /// AV1 Image File Format Sequence - `avifs`
    Av1ImageFileFormatSequence,
    /// Better Portable Graphics - `bpg`
    BetterPortableGraphics,
    /// Blender - `blend`
    Blender,
    /// bzip2 - `bz2`
    Bzip2,
    /// Cabinet - `cab`
    Cabinet,
    /// Canon Raw 2 - `cr2`
    CanonRaw2,
    /// Canon Raw 3 - `cr3`
    CanonRaw3,
    /// Cineon - `cin`
    Cineon,
    /// Circuit Diagram Document - `cddx`
    #[cfg(feature = "zip")]
    CircuitDiagramDocument,
    /// Compound File Binary - `cfb`
    CompoundFileBinary,
    /// cpio - `cpio`
    Cpio,
    /// Creative Voice - `voc`
    CreativeVoice,
    /// CUR - `cur`
    Cur,
    /// Dalvik Executable - `dex`
    DalvikExecutable,
    /// Debian Binary Package - `deb`
    DebianBinaryPackage,
    /// Design Web Format XPS - `dwfx`
    #[cfg(feature = "zip")]
    DesignWebFormatXps,
    /// Digital Imaging and Communications in Medicine - `dcm`
    DigitalImagingAndCommunicationsInMedicine,
    /// Digital Picture Exchange - `dpx`
    DigitalPictureExchange,
    /// DjVu - `djvu`
    Djvu,
    /// Dynamic Link Library - `dll`
    DynamicLinkLibrary,
    /// Electronic Publication - `epub`
    #[cfg(feature = "zip")]
    ElectronicPublication,
    /// Embedded OpenType - `eot`
    EmbeddedOpentype,
    /// Enterprise Application Archive - `ear`
    #[cfg(feature = "zip")]
    EnterpriseApplicationArchive,
    /// Executable and Linkable Format - `elf`
    ExecutableAndLinkableFormat,
    /// Experimental Computing Facility - `xcf`
    ExperimentalComputingFacility,
    /// Extensible Archive - `xar`
    ExtensibleArchive,
    /// FastTracker 2 Extended Module - `xm`
    FastTracker2ExtendedModule,
    /// Flash Video - `flv`
    FlashVideo,
    /// Flexible Image Transport System - `fits`
    FlexibleImageTransportSystem,
    /// Free Lossless Audio Codec - `flac`
    FreeLosslessAudioCodec,
    /// Free Lossless Image Format - `flif`
    FreeLosslessImageFormat,
    /// Fujifilm Raw - `raf`
    FujifilmRaw,
    /// Game Boy Advance ROM - `gba`
    GameBoyAdvanceRom,
    /// Game Boy Color ROM - `gbc`
    GameBoyColorRom,
    /// Game Boy ROM - `gb`
    GameBoyRom,
    /// GL Transmission Format Binary - `glb`
    GlTransmissionFormatBinary,
    /// Google Chrome Extension - `crx`
    GoogleChromeExtension,
    /// Graphics Interchange Format - `gif`
    GraphicsInterchangeFormat,
    /// gzip - `gz`
    Gzip,
    /// High Efficiency Image Coding - `heic`
    HighEfficiencyImageCoding,
    /// High Efficiency Image Coding Sequence - `heics`
    HighEfficiencyImageCodingSequence,
    /// High Efficiency Image File Format - `heif`
    HighEfficiencyImageFileFormat,
    /// High Efficiency Image File Format Sequence - `heifs`
    HighEfficiencyImageFileFormatSequence,
    /// ICO - `ico`
    Ico,
    /// Impulse Tracker Module - `it`
    ImpulseTrackerModule,
    /// iOS App Store Package - `ipa`
    #[cfg(feature = "zip")]
    IosAppStorePackage,
    /// ISO 9660 - `iso`
    Iso9660,
    /// Java Archive - `jar`
    #[cfg(feature = "zip")]
    JavaArchive,
    /// Java Class - `class`
    JavaClass,
    /// Java KeyStore - `jks`
    JavaKeystore,
    /// Joint Photographic Experts Group - `jpg`
    JointPhotographicExpertsGroup,
    /// JPEG 2000 Part 1 - `jp2`
    Jpeg2000Part1,
    /// JPEG 2000 Part 2 - `jpx`
    Jpeg2000Part2,
    /// JPEG 2000 Part 3 - `mj2`
    Jpeg2000Part3,
    /// JPEG 2000 Part 6 - `jpm`
    Jpeg2000Part6,
    /// JPEG Extended Range - `jxr`
    JpegExtendedRange,
    /// JPEG XL - `jxl`
    JpegXl,
    /// Keyhole Markup Language Zipped - `kmz`
    #[cfg(feature = "zip")]
    KeyholeMarkupLanguageZipped,
    /// Khronos Texture - `ktx`
    KhronosTexture,
    /// Khronos Texture 2 - `ktx2`
    KhronosTexture2,
    /// Lempel–Ziv Finite State Entropy - `lzfse`
    LempelZivFiniteStateEntropy,
    /// LHA - `lzh`
    Lha,
    /// Long Range ZIP - `lrz`
    LongRangeZip,
    /// Lua Bytecode - `luac`
    LuaBytecode,
    /// LZ4 - `lz4`
    Lz4,
    /// lzip - `lz`
    Lzip,
    /// lzop - `lzo`
    Lzop,
    /// macOS Alias - `alias`
    MacosAlias,
    /// Material Exchange Format - `mxf`
    MaterialExchangeFormat,
    /// Matroska Video - `mkv`
    MatroskaVideo,
    /// Meta Information Encapsulation - `mie`
    MetaInformationEncapsulation,
    /// Microsoft Compiled HTML Help - `chm`
    MicrosoftCompiledHtmlHelp,
    /// Microsoft DirectDraw Surface - `dds`
    MicrosoftDirectDrawSurface,
    /// Microsoft Virtual Hard Disk - `vhd`
    MicrosoftVirtualHardDisk,
    /// Microsoft Virtual Hard Disk 2 - `vhdx`
    MicrosoftVirtualHardDisk2,
    /// Microsoft Visual Studio Extension - `vsix`
    #[cfg(feature = "zip")]
    MicrosoftVisualStudioExtension,
    /// Microsoft Excel Spreadsheet - `xls`
    #[cfg(feature = "cfb")]
    MicrosoftExcelSpreadsheet,
    /// Microsoft PowerPoint Presentation - `ppt`
    #[cfg(feature = "cfb")]
    MicrosoftPowerpointPresentation,
    /// Microsoft Project Plan - `mpp`
    #[cfg(feature = "cfb")]
    MicrosoftProjectPlan,
    /// Microsoft Publisher Document - `pub`
    #[cfg(feature = "cfb")]
    MicrosoftPublisherDocument,
    /// Microsoft Software Installer - `msi`
    #[cfg(feature = "cfb")]
    MicrosoftSoftwareInstaller,
    /// Microsoft Visio Drawing - `vsd`
    #[cfg(feature = "cfb")]
    MicrosoftVisioDrawing,
    /// Microsoft Word Document - `doc`
    #[cfg(feature = "cfb")]
    MicrosoftWordDocument,
    /// Mobipocket - `mobi`
    Mobipocket,
    /// Monkey's Audio - `ape`
    MonkeysAudio,
    /// MPEG-1 Video - `mpg`
    Mpeg1Video,
    /// MPEG-2 Transport Stream - `mts`
    Mpeg2TransportStream,
    /// MPEG-4 Part 14 Video - `mp4`
    Mpeg4Part14Video,
    /// MPEG-1/2 Audio Layer 3 - `mp3`
    Mpeg12AudioLayer3,
    /// MPEG-1 Audio Layer 1 - `mp1`
    Mpeg1AudioLayer1,
    /// MPEG-1 Audio Layer 2 - `mp2`
    Mpeg1AudioLayer2,
    /// MS-DOS Executable - `exe`
    MsDosExecutable,
    /// Musepack - `mpc`
    Musepack,
    /// Musical Instrument Digital Interface - `mid`
    MusicalInstrumentDigitalInterface,
    /// Nikon Electronic File - `nef`
    NikonElectronicFile,
    /// Nintendo 64 ROM - `z64`
    Nintendo64Rom,
    /// Nintendo DS ROM - `nds`
    NintendoDsRom,
    /// Nintendo Entertainment System ROM - `nes`
    NintendoEntertainmentSystemRom,
    /// Office Open XML Document - `docx`
    #[cfg(feature = "zip")]
    OfficeOpenXmlDocument,
    /// Office Open XML Drawing - `vsdx`
    #[cfg(feature = "zip")]
    OfficeOpenXmlDrawing,
    /// Office Open XML Presentation - `pptx`
    #[cfg(feature = "zip")]
    OfficeOpenXmlPresentation,
    /// Office Open XML Spreadsheet - `xlsx`
    #[cfg(feature = "zip")]
    OfficeOpenXmlSpreadsheet,
    /// Ogg FLAC - `oga`
    OggFlac,
    /// Ogg Media - `ogm`
    OggMedia,
    /// Ogg Multiplexed Media - `ogx`
    OggMultiplexedMedia,
    /// Ogg Opus - `opus`
    OggOpus,
    /// Ogg Speex - `spx`
    OggSpeex,
    /// Ogg Theora - `ogv`
    OggTheora,
    /// Ogg Vorbis - `ogg`
    OggVorbis,
    /// Olympus Raw Format - `orf`
    OlympusRawFormat,
    /// OpenDocument Graphics - `odg`
    #[cfg(feature = "zip")]
    OpenDocumentGraphics,
    /// OpenDocument Presentation - `odp`
    #[cfg(feature = "zip")]
    OpenDocumentPresentation,
    /// OpenDocument Spreadsheet - `ods`
    #[cfg(feature = "zip")]
    OpenDocumentSpreadsheet,
    /// OpenDocument Text - `odt`
    #[cfg(feature = "zip")]
    OpenDocumentText,
    /// OpenEXR - `exr`
    Openexr,
    /// OpenType - `otf`
    Opentype,
    /// Optimized Dalvik Executable - `dey`
    OptimizedDalvikExecutable,
    /// Panasonic Raw - `rw2`
    PanasonicRaw,
    /// PCAP Dump - `pcap`
    PcapDump,
    /// PCAP Next Generation Dump - `pcapng`
    PcapNextGenerationDump,
    /// Portable Document Format - `pdf`
    PortableDocumentFormat,
    /// Portable Network Graphics - `png`
    PortableNetworkGraphics,
    /// Portable Executable - `exe`
    PortableExecutable,
    /// Qualcomm PureVoice - `qcp`
    QualcommPureVoice,
    /// Radiance HDR - `hdr`
    RadianceHdr,
    /// Red Hat Package Manager - `rpm`
    RedHatPackageManager,
    /// Roshal Archive - `rar`
    RoshalArchive,
    /// ScreamTracker 3 Module - `s3m`
    Screamtracker3Module,
    /// SeqBox - `sbx`
    Seqbox,
    /// 7-Zip - `7z`
    SevenZip,
    /// Shapefile - `shp`
    Shapefile,
    /// SketchUp - `skp`
    Sketchup,
    /// Small Web Format - `swf`
    SmallWebFormat,
    /// Snappy - `sz`
    Snappy,
    /// Sony DSD Stream File - `dsf`
    SonyDsdStreamFile,
    /// Sony Movie - `mqv`
    SonyMovie,
    /// SQLite 3 - `sqlite`
    Sqlite3,
    /// Tag Image File Format - `tiff`
    TagImageFileFormat,
    /// Tape Archive - `tar`
    TapeArchive,
    /// TASTy - `tasty`
    Tasty,
    /// 3rd Generation Partnership Project - `3gp`
    ThirdGenerationPartnershipProject,
    /// 3rd Generation Partnership Project 2 - `3g2`
    ThirdGenerationPartnershipProject2,
    /// 3D Manufacturing Format - `3mf`
    #[cfg(feature = "zip")]
    ThreeDimensionalManufacturingFormat,
    /// TrueType - `ttf`
    Truetype,
    /// UNIX archiver - `a`
    UnixArchiver,
    /// UNIX compress - `Z`
    UnixCompress,
    /// VirtualBox Virtual Disk Image - `vdi`
    VirtualboxVirtualDiskImage,
    /// WavPack - `wv`
    Wavpack,
    /// Waveform Audio - `wav`
    WaveformAudio,
    /// Web Application Archive - `war`
    #[cfg(feature = "zip")]
    WebApplicationArchive,
    /// WebAssembly Binary - `wasm`
    WebassemblyBinary,
    /// Web Open Font Format - `woff`
    WebOpenFontFormat,
    /// Web Open Font Format 2 - `woff2`
    WebOpenFontFormat2,
    /// WebM - `webm`
    Webm,
    /// WebP - `webp`
    Webp,
    /// Windows App Package - `appx`
    #[cfg(feature = "zip")]
    WindowsAppPackage,
    /// Windows Bitmap - `bmp`
    WindowsBitmap,
    /// Windows Media Video - `wmv`
    WindowsMediaVideo,
    /// Windows Metafile - `wmf`
    WindowsMetafile,
    /// Windows Shortcut - `lnk`
    WindowsShortcut,
    /// XAP - `xap`
    #[cfg(feature = "zip")]
    Xap,
    /// XPInstall - `xpi`
    #[cfg(feature = "zip")]
    Xpinstall,
    /// XZ - `xz`
    Xz,
    /// ZIP - `zip`
    Zip,
    /// zoo - `zoo`
    Zoo,
    /// Zstandard - `zst`
    Zstandard,
}

impl FileFormat {
    /// Maximum number of bytes to read to detect a `FileFormat` from its signature.
    const MAX_BYTES: usize = 36870;

    /// Returns the name of the `FileFormat`.
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
            Self::AdaptiveMultiRate => "Adaptive Multi-Rate",
            Self::AdobeFlashPlayerAudio => "Adobe Flash Player Audio",
            Self::AdobeFlashPlayerAudiobook => "Adobe Flash Player Audiobook ",
            Self::AdobeFlashPlayerProtectedVideo => "Adobe Flash Player Protected Video",
            Self::AdobeFlashPlayerVideo => "Adobe Flash Player Video",
            Self::AdobeIllustratorArtwork => "Adobe Illustrator Artwork",
            Self::AdobeIndesignDocument => "Adobe InDesign Document",
            Self::AdobePhotoshopDocument => "Adobe Photoshop Document",
            Self::AdvancedAudioCoding => "Advanced Audio Coding",
            Self::Alz => "ALZ",
            Self::AndroidBinaryXml => "Android Binary XML",
            Self::AndroidCompiledResources => "Android Compiled Resources",
            #[cfg(feature = "zip")]
            Self::AndroidPackage => "Android Package",
            Self::Ani => "ANI",
            Self::AnimatedPortableNetworkGraphics => "Animated Portable Network Graphics",
            Self::ApacheArrowColumnar => "Apache Arrow Columnar",
            Self::AppleDiskImage => "Apple Disk Image",
            Self::AppleIconImage => "Apple Icon Image",
            Self::AppleItunesAudio => "Apple iTunes Audio",
            Self::AppleItunesAudiobook => "Apple iTunes Audiobook",
            Self::AppleItunesProtectedAudio => "Apple iTunes Protected Audio",
            Self::AppleItunesVideo => "Apple iTunes Video",
            Self::AppleQuicktime => "Apple QuickTime",
            Self::ArbitraryBinaryData => "Arbitrary Binary Data",
            Self::ArchivedByRobertJung => "Archived by Robert Jung",
            Self::Au => "Au",
            Self::AudioCodec3 => "Audio Codec 3",
            Self::AudioInterchangeFileFormat => "Audio Interchange File Format",
            Self::AudioVideoInterleave => "Audio Video Interleave",
            Self::Av1ImageFileFormat => "AV1 Image File Format",
            Self::Av1ImageFileFormatSequence => "AV1 Image File Format Sequence",
            Self::BetterPortableGraphics => "Better Portable Graphics",
            Self::Blender => "Blender",
            Self::Bzip2 => "bzip2",
            Self::Cabinet => "Cabinet",
            Self::CanonRaw2 => "Canon Raw 2",
            Self::CanonRaw3 => "Canon Raw 3",
            Self::Cineon => "Cineon",
            #[cfg(feature = "zip")]
            Self::CircuitDiagramDocument => "Circuit Diagram Document",
            Self::CompoundFileBinary => "Compound File Binary",
            Self::Cpio => "cpio",
            Self::CreativeVoice => "Creative Voice",
            Self::Cur => "CUR",
            Self::DalvikExecutable => "Dalvik Executable",
            Self::DebianBinaryPackage => "Debian Binary Package",
            #[cfg(feature = "zip")]
            Self::DesignWebFormatXps => "Design Web Format XPS",
            Self::DigitalImagingAndCommunicationsInMedicine => {
                "Digital Imaging and Communications in Medicine"
            }
            Self::DigitalPictureExchange => "Digital Picture Exchange",
            Self::Djvu => "DjVu",
            Self::DynamicLinkLibrary => "Dynamic Link Library",
            #[cfg(feature = "zip")]
            Self::ElectronicPublication => "Electronic Publication",
            Self::EmbeddedOpentype => "Embedded OpenType",
            #[cfg(feature = "zip")]
            Self::EnterpriseApplicationArchive => "Enterprise Application Archive",
            Self::ExecutableAndLinkableFormat => "Executable and Linkable Format",
            Self::ExperimentalComputingFacility => "Experimental Computing Facility",
            Self::ExtensibleArchive => "Extensible Archive",
            Self::FastTracker2ExtendedModule => "FastTracker 2 Extended Module",
            Self::FlashVideo => "Flash Video",
            Self::FlexibleImageTransportSystem => "Flexible Image Transport System",
            Self::FreeLosslessAudioCodec => "Free Lossless Audio Codec",
            Self::FreeLosslessImageFormat => "Free Lossless Image Format",
            Self::FujifilmRaw => "Fujifilm Raw",
            Self::GameBoyAdvanceRom => "Game Boy Advance ROM",
            Self::GameBoyColorRom => "Game Boy Color ROM",
            Self::GameBoyRom => "Game Boy ROM",
            Self::GlTransmissionFormatBinary => "GL Transmission Format Binary",
            Self::GoogleChromeExtension => "Google Chrome Extension",
            Self::GraphicsInterchangeFormat => "Graphics Interchange Format",
            Self::Gzip => "gzip",
            Self::HighEfficiencyImageCoding => "High Efficiency Image Coding",
            Self::HighEfficiencyImageCodingSequence => "High Efficiency Image Coding Sequence",
            Self::HighEfficiencyImageFileFormat => "High Efficiency Image File Format",
            Self::HighEfficiencyImageFileFormatSequence => {
                "High Efficiency Image File Format Sequence"
            }
            Self::Ico => "ICO",
            Self::ImpulseTrackerModule => "Impulse Tracker Module",
            #[cfg(feature = "zip")]
            Self::IosAppStorePackage => "iOS App Store Package",
            Self::Iso9660 => "ISO 9660",
            #[cfg(feature = "zip")]
            Self::JavaArchive => "Java Archive",
            Self::JavaClass => "Java Class",
            Self::JavaKeystore => "Java KeyStore",
            Self::JointPhotographicExpertsGroup => "Joint Photographic Experts Group",
            Self::Jpeg2000Part1 => "JPEG 2000 Part 1",
            Self::Jpeg2000Part2 => "JPEG 2000 Part 2",
            Self::Jpeg2000Part3 => "JPEG 2000 Part 3",
            Self::Jpeg2000Part6 => "JPEG 2000 Part 6",
            Self::JpegExtendedRange => "PEG Extended Range",
            Self::JpegXl => "JPEG XL",
            #[cfg(feature = "zip")]
            Self::KeyholeMarkupLanguageZipped => "Keyhole Markup Language Zipped",
            Self::KhronosTexture => "Khronos Texture",
            Self::KhronosTexture2 => "Khronos Texture 2",
            Self::LempelZivFiniteStateEntropy => "Lempel–Ziv Finite State Entropy",
            Self::Lha => "LHA",
            Self::LongRangeZip => "Long Range ZIP",
            Self::LuaBytecode => "Lua Bytecode",
            Self::Lz4 => "LZ4",
            Self::Lzip => "lzip",
            Self::Lzop => "lzop",
            Self::MacosAlias => "macOS Alias",
            Self::MaterialExchangeFormat => "Material Exchange Format",
            Self::MatroskaVideo => "Meta Information Encapsulation",
            Self::MetaInformationEncapsulation => "Matroska Video",
            Self::MicrosoftCompiledHtmlHelp => "Microsoft Compiled HTML Help",
            Self::MicrosoftDirectDrawSurface => "Microsoft DirectDraw Surface",
            Self::MicrosoftVirtualHardDisk => "Microsoft Virtual Hard Disk",
            Self::MicrosoftVirtualHardDisk2 => "Microsoft Virtual Hard Disk 2",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisualStudioExtension => "Microsoft Visual Studio Extension",
            #[cfg(feature = "cfb")]
            Self::MicrosoftExcelSpreadsheet => "Microsoft Excel Spreadsheet",
            #[cfg(feature = "cfb")]
            Self::MicrosoftPowerpointPresentation => "Microsoft PowerPoint Presentation",
            #[cfg(feature = "cfb")]
            Self::MicrosoftProjectPlan => "Microsoft Project Plan",
            #[cfg(feature = "cfb")]
            Self::MicrosoftPublisherDocument => "Microsoft Publisher Document",
            #[cfg(feature = "cfb")]
            Self::MicrosoftSoftwareInstaller => "Microsoft Software Installer",
            #[cfg(feature = "cfb")]
            Self::MicrosoftVisioDrawing => "Microsoft Visio Drawing",
            #[cfg(feature = "cfb")]
            Self::MicrosoftWordDocument => "Microsoft Word Document",
            Self::Mobipocket => "Mobipocket",
            Self::MonkeysAudio => "Monkey's Audio",
            Self::Mpeg1Video => "MPEG-1 Video",
            Self::Mpeg2TransportStream => "MPEG-2 Transport Stream",
            Self::Mpeg4Part14Video => "MPEG-4 Part 14 Video",
            Self::Mpeg12AudioLayer3 => "MPEG-1/2 Audio Layer 3",
            Self::Mpeg1AudioLayer1 => "MPEG-1 Audio Layer 1",
            Self::Mpeg1AudioLayer2 => "MPEG-1 Audio Layer 2",
            Self::MsDosExecutable => "MS-DOS Executable",
            Self::Musepack => "Musepack",
            Self::MusicalInstrumentDigitalInterface => "Musical Instrument Digital Interface",
            Self::NikonElectronicFile => "Nikon Electronic File",
            Self::Nintendo64Rom => "Nintendo 64 ROM",
            Self::NintendoDsRom => "Nintendo DS ROM",
            Self::NintendoEntertainmentSystemRom => "Nintendo Entertainment System ROM",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDocument => "Office Open XML Document",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDrawing => "Office Open XML Drawing",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlPresentation => "Office Open XML Presentation",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlSpreadsheet => "Office Open XML Spreadsheet",
            Self::OggFlac => "Ogg FLAC",
            Self::OggMedia => "Ogg Media",
            Self::OggMultiplexedMedia => "Ogg Multiplexed Media",
            Self::OggOpus => "Ogg Opus",
            Self::OggSpeex => "Ogg Speex",
            Self::OggTheora => "Ogg Theora",
            Self::OggVorbis => "Ogg Vorbis",
            Self::OlympusRawFormat => "Olympus Raw Format",
            #[cfg(feature = "zip")]
            Self::OpenDocumentGraphics => "OpenDocument Graphics",
            #[cfg(feature = "zip")]
            Self::OpenDocumentPresentation => "OpenDocument Presentation",
            #[cfg(feature = "zip")]
            Self::OpenDocumentSpreadsheet => "OpenDocument Spreadsheet",
            #[cfg(feature = "zip")]
            Self::OpenDocumentText => "OpenDocument Text",
            Self::Openexr => "OpenEXR",
            Self::Opentype => "OpenType",
            Self::OptimizedDalvikExecutable => "Optimized Dalvik Executable",
            Self::PanasonicRaw => "Panasonic Raw",
            Self::PcapDump => "PCAP Dump",
            Self::PcapNextGenerationDump => "PCAP Next Generation Dump",
            Self::PortableDocumentFormat => "Portable Document Format",
            Self::PortableExecutable => "Portable Executable",
            Self::PortableNetworkGraphics => "Portable Network Graphics",
            Self::QualcommPureVoice => "Qualcomm PureVoice",
            Self::RadianceHdr => "Radiance HDR",
            Self::RedHatPackageManager => "Red Hat Package Manager",
            Self::RoshalArchive => "Roshal Archive",
            Self::Screamtracker3Module => "ScreamTracker 3 Module",
            Self::Seqbox => "SeqBox",
            Self::SevenZip => "7-Zip",
            Self::Shapefile => "Shapefile",
            Self::Sketchup => "SketchUp",
            Self::SmallWebFormat => "Small Web Format",
            Self::Snappy => "Snappy",
            Self::SonyDsdStreamFile => "Sony DSD Stream File",
            Self::SonyMovie => "Sony Movie",
            Self::Sqlite3 => "SQLite 3",
            Self::TagImageFileFormat => "Tag Image File Format",
            Self::TapeArchive => "Tape Archive",
            Self::Tasty => "TASTy",
            Self::ThirdGenerationPartnershipProject => "3rd Generation Partnership Project",
            Self::ThirdGenerationPartnershipProject2 => "3rd Generation Partnership Project 2",
            #[cfg(feature = "zip")]
            Self::ThreeDimensionalManufacturingFormat => "3D Manufacturing Format",
            Self::Truetype => "TrueType",
            Self::UnixArchiver => "UNIX archiver",
            Self::UnixCompress => "UNIX compress",
            Self::VirtualboxVirtualDiskImage => "VirtualBox Virtual Disk Image",
            Self::Wavpack => "WavPack",
            Self::WaveformAudio => "Waveform Audio",
            #[cfg(feature = "zip")]
            Self::WebApplicationArchive => "Web Application Archive",
            Self::WebassemblyBinary => "WebAssembly Binary",
            Self::WebOpenFontFormat => "Web Open Font Format",
            Self::WebOpenFontFormat2 => "Web Open Font Format 2",
            Self::Webm => "WebM",
            Self::Webp => "WebP",
            #[cfg(feature = "zip")]
            Self::WindowsAppPackage => "Windows App Package",
            Self::WindowsBitmap => "Windows Bitmap",
            Self::WindowsMediaVideo => "Windows Media Video",
            Self::WindowsMetafile => "Windows Metafile",
            Self::WindowsShortcut => "Windows Shortcut",
            #[cfg(feature = "zip")]
            Self::Xap => "XAP",
            #[cfg(feature = "zip")]
            Self::Xpinstall => "XPInstall",
            Self::Xz => "XZ",
            Self::Zip => "ZIP",
            Self::Zoo => "zoo",
            Self::Zstandard => "Zstandard",
        }
    }

    /// Returns the media type (formerly known as MIME type) of the `FileFormat`.
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
            Self::AdaptiveMultiRate => "audio/amr",
            Self::AdobeFlashPlayerAudio => "audio/mp4",
            Self::AdobeFlashPlayerAudiobook => "audio/mp4",
            Self::AdobeFlashPlayerProtectedVideo => "video/mp4",
            Self::AdobeFlashPlayerVideo => "video/mp4",
            Self::AdobeIllustratorArtwork => "application/pdf",
            Self::AdobeIndesignDocument => "application/x-indesign",
            Self::AdobePhotoshopDocument => "image/vnd.adobe.photoshop",
            Self::AdvancedAudioCoding => "audio/aac",
            Self::Alz => "application/x-alz-compressed",
            Self::AndroidBinaryXml => "application/vnd.android.axml",
            Self::AndroidCompiledResources => "application/vnd.android.arsc",
            #[cfg(feature = "zip")]
            Self::AndroidPackage => "application/vnd.android.package-archive",
            Self::Ani => "application/x-navi-animation",
            Self::AnimatedPortableNetworkGraphics => "image/apng",
            Self::ApacheArrowColumnar => "application/x-apache-arrow",
            Self::AppleDiskImage => "application/x-apple-diskimage",
            Self::AppleIconImage => "image/x-icns",
            Self::AppleItunesAudio => "audio/x-m4a",
            Self::AppleItunesAudiobook => "audio/mp4",
            Self::AppleItunesProtectedAudio => "audio/mp4",
            Self::AppleItunesVideo => "video/x-m4v",
            Self::AppleQuicktime => "video/quicktime",
            Self::ArbitraryBinaryData => "application/octet-stream",
            Self::ArchivedByRobertJung => "application/x-arj",
            Self::Au => "audio/basic",
            Self::AudioCodec3 => "audio/vnd.dolby.dd-raw",
            Self::AudioInterchangeFileFormat => "audio/aiff",
            Self::AudioVideoInterleave => "video/avi",
            Self::Av1ImageFileFormat => "image/avif",
            Self::Av1ImageFileFormatSequence => "image/avif-sequence",
            Self::BetterPortableGraphics => "image/bpg",
            Self::Blender => "application/x-blender",
            Self::Bzip2 => "application/x-bzip2",
            Self::Cabinet => "application/vnd.ms-cab-compressed",
            Self::CanonRaw2 => "image/x-canon-cr2",
            Self::CanonRaw3 => "image/x-canon-cr3",
            Self::Cineon => "image/cineon",
            #[cfg(feature = "zip")]
            Self::CircuitDiagramDocument => "application/vnd.circuitdiagram.document.main+xml",
            Self::CompoundFileBinary => "application/x-cfb",
            Self::Cpio => "application/x-cpio",
            Self::CreativeVoice => "audio/x-voc",
            Self::Cur => "image/x-icon",
            Self::DalvikExecutable => "application/vnd.android.dex",
            Self::DebianBinaryPackage => "application/vnd.debian.binary-package",
            #[cfg(feature = "zip")]
            Self::DesignWebFormatXps => "model/vnd.dwfx+xps",
            Self::DigitalImagingAndCommunicationsInMedicine => "application/dicom",
            Self::DigitalPictureExchange => "image/x-dpx",
            Self::Djvu => "image/vnd.djvu",
            Self::DynamicLinkLibrary => "application/vnd.microsoft.portable-executable",
            #[cfg(feature = "zip")]
            Self::ElectronicPublication => "application/epub+zip",
            Self::EmbeddedOpentype => "application/vnd.ms-fontobject",
            #[cfg(feature = "zip")]
            Self::EnterpriseApplicationArchive => "application/java-archive",
            Self::ExecutableAndLinkableFormat => "application/x-executable",
            Self::ExperimentalComputingFacility => "image/x-xcf",
            Self::ExtensibleArchive => "application/x-xar",
            Self::FastTracker2ExtendedModule => "audio/x-xm",
            Self::FlashVideo => "video/x-flv",
            Self::FlexibleImageTransportSystem => "image/fits",
            Self::FreeLosslessAudioCodec => "audio/x-flac",
            Self::FreeLosslessImageFormat => "image/flif",
            Self::FujifilmRaw => "image/x-fuji-raf",
            Self::GameBoyAdvanceRom => "application/x-gba-rom",
            Self::GameBoyColorRom => "application/x-gameboy-color-rom",
            Self::GameBoyRom => "application/x-gameboy-rom",
            Self::GlTransmissionFormatBinary => "model/gltf-binary",
            Self::GoogleChromeExtension => "application/x-google-chrome-extension",
            Self::GraphicsInterchangeFormat => "image/gif",
            Self::Gzip => "application/gzip",
            Self::HighEfficiencyImageCoding => "image/heic",
            Self::HighEfficiencyImageCodingSequence => "image/heic-sequence",
            Self::HighEfficiencyImageFileFormat => "image/heif",
            Self::HighEfficiencyImageFileFormatSequence => "image/heif-sequence",
            Self::Ico => "image/x-icon",
            Self::ImpulseTrackerModule => "audio/x-it",
            #[cfg(feature = "zip")]
            Self::IosAppStorePackage => "application/x-ios-app",
            Self::Iso9660 => "application/x-iso9660-image",
            #[cfg(feature = "zip")]
            Self::JavaArchive => "application/java-archive",
            Self::JavaClass => "application/java-vm",
            Self::JavaKeystore => "application/x-java-keystore",
            Self::JointPhotographicExpertsGroup => "image/jpeg",
            Self::Jpeg2000Part1 => "image/jp2",
            Self::Jpeg2000Part2 => "image/jpx",
            Self::Jpeg2000Part3 => "image/mj2",
            Self::Jpeg2000Part6 => "image/jpm",
            Self::JpegExtendedRange => "image/jxr",
            Self::JpegXl => "image/jxl",
            #[cfg(feature = "zip")]
            Self::KeyholeMarkupLanguageZipped => "application/vnd.google-earth.kmz",
            Self::KhronosTexture => "image/ktx",
            Self::KhronosTexture2 => "image/ktx2",
            Self::LempelZivFiniteStateEntropy => "application/x-lzfse",
            Self::Lha => "application/x-lzh-compressed",
            Self::LongRangeZip => "application/x-lrzip",
            Self::LuaBytecode => "application/x-lua-bytecode",
            Self::Lz4 => "application/x-lz4",
            Self::Lzip => "application/x-lzip",
            Self::Lzop => "application/x-lzop",
            Self::MacosAlias => "application/x-apple-alias",
            Self::MaterialExchangeFormat => "application/mxf",
            Self::MatroskaVideo => "video/x-matroska",
            Self::MetaInformationEncapsulation => "application/x-mie",
            Self::MicrosoftCompiledHtmlHelp => "application/vnd.ms-htmlhelp",
            Self::MicrosoftDirectDrawSurface => "image/vnd-ms.dds",
            Self::MicrosoftVirtualHardDisk => "application/x-vhd",
            Self::MicrosoftVirtualHardDisk2 => "application/x-vhdx",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisualStudioExtension => "application/vsix",
            #[cfg(feature = "cfb")]
            Self::MicrosoftExcelSpreadsheet => "application/vnd.ms-excel",
            #[cfg(feature = "cfb")]
            Self::MicrosoftPowerpointPresentation => "application/vnd.ms-powerpoint",
            #[cfg(feature = "cfb")]
            Self::MicrosoftProjectPlan => "application/vnd.ms-project",
            #[cfg(feature = "cfb")]
            Self::MicrosoftPublisherDocument => "application/vnd.ms-publisher",
            #[cfg(feature = "cfb")]
            Self::MicrosoftSoftwareInstaller => "application/x-msi",
            #[cfg(feature = "cfb")]
            Self::MicrosoftVisioDrawing => "application/vnd.visio",
            #[cfg(feature = "cfb")]
            Self::MicrosoftWordDocument => "application/msword",
            Self::Mobipocket => "application/x-mobipocket-ebook",
            Self::MonkeysAudio => "audio/x-ape",
            Self::Mpeg1Video => "video/mpeg",
            Self::Mpeg2TransportStream => "video/mp2t",
            Self::Mpeg4Part14Video => "video/mp4",
            Self::Mpeg12AudioLayer3 => "audio/mpeg",
            Self::Mpeg1AudioLayer1 => "audio/mpeg",
            Self::Mpeg1AudioLayer2 => "audio/mpeg",
            Self::MsDosExecutable => "application/x-dosexec",
            Self::Musepack => "audio/x-musepack",
            Self::MusicalInstrumentDigitalInterface => "audio/midi",
            Self::NikonElectronicFile => "image/x-nikon-nef",
            Self::Nintendo64Rom => "application/x-n64-rom",
            Self::NintendoDsRom => "application/x-nintendo-ds-rom",
            Self::NintendoEntertainmentSystemRom => "application/x-nintendo-nes-rom",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDocument => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDrawing => "application/vnd.ms-visio.drawing.main+xml",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlPresentation => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlSpreadsheet => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
            Self::OggFlac => "audio/ogg",
            Self::OggMedia => "video/ogg",
            Self::OggMultiplexedMedia => "application/ogg",
            Self::OggOpus => "audio/opus",
            Self::OggSpeex => "audio/ogg",
            Self::OggTheora => "video/ogg",
            Self::OggVorbis => "audio/ogg",
            Self::OlympusRawFormat => "image/x-olympus-orf",
            #[cfg(feature = "zip")]
            Self::OpenDocumentGraphics => "application/vnd.oasis.opendocument.graphics",
            #[cfg(feature = "zip")]
            Self::OpenDocumentPresentation => "application/vnd.oasis.opendocument.presentation",
            #[cfg(feature = "zip")]
            Self::OpenDocumentSpreadsheet => "application/vnd.oasis.opendocument.spreadsheet",
            #[cfg(feature = "zip")]
            Self::OpenDocumentText => "application/vnd.oasis.opendocument.text",
            Self::Openexr => "image/x-exr",
            Self::Opentype => "font/otf",
            Self::OptimizedDalvikExecutable => "application/vnd.android.dey",
            Self::PanasonicRaw => "image/x-panasonic-rw2",
            Self::PcapDump => "application/vnd.tcpdump.pcap",
            Self::PcapNextGenerationDump => "application/x-pcapng",
            Self::PortableDocumentFormat => "application/pdf",
            Self::PortableExecutable => "application/vnd.microsoft.portable-executable",
            Self::PortableNetworkGraphics => "image/png",
            Self::QualcommPureVoice => "audio/qcelp",
            Self::RadianceHdr => "image/vnd.radiance",
            Self::RedHatPackageManager => "application/x-rpm",
            Self::RoshalArchive => "application/vnd.rar",
            Self::Screamtracker3Module => "audio/x-s3m",
            Self::Seqbox => "application/x-sbx",
            Self::SevenZip => "application/x-7z-compressed",
            Self::Shapefile => "application/x-esri-shape",
            Self::Sketchup => "application/vnd.sketchup.skp",
            Self::SmallWebFormat => "application/x-shockwave-flash",
            Self::Snappy => "application/x-snappy-framed",
            Self::SonyDsdStreamFile => "audio/x-dsf",
            Self::SonyMovie => "video/quicktime",
            Self::Sqlite3 => "application/vnd.sqlite3",
            Self::TagImageFileFormat => "image/tiff",
            Self::TapeArchive => "application/x-tar",
            Self::Tasty => "application/x-tasty",
            Self::ThirdGenerationPartnershipProject => "video/3gpp",
            Self::ThirdGenerationPartnershipProject2 => "video/3gpp2",
            #[cfg(feature = "zip")]
            Self::ThreeDimensionalManufacturingFormat => {
                "application/vnd.ms-package.3dmanufacturing-3dmodel+xml"
            }
            Self::Truetype => "font/ttf",
            Self::UnixArchiver => "application/x-archive",
            Self::UnixCompress => "application/x-compress",
            Self::VirtualboxVirtualDiskImage => "application/x-virtualbox-vdi",
            Self::Wavpack => "audio/wavpack",
            Self::WaveformAudio => "audio/vnd.wave",
            #[cfg(feature = "zip")]
            Self::WebApplicationArchive => "application/java-archive",
            Self::WebassemblyBinary => "application/wasm",
            Self::WebOpenFontFormat => "font/woff",
            Self::WebOpenFontFormat2 => "font/woff2",
            Self::Webm => "video/webm",
            Self::Webp => "image/webp",
            #[cfg(feature = "zip")]
            Self::WindowsAppPackage => "application/vnd.ms-appx",
            Self::WindowsBitmap => "image/bmp",
            Self::WindowsMediaVideo => "video/x-ms-asf",
            Self::WindowsMetafile => "image/wmf",
            Self::WindowsShortcut => "application/x-ms-shortcut",
            #[cfg(feature = "zip")]
            Self::Xap => "application/x-silverlight-app",
            #[cfg(feature = "zip")]
            Self::Xpinstall => "application/x-xpinstall",
            Self::Xz => "application/x-xz",
            Self::Zip => "application/zip",
            Self::Zoo => "application/x-zoo",
            Self::Zstandard => "application/zstd",
        }
    }

    /// Returns the extension of the `FileFormat`.
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
            Self::AdaptiveMultiRate => "amr",
            Self::AdobeFlashPlayerAudio => "f4a",
            Self::AdobeFlashPlayerAudiobook => "f4b",
            Self::AdobeFlashPlayerProtectedVideo => "f4p",
            Self::AdobeFlashPlayerVideo => "f4v",
            Self::AdobeIllustratorArtwork => "ai",
            Self::AdobeIndesignDocument => "indd",
            Self::AdobePhotoshopDocument => "psd",
            Self::AdvancedAudioCoding => "aac",
            Self::Alz => "alz",
            Self::AndroidBinaryXml => "xml",
            Self::AndroidCompiledResources => "arsc",
            #[cfg(feature = "zip")]
            Self::AndroidPackage => "apk",
            Self::Ani => "ani",
            Self::AnimatedPortableNetworkGraphics => "apng",
            Self::ApacheArrowColumnar => "arrow",
            Self::AppleDiskImage => "dmg",
            Self::AppleIconImage => "icns",
            Self::AppleItunesAudio => "m4a",
            Self::AppleItunesAudiobook => "m4b",
            Self::AppleItunesProtectedAudio => "m4p",
            Self::AppleItunesVideo => "m4v",
            Self::AppleQuicktime => "mov",
            Self::ArbitraryBinaryData => "bin",
            Self::ArchivedByRobertJung => "arj",
            Self::Au => "au",
            Self::AudioCodec3 => "ac3",
            Self::AudioInterchangeFileFormat => "aiff",
            Self::AudioVideoInterleave => "avi",
            Self::Av1ImageFileFormat => "avif",
            Self::Av1ImageFileFormatSequence => "avifs",
            Self::BetterPortableGraphics => "bpg",
            Self::Blender => "blend",
            Self::Bzip2 => "bz2",
            Self::Cabinet => "cab",
            Self::CanonRaw2 => "cr2",
            Self::CanonRaw3 => "cr3",
            Self::Cineon => "cin",
            #[cfg(feature = "zip")]
            Self::CircuitDiagramDocument => "cddx",
            Self::CompoundFileBinary => "cfb",
            Self::Cpio => "cpio",
            Self::CreativeVoice => "voc",
            Self::Cur => "cur",
            Self::DalvikExecutable => "dex",
            Self::DebianBinaryPackage => "deb",
            #[cfg(feature = "zip")]
            Self::DesignWebFormatXps => "dwfx",
            Self::DigitalImagingAndCommunicationsInMedicine => "dcm",
            Self::DigitalPictureExchange => "dpx",
            Self::Djvu => "djvu",
            Self::DynamicLinkLibrary => "dll",
            #[cfg(feature = "zip")]
            Self::ElectronicPublication => "epub",
            Self::EmbeddedOpentype => "eot",
            #[cfg(feature = "zip")]
            Self::EnterpriseApplicationArchive => "ear",
            Self::ExecutableAndLinkableFormat => "elf",
            Self::ExperimentalComputingFacility => "xcf",
            Self::ExtensibleArchive => "xar",
            Self::FastTracker2ExtendedModule => "xm",
            Self::FlashVideo => "flv",
            Self::FlexibleImageTransportSystem => "fits",
            Self::FreeLosslessAudioCodec => "flac",
            Self::FreeLosslessImageFormat => "flif",
            Self::FujifilmRaw => "raf",
            Self::GameBoyAdvanceRom => "gba",
            Self::GameBoyColorRom => "gbc",
            Self::GameBoyRom => "gb",
            Self::GlTransmissionFormatBinary => "glb",
            Self::GoogleChromeExtension => "crx",
            Self::GraphicsInterchangeFormat => "gif",
            Self::Gzip => "gz",
            Self::HighEfficiencyImageCoding => "heic",
            Self::HighEfficiencyImageCodingSequence => "heics",
            Self::HighEfficiencyImageFileFormat => "heif",
            Self::HighEfficiencyImageFileFormatSequence => "heifs",
            Self::Ico => "ico",
            Self::ImpulseTrackerModule => "it",
            #[cfg(feature = "zip")]
            Self::IosAppStorePackage => "ipa",
            Self::Iso9660 => "iso",
            #[cfg(feature = "zip")]
            Self::JavaArchive => "jar",
            Self::JavaClass => "class",
            Self::JavaKeystore => "jks",
            Self::JointPhotographicExpertsGroup => "jpg",
            Self::Jpeg2000Part1 => "jp2",
            Self::Jpeg2000Part2 => "jpx",
            Self::Jpeg2000Part3 => "mj2",
            Self::Jpeg2000Part6 => "jpm",
            Self::JpegExtendedRange => "jxr",
            Self::JpegXl => "jxl",
            #[cfg(feature = "zip")]
            Self::KeyholeMarkupLanguageZipped => "kmz",
            Self::KhronosTexture => "ktx",
            Self::KhronosTexture2 => "ktx2",
            Self::LempelZivFiniteStateEntropy => "lzfse",
            Self::Lha => "lzh",
            Self::LongRangeZip => "lrz",
            Self::LuaBytecode => "luac",
            Self::Lz4 => "lz4",
            Self::Lzip => "lz",
            Self::Lzop => "lzo",
            Self::MacosAlias => "alias",
            Self::MaterialExchangeFormat => "mxf",
            Self::MatroskaVideo => "mkv",
            Self::MetaInformationEncapsulation => "mie",
            Self::MicrosoftCompiledHtmlHelp => "chm",
            Self::MicrosoftDirectDrawSurface => "dds",
            Self::MicrosoftVirtualHardDisk => "vhd",
            Self::MicrosoftVirtualHardDisk2 => "vhdx",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisualStudioExtension => "vsix",
            #[cfg(feature = "cfb")]
            Self::MicrosoftExcelSpreadsheet => "xls",
            #[cfg(feature = "cfb")]
            Self::MicrosoftPowerpointPresentation => "ppt",
            #[cfg(feature = "cfb")]
            Self::MicrosoftProjectPlan => "mpp",
            #[cfg(feature = "cfb")]
            Self::MicrosoftPublisherDocument => "pub",
            #[cfg(feature = "cfb")]
            Self::MicrosoftSoftwareInstaller => "msi",
            #[cfg(feature = "cfb")]
            Self::MicrosoftVisioDrawing => "vsd",
            #[cfg(feature = "cfb")]
            Self::MicrosoftWordDocument => "doc",
            Self::Mobipocket => "mobi",
            Self::MonkeysAudio => "ape",
            Self::Mpeg1Video => "mpg",
            Self::Mpeg2TransportStream => "mts",
            Self::Mpeg4Part14Video => "mp4",
            Self::Mpeg12AudioLayer3 => "mp3",
            Self::Mpeg1AudioLayer1 => "mp1",
            Self::Mpeg1AudioLayer2 => "mp2",
            Self::MsDosExecutable => "exe",
            Self::Musepack => "mpc",
            Self::MusicalInstrumentDigitalInterface => "mid",
            Self::NikonElectronicFile => "nef",
            Self::Nintendo64Rom => "z64",
            Self::NintendoDsRom => "nds",
            Self::NintendoEntertainmentSystemRom => "nes",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDocument => "docx",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDrawing => "vsdx",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlPresentation => "pptx",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlSpreadsheet => "xlsx",
            Self::OggFlac => "oga",
            Self::OggMedia => "ogm",
            Self::OggMultiplexedMedia => "ogx",
            Self::OggOpus => "opus",
            Self::OggSpeex => "spx",
            Self::OggTheora => "ogv",
            Self::OggVorbis => "ogg",
            Self::OlympusRawFormat => "orf",
            #[cfg(feature = "zip")]
            Self::OpenDocumentGraphics => "odg",
            #[cfg(feature = "zip")]
            Self::OpenDocumentPresentation => "odp",
            #[cfg(feature = "zip")]
            Self::OpenDocumentSpreadsheet => "ods",
            #[cfg(feature = "zip")]
            Self::OpenDocumentText => "odt",
            Self::Openexr => "exr",
            Self::Opentype => "otf",
            Self::OptimizedDalvikExecutable => "dey",
            Self::PanasonicRaw => "rw2",
            Self::PcapDump => "pcap",
            Self::PcapNextGenerationDump => "pcapng",
            Self::PortableDocumentFormat => "pdf",
            Self::PortableExecutable => "exe",
            Self::PortableNetworkGraphics => "png",
            Self::QualcommPureVoice => "qcp",
            Self::RadianceHdr => "hdr",
            Self::RedHatPackageManager => "rpm",
            Self::RoshalArchive => "rar",
            Self::Screamtracker3Module => "s3m",
            Self::Seqbox => "sbx",
            Self::SevenZip => "7z",
            Self::Shapefile => "shp",
            Self::Sketchup => "skp",
            Self::SmallWebFormat => "swf",
            Self::Snappy => "sz",
            Self::SonyDsdStreamFile => "dsf",
            Self::SonyMovie => "mqv",
            Self::Sqlite3 => "sqlite",
            Self::TagImageFileFormat => "tiff",
            Self::TapeArchive => "tar",
            Self::Tasty => "tasty",
            Self::ThirdGenerationPartnershipProject => "3gp",
            Self::ThirdGenerationPartnershipProject2 => "3g2",
            #[cfg(feature = "zip")]
            Self::ThreeDimensionalManufacturingFormat => "3mf",
            Self::Truetype => "ttf",
            Self::UnixArchiver => "a",
            Self::UnixCompress => "Z",
            Self::VirtualboxVirtualDiskImage => "vdi",
            Self::Wavpack => "wv",
            Self::WaveformAudio => "wav",
            #[cfg(feature = "zip")]
            Self::WebApplicationArchive => "war",
            Self::WebassemblyBinary => "wasm",
            Self::WebOpenFontFormat => "woff",
            Self::WebOpenFontFormat2 => "woff2",
            Self::Webm => "webm",
            Self::Webp => "webp",
            #[cfg(feature = "zip")]
            Self::WindowsAppPackage => "appx",
            Self::WindowsBitmap => "bmp",
            Self::WindowsMediaVideo => "wmv",
            Self::WindowsMetafile => "wmf",
            Self::WindowsShortcut => "lnk",
            #[cfg(feature = "zip")]
            Self::Xap => "xap",
            #[cfg(feature = "zip")]
            Self::Xpinstall => "xpi",
            Self::Xz => "xz",
            Self::Zip => "zip",
            Self::Zoo => "zoo",
            Self::Zstandard => "zst",
        }
    }

    /// Returns the `Kind` of the `FileFormat`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::{FileFormat, Kind};
    ///
    /// let format = FileFormat::Mpeg12AudioLayer3;
    /// assert_eq!(format.kind(), Kind::Audio);
    ///```
    pub const fn kind(&self) -> Kind {
        match self {
            Self::AdaptiveMultiRate => Kind::Audio,
            Self::AdobeFlashPlayerAudio => Kind::Audio,
            Self::AdobeFlashPlayerAudiobook => Kind::Audio,
            Self::AdobeFlashPlayerProtectedVideo => Kind::Video,
            Self::AdobeFlashPlayerVideo => Kind::Video,
            Self::AdobePhotoshopDocument => Kind::Image,
            Self::AdvancedAudioCoding => Kind::Audio,
            Self::AnimatedPortableNetworkGraphics => Kind::Image,
            Self::AppleIconImage => Kind::Image,
            Self::AppleItunesAudio => Kind::Audio,
            Self::AppleItunesAudiobook => Kind::Audio,
            Self::AppleItunesProtectedAudio => Kind::Audio,
            Self::AppleItunesVideo => Kind::Video,
            Self::AppleQuicktime => Kind::Video,
            Self::Au => Kind::Audio,
            Self::AudioCodec3 => Kind::Audio,
            Self::AudioInterchangeFileFormat => Kind::Audio,
            Self::AudioVideoInterleave => Kind::Video,
            Self::Av1ImageFileFormat => Kind::Image,
            Self::Av1ImageFileFormatSequence => Kind::Image,
            Self::BetterPortableGraphics => Kind::Image,
            Self::CanonRaw2 => Kind::Image,
            Self::CanonRaw3 => Kind::Image,
            Self::Cineon => Kind::Image,
            Self::CreativeVoice => Kind::Audio,
            Self::Cur => Kind::Image,
            #[cfg(feature = "zip")]
            Self::DesignWebFormatXps => Kind::Model,
            Self::DigitalPictureExchange => Kind::Image,
            Self::Djvu => Kind::Image,
            Self::ExperimentalComputingFacility => Kind::Image,
            Self::FastTracker2ExtendedModule => Kind::Audio,
            Self::FlashVideo => Kind::Video,
            Self::FlexibleImageTransportSystem => Kind::Image,
            Self::FreeLosslessAudioCodec => Kind::Audio,
            Self::FreeLosslessImageFormat => Kind::Image,
            Self::FujifilmRaw => Kind::Image,
            Self::GlTransmissionFormatBinary => Kind::Model,
            Self::GraphicsInterchangeFormat => Kind::Image,
            Self::HighEfficiencyImageCoding => Kind::Image,
            Self::HighEfficiencyImageCodingSequence => Kind::Image,
            Self::HighEfficiencyImageFileFormat => Kind::Image,
            Self::HighEfficiencyImageFileFormatSequence => Kind::Image,
            Self::Ico => Kind::Image,
            Self::ImpulseTrackerModule => Kind::Audio,
            Self::JointPhotographicExpertsGroup => Kind::Image,
            Self::Jpeg2000Part1 => Kind::Image,
            Self::Jpeg2000Part2 => Kind::Image,
            Self::Jpeg2000Part3 => Kind::Image,
            Self::Jpeg2000Part6 => Kind::Image,
            Self::JpegExtendedRange => Kind::Image,
            Self::JpegXl => Kind::Image,
            Self::KhronosTexture => Kind::Image,
            Self::KhronosTexture2 => Kind::Image,
            Self::MatroskaVideo => Kind::Video,
            Self::MicrosoftDirectDrawSurface => Kind::Image,
            Self::MonkeysAudio => Kind::Audio,
            Self::Mpeg1Video => Kind::Video,
            Self::Mpeg2TransportStream => Kind::Video,
            Self::Mpeg4Part14Video => Kind::Video,
            Self::Mpeg12AudioLayer3 => Kind::Audio,
            Self::Mpeg1AudioLayer1 => Kind::Audio,
            Self::Mpeg1AudioLayer2 => Kind::Audio,
            Self::Musepack => Kind::Audio,
            Self::MusicalInstrumentDigitalInterface => Kind::Audio,
            Self::NikonElectronicFile => Kind::Image,
            Self::OggFlac => Kind::Audio,
            Self::OggMedia => Kind::Video,
            Self::OggOpus => Kind::Audio,
            Self::OggSpeex => Kind::Audio,
            Self::OggTheora => Kind::Video,
            Self::OggVorbis => Kind::Audio,
            Self::OlympusRawFormat => Kind::Image,
            Self::Openexr => Kind::Image,
            Self::Opentype => Kind::Font,
            Self::PanasonicRaw => Kind::Image,
            Self::PortableNetworkGraphics => Kind::Image,
            Self::QualcommPureVoice => Kind::Audio,
            Self::RadianceHdr => Kind::Image,
            Self::Screamtracker3Module => Kind::Audio,
            Self::SonyDsdStreamFile => Kind::Audio,
            Self::SonyMovie => Kind::Video,
            Self::TagImageFileFormat => Kind::Image,
            Self::ThirdGenerationPartnershipProject => Kind::Video,
            Self::ThirdGenerationPartnershipProject2 => Kind::Video,
            Self::Truetype => Kind::Font,
            Self::Wavpack => Kind::Audio,
            Self::WaveformAudio => Kind::Audio,
            Self::WebOpenFontFormat => Kind::Font,
            Self::WebOpenFontFormat2 => Kind::Font,
            Self::Webm => Kind::Video,
            Self::Webp => Kind::Image,
            Self::WindowsBitmap => Kind::Image,
            Self::WindowsMediaVideo => Kind::Video,
            Self::WindowsMetafile => Kind::Image,
            _ => Kind::Application,
        }
    }

    /// Determines `FileFormat` from bytes.
    ///
    /// If the `FileFormat` is not recognized, the [default value] will be returned.
    ///
    /// # Examples
    ///
    /// Detects from the first bytes of a PNG file:
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

    /// Determines `FileFormat` from a file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_file("fixtures/video/sample.mkv")?;
    /// assert_eq!(format, FileFormat::MatroskaVideo);
    /// # Ok::<(), std::io::Error>(())
    ///```
    #[inline]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_reader(File::open(path)?)
    }

    /// Determines `FileFormat` from a reader.
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
        let mut reader = BufReader::with_capacity(Self::MAX_BYTES, reader);
        Ok(Self::from_signature(reader.fill_buf()?)
            .map(|format| match format {
                #[cfg(feature = "cfb")]
                Self::CompoundFileBinary => Self::from_cfb(reader).unwrap_or_default(),
                Self::MatroskaVideo => Self::from_mkv(reader).unwrap_or_default(),
                Self::MsDosExecutable => Self::from_ms_dos_executable(reader).unwrap_or_default(),
                Self::PortableDocumentFormat => Self::from_pdf(reader).unwrap_or_default(),
                #[cfg(feature = "zip")]
                Self::Zip => Self::from_zip(reader).unwrap_or_default(),
                _ => format,
            })
            .unwrap_or_default())
    }
}

impl Default for FileFormat {
    /// Returns the default `FileFormat` which corresponds to [`FileFormat::ArbitraryBinaryData`].
    #[inline]
    fn default() -> Self {
        Self::ArbitraryBinaryData
    }
}

impl Display for FileFormat {
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

/// Kind of [`FileFormat`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    Application,
    Audio,
    Font,
    Image,
    Model,
    Video,
}
