#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Result, Seek},
    path::Path,
    str,
};

#[cfg(feature = "zip")]
use zip::ZipArchive;

/// Generates [`FileFormat`] enum.
macro_rules! file_formats {
    {
        $(
            -   variant: $variant:ident
                name: $name:literal
                media_type: $media_type:literal
                extension: $extension:literal
        )*
    } => {
        /// A file format.
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum FileFormat {
            $(
                #[doc=concat!($name, " (", $extension, ")")]
                $variant,
            )*
        }

        impl FileFormat {
            /// Returns the name of the `FileFormat`.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::MpegAudioLayer3;
            /// assert_eq!(format.name(), "MPEG-1/2 Audio Layer III");
            ///```
            pub fn name(&self) -> &str {
                match self {
                    $(
                        FileFormat::$variant => $name,
                    )*
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
            pub fn media_type(&self) -> &str {
                match self {
                    $(
                        FileFormat::$variant => $media_type,
                    )*
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
            pub fn extension(&self) -> &str {
                match self {
                    $(
                        FileFormat::$variant => $extension,
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
            -   file_format: $file_format:ident
                signatures:
                $(
                    -   parts:
                    $(
                        - value: $signature:literal
                          $(offset: $offset:literal)?
                    )+
                )+
        )*
    } => {
        impl FileFormat {
            /// Determines `FileFormat` by checking its signature.
            fn from_signature(bytes: &[u8]) -> Option<FileFormat> {
                $(
                    if
                        $(
                            $(
                                bytes.len() >= $($offset +)? $signature.len()
                                    && &bytes[$($offset)?..$($offset +)? $signature.len()] == $signature
                            )&&*
                        )||*
                    { return Some(FileFormat::$file_format); }
                )*
                None
            }
        }
    };
}

file_formats! {
  - variant: AdaptiveMultiRate
    name: "Adaptive Multi-Rate"
    media_type: "audio/amr"
    extension: "amr"

  - variant: AdobeFlashPlayerAudio
    name: "Adobe Flash Player Audio"
    media_type: "audio/mp4"
    extension: "f4a"

  - variant: AdobeFlashPlayerAudiobook
    name: "Adobe Flash Player Audiobook"
    media_type: "audio/mp4"
    extension: "f4b"

  - variant: AdobeFlashPlayerProtectedVideo
    name: "Adobe Flash Player Protected Video"
    media_type: "video/mp4"
    extension: "f4p"

  - variant: AdobeFlashPlayerVideo
    name: "Adobe Flash Player Video"
    media_type: "video/mp4"
    extension: "f4v"

  - variant: AdobeInDesignDocument
    name: "Adobe InDesign Document"
    media_type: "application/x-indesign"
    extension: "indd"

  - variant: AdobePhotoshopDocument
    name: "Adobe Photoshop Document"
    media_type: "image/vnd.adobe.photoshop"
    extension: "psd"

  - variant: AdvancedAudioCoding
    name: "Advanced Audio Coding"
    media_type: "audio/aac"
    extension: "aac"

  - variant: Alz
    name: "ALZ"
    media_type: "application/x-alz-compressed"
    extension: "alz"

  - variant: AndroidBinaryXml
    name: "Android Binary XML"
    media_type: "application/vnd.android.axml"
    extension: "xml"

  - variant: AndroidCompiledResources
    name: "Android Compiled Resources"
    media_type: "application/vnd.android.arsc"
    extension: "arsc"

  - variant: AndroidPackage
    name: "Android Package"
    media_type: "application/vnd.android.package-archive"
    extension: "apk"

  - variant: Ani
    name: "ANI"
    media_type: "application/x-navi-animation"
    extension: "ani"

  - variant: AnimatedPortableNetworkGraphics
    name: "Animated Portable Network Graphics"
    media_type: "image/apng"
    extension: "apng"

  - variant: ApacheArrowColumnar
    name: "Apache Arrow Columnar"
    media_type: "application/x-apache-arrow"
    extension: "arrow"

  - variant: AppleDiskImage
    name: "Apple Disk Image"
    media_type: "application/x-apple-diskimage"
    extension: "dmg"

  - variant: AppleIconImage
    name: "Apple Icon Image"
    media_type: "image/x-icns"
    extension: "icns"

  - variant: AppleItunesAudio
    name: "Apple iTunes Audio"
    media_type: "audio/x-m4a"
    extension: "m4a"

  - variant: AppleItunesAudiobook
    name: "Apple iTunes Audiobook"
    media_type: "audio/mp4"
    extension: "m4b"

  - variant: AppleItunesProtectedAudio
    name: "Apple iTunes Protected Audio"
    media_type: "audio/mp4"
    extension: "m4p"

  - variant: AppleItunesVideo
    name: "Apple iTunes Video"
    media_type: "video/x-m4v"
    extension: "m4v"

  - variant: AppleQuickTime
    name: "Apple QuickTime"
    media_type: "video/quicktime"
    extension: "mov"

  - variant: ArbitraryBinaryData
    name: "Arbitrary Binary Data"
    media_type: "application/octet-stream"
    extension: "bin"

  - variant: ArchivedByRobertJung
    name: "Archived by Robert Jung"
    media_type: "application/x-arj"
    extension: "arj"

  - variant: Au
    name: "Au"
    media_type: "audio/basic"
    extension: "au"

  - variant: AudioCodec3
    name: "Audio Codec 3"
    media_type: "audio/vnd.dolby.dd-raw"
    extension: "ac3"

  - variant: AudioInterchangeFileFormat
    name: "Audio Interchange File Format"
    media_type: "audio/aiff"
    extension: "aiff"

  - variant: AudioVideoInterleave
    name: "Audio Video Interleave"
    media_type: "video/avi"
    extension: "avi"

  - variant: Av1ImageFileFormat
    name: "AV1 Image File Format"
    media_type: "image/avif"
    extension: "avif"

  - variant: Av1ImageFileFormatSequence
    name: "AV1 Image File Format Sequence"
    media_type: "image/avif-sequence"
    extension: "avifs"

  - variant: BetterPortableGraphics
    name: "Better Portable Graphics"
    media_type: "image/bpg"
    extension: "bpg"

  - variant: Blender
    name: "Blender"
    media_type: "application/x-blender"
    extension: "blend"

  - variant: Bzip2
    name: "bzip2"
    media_type: "application/x-bzip2"
    extension: "bz2"

  - variant: Cabinet
    name: "Cabinet"
    media_type: "application/vnd.ms-cab-compressed"
    extension: "cab"

  - variant: CanonRaw2
    name: "Canon Raw 2"
    media_type: "image/x-canon-cr2"
    extension: "cr2"

  - variant: CanonRaw3
    name: "Canon Raw 3"
    media_type: "image/x-canon-cr3"
    extension: "cr3"

  - variant: Cineon
    name: "Cineon"
    media_type: "image/cineon"
    extension: "cin"

  - variant: CompoundFileBinary
    name: "Compound File Binary"
    media_type: "application/x-cfb"
    extension: "cfb"

  - variant: Cpio
    name: "cpio"
    media_type: "application/x-cpio"
    extension: "cpio"

  - variant: Cur
    name: "CUR"
    media_type: "image/x-icon"
    extension: "cur"

  - variant: DalvikExecutable
    name: "Dalvik Executable"
    media_type: "application/vnd.android.dex"
    extension: "dex"

  - variant: DebianBinaryPackage
    name: "Debian Binary Package"
    media_type: "application/vnd.debian.binary-package"
    extension: "deb"

  - variant: DesignWebFormatXps
    name: "Design Web Format XPS"
    media_type: "model/vnd.dwfx+xps"
    extension: "dwfx"

  - variant: DigitalImagingAndCommunicationsInMedicine
    name: "Digital Imaging and Communications in Medicine"
    media_type: "application/dicom"
    extension: "dcm"

  - variant: DigitalPictureExchange
    name: "Digital Picture Exchange"
    media_type: "image/x-dpx"
    extension: "dpx"

  - variant: ElectronicPublication
    name: "Electronic Publication"
    media_type: "application/epub+zip"
    extension: "epub"

  - variant: EmbeddedOpenType
    name: "Embedded OpenType"
    media_type: "application/vnd.ms-fontobject"
    extension: "eot"

  - variant: ExecutableAndLinkableFormat
    name: "Executable and Linkable Format"
    media_type: "application/x-executable"
    extension: "elf"

  - variant: ExperimentalComputingFacility
    name: "Experimental Computing Facility"
    media_type: "image/x-xcf"
    extension: "xcf"

  - variant: ExtensibleArchive
    name: "Extensible Archive"
    media_type: "application/x-xar"
    extension: "xar"

  - variant: FastTracker2ExtendedModule
    name: "FastTracker 2 Extended Module"
    media_type: "audio/x-xm"
    extension: "xm"

  - variant: FlashVideo
    name: "Flash Video"
    media_type: "video/x-flv"
    extension: "flv"

  - variant: FlexibleImageTransportSystem
    name: "Flexible Image Transport System"
    media_type: "image/fits"
    extension: "fits"

  - variant: FreeLosslessAudioCodec
    name: "Free Lossless Audio Codec"
    media_type: "audio/x-flac"
    extension: "flac"

  - variant: FreeLosslessImageFormat
    name: "Free Lossless Image Format"
    media_type: "image/flif"
    extension: "flif"

  - variant: FujifilmRaw
    name: "Fujifilm Raw"
    media_type: "image/x-fuji-raf"
    extension: "raf"

  - variant: GameBoyAdvanceRom
    name: "Game Boy Advance ROM"
    media_type: "application/x-gba-rom"
    extension: "gba"

  - variant: GameBoyColorRom
    name: "Game Boy Color ROM"
    media_type: "application/x-gameboy-color-rom"
    extension: "gbc"

  - variant: GameBoyRom
    name: "Game Boy ROM"
    media_type: "application/x-gameboy-rom"
    extension: "gb"

  - variant: GlTransmissionFormatBinary
    name: "GL Transmission Format Binary"
    media_type: "model/gltf-binary"
    extension: "glb"

  - variant: GoogleChromeExtension
    name: "Google Chrome Extension"
    media_type: "application/x-google-chrome-extension"
    extension: "crx"

  - variant: GraphicsInterchangeFormat
    name: "Graphics Interchange Format"
    media_type: "image/gif"
    extension: "gif"

  - variant: Gzip
    name: "gzip"
    media_type: "application/gzip"
    extension: "gz"

  - variant: HighEfficiencyImageCoding
    name: "High Efficiency Image Coding"
    media_type: "image/heic"
    extension: "heic"

  - variant: HighEfficiencyImageCodingSequence
    name: "High Efficiency Image Coding Sequence"
    media_type: "image/heic-sequence"
    extension: "heics"

  - variant: HighEfficiencyImageFileFormat
    name: "High Efficiency Image File Format"
    media_type: "image/heif"
    extension: "heif"

  - variant: HighEfficiencyImageFileFormatSequence
    name: "High Efficiency Image File Format Sequence"
    media_type: "image/heif-sequence"
    extension: "heifs"

  - variant: Ico
    name: "ICO"
    media_type: "image/x-icon"
    extension: "ico"

  - variant: ImpulseTrackerModule
    name: "Impulse Tracker Module"
    media_type: "audio/x-it"
    extension: "it"

  - variant: Iso9660
    name: "ISO 9660"
    media_type: "application/x-iso9660-image"
    extension: "iso"

  - variant: JavaArchive
    name: "Java Archive"
    media_type: "application/java-archive"
    extension: "jar"

  - variant: JavaClass
    name: "Java Class"
    media_type: "application/java-vm"
    extension: "class"

  - variant: JavaKeyStore
    name: "Java KeyStore"
    media_type: "application/x-java-keystore"
    extension: "jks"

  - variant: JointPhotographicExpertsGroup
    name: "Joint Photographic Experts Group"
    media_type: "image/jpeg"
    extension: "jpg"

  - variant: Jpeg2000Part1
    name: "JPEG 2000 Part 1"
    media_type: "image/jp2"
    extension: "jp2"

  - variant: Jpeg2000Part2
    name: "JPEG 2000 Part 1"
    media_type: "image/jpx"
    extension: "jpx"

  - variant: Jpeg2000Part3
    name: "JPEG 2000 Part 3"
    media_type: "image/mj2"
    extension: "mj2"

  - variant: Jpeg2000Part6
    name: "JPEG 2000 Part 6"
    media_type: "image/jpm"
    extension: "jpm"

  - variant: JpegExtendedRange
    name: "JPEG Extended Range"
    media_type: "image/jxr"
    extension: "jxr"

  - variant: JpegXl
    name: "JPEG XL"
    media_type: "image/jxl"
    extension: "jxl"

  - variant: KhronosTexture
    name: "Khronos Texture"
    media_type: "image/ktx"
    extension: "ktx"

  - variant: KhronosTexture2
    name: "Khronos Texture 2"
    media_type: "image/ktx2"
    extension: "ktx2"

  - variant: LempelZivFiniteStateEntropy
    name: "Lempel–Ziv Finite State Entropy"
    media_type: "application/x-lzfse"
    extension: "lzfse"

  - variant: Lha
    name: "LHA"
    media_type: "application/x-lzh-compressed"
    extension: "lzh"

  - variant: LongRangeZip
    name: "Long Range ZIP"
    media_type: "application/x-lrzip"
    extension: "lrz"

  - variant: Lz4
    name: "LZ4"
    media_type: "application/x-lz4"
    extension: "lz4"

  - variant: Lzip
    name: "lzip"
    media_type: "application/x-lzip"
    extension: "lz"

  - variant: Lzop
    name: "lzop"
    media_type: "application/x-lzop"
    extension: "lzo"

  - variant: MacOsAlias
    name: "macOS Alias"
    media_type: "application/x-apple-alias"
    extension: "alias"

  - variant: MaterialExchangeFormat
    name: "Material Exchange Format"
    media_type: "application/mxf"
    extension: "mxf"

  - variant: MatroskaVideo
    name: "Matroska Video"
    media_type: "video/x-matroska"
    extension: "mkv"

  - variant: MicrosoftCompiledHtmlHelp
    name: "Microsoft Compiled HTML Help"
    media_type: "application/vnd.ms-htmlhelp"
    extension: "chm"

  - variant: MicrosoftDirectDrawSurface
    name: "Microsoft DirectDraw Surface"
    media_type: "image/vnd-ms.dds"
    extension: "dds"

  - variant: MicrosoftVirtualHardDisk
    name: "Microsoft Virtual Hard Disk"
    media_type: "application/x-vhd"
    extension: "vhd"

  - variant: MicrosoftVirtualHardDisk2
    name: "Microsoft Virtual Hard Disk 2"
    media_type: "application/x-vhdx"
    extension: "vhdx"

  - variant: MicrosoftVisioDrawing
    name: "Microsoft Visio Drawing"
    media_type: "application/vnd.ms-visio.drawing.main+xml"
    extension: "vsdx"

  - variant: MicrosoftVisualStudioExtension
    name: "Microsoft Visual Studio Extension"
    media_type: "application/vsix"
    extension: "vsix"

  - variant: Mobipocket
    name: "Mobipocket"
    media_type: "application/x-mobipocket-ebook"
    extension: "mobi"

  - variant: MonkeysAudio
    name: "Monkey's Audio"
    media_type: "audio/x-ape"
    extension: "ape"

  - variant: Mpeg1Video
    name: "MPEG-1 Video"
    media_type: "video/mpeg"
    extension: "mpg"

  - variant: Mpeg4Part14Video
    name: "MPEG-4 Part 14 Video"
    media_type: "video/mp4"
    extension: "mp4"

  - variant: MpegAudioLayer3
    name: "MPEG-1/2 Audio Layer III"
    media_type: "audio/mpeg"
    extension: "mp3"

  - variant: Musepack
    name: "Musepack"
    media_type: "audio/x-musepack"
    extension: "mpc"

  - variant: MusicalInstrumentDigitalInterface
    name: "Musical Instrument Digital Interface"
    media_type: "audio/midi"
    extension: "mid"

  - variant: NikonElectronicFile
    name: "Nikon Electronic File"
    media_type: "image/x-nikon-nef"
    extension: "nef"

  - variant: Nintendo64Rom
    name: "Nintendo 64 ROM"
    media_type: "application/x-n64-rom"
    extension: "z64"

  - variant: NintendoDsRom
    name: "Nintendo DS ROM"
    media_type: "application/x-nintendo-ds-rom"
    extension: "nds"

  - variant: NintendoEntertainmentSystemRom
    name: "Nintendo Entertainment System ROM"
    media_type: "application/x-nintendo-nes-rom"
    extension: "nes"

  - variant: OfficeOpenXmlDocument
    name: "Office Open XML Document"
    media_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    extension: "docx"

  - variant: OfficeOpenXmlPresentation
    name: "Office Open XML Presentation"
    media_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    extension: "pptx"

  - variant: OfficeOpenXmlWorkbook
    name: "Office Open XML Workbook"
    media_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    extension: "xlsx"

  - variant: OggFlac
    name: "Ogg FLAC"
    media_type: "audio/ogg"
    extension: "oga"

  - variant: OggMedia
    name: "Ogg Media"
    media_type: "video/ogg"
    extension: "ogm"

  - variant: OggMultiplexedMedia
    name: "Ogg Multiplexed Media"
    media_type: "application/ogg"
    extension: "ogx"

  - variant: OggOpus
    name: "Ogg Opus"
    media_type: "audio/opus"
    extension: "opus"

  - variant: OggSpeex
    name: "Ogg Speex"
    media_type: "audio/ogg"
    extension: "spx"

  - variant: OggTheora
    name: "Ogg Theora"
    media_type: "video/ogg"
    extension: "ogv"

  - variant: OggVorbis
    name: "Ogg Vorbis"
    media_type: "audio/ogg"
    extension: "ogg"

  - variant: OlympusRawFormat
    name: "Olympus Raw Format"
    media_type: "image/x-olympus-orf"
    extension: "orf"

  - variant: OpenDocumentGraphics
    name: "OpenDocument Graphics"
    media_type: "application/vnd.oasis.opendocument.graphics"
    extension: "odg"

  - variant: OpenDocumentPresentation
    name: "OpenDocument Presentation"
    media_type: "application/vnd.oasis.opendocument.presentation"
    extension: "odp"

  - variant: OpenDocumentSpreadsheet
    name: "OpenDocument Spreadsheet"
    media_type: "application/vnd.oasis.opendocument.spreadsheet"
    extension: "ods"

  - variant: OpenDocumentText
    name: "OpenDocument Text"
    media_type: "application/vnd.oasis.opendocument.text"
    extension: "odt"

  - variant: OpenExr
    name: "OpenEXR"
    media_type: "image/x-exr"
    extension: "exr"

  - variant: OpenType
    name: "OpenType"
    media_type: "font/otf"
    extension: "otf"

  - variant: OptimizedDalvikExecutable
    name: "Optimized Dalvik Executable"
    media_type: "application/vnd.android.dey"
    extension: "dey"

  - variant: PanasonicRaw
    name: "Panasonic Raw"
    media_type: "image/x-panasonic-rw2"
    extension: "rw2"

  - variant: PcapDump
    name: "PCAP Dump"
    media_type: "application/vnd.tcpdump.pcap"
    extension: "pcap"

  - variant: PcapNextGenerationDump
    name: "PCAP Next Generation Dump"
    media_type: "application/x-pcapng"
    extension: "pcapng"

  - variant: PortableDocumentFormat
    name: "Portable Document Format"
    media_type: "application/pdf"
    extension: "pdf"

  - variant: PortableNetworkGraphics
    name: "Portable Network Graphics"
    media_type: "image/png"
    extension: "png"

  - variant: QualcommPureVoice
    name: "Qualcomm PureVoice"
    media_type: "audio/qcelp"
    extension: "qcp"

  - variant: RadianceHdr
    name: "Radiance HDR"
    media_type: "image/vnd.radiance"
    extension: "hdr"

  - variant: RedHatPackageManager
    name: "Red Hat Package Manager"
    media_type: "application/x-rpm"
    extension: "rpm"

  - variant: RoshalArchive
    name: "Roshal Archive"
    media_type: "application/vnd.rar"
    extension: "rar"

  - variant: ScreamTracker3Module
    name: "ScreamTracker 3 Module"
    media_type: "audio/x-s3m"
    extension: "s3m"

  - variant: SeqBox
    name: "SeqBox"
    media_type: "application/x-sbx"
    extension: "sbx"

  - variant: SevenZip
    name: "7-Zip"
    media_type: "application/x-7z-compressed"
    extension: "7z"

  - variant: Shapefile
    name: "Shapefile"
    media_type: "application/x-esri-shape"
    extension: "shp"

  - variant: SketchUp
    name: "SketchUp"
    media_type: "application/vnd.sketchup.skp"
    extension: "skp"

  - variant: SmallWebFormat
    name: "Small Web Format"
    media_type: "application/x-shockwave-flash"
    extension: "swf"

  - variant: Snappy
    name: "Snappy"
    media_type: "application/x-snappy-framed"
    extension: "sz"

  - variant: SonyDsdStreamFile
    name: "Sony DSD Stream File"
    media_type: "audio/x-dsf"
    extension: "dsf"

  - variant: Sqlite3
    name: "SQLite 3"
    media_type: "application/vnd.sqlite3"
    extension: "sqlite"

  - variant: TagImageFileFormat
    name: "Tag Image File Format"
    media_type: "image/tiff"
    extension: "tiff"

  - variant: TapeArchive
    name: "Tape Archive"
    media_type: "application/x-tar"
    extension: "tar"

  - variant: ThirdGenerationPartnershipProject
    name: "3rd Generation Partnership Project"
    media_type: "video/3gpp"
    extension: "3gp"

  - variant: ThirdGenerationPartnershipProject2
    name: "3rd Generation Partnership Project 2"
    media_type: "video/3gpp2"
    extension: "3g2"

  - variant: ThreeDimensionalManufacturingFormat
    name: "3D Manufacturing Format"
    media_type: "application/vnd.ms-package.3dmanufacturing-3dmodel+xml"
    extension: "3mf"

  - variant: TrueType
    name: "TrueType"
    media_type: "font/ttf"
    extension: "ttf"

  - variant: UnixArchiver
    name: "UNIX archiver"
    media_type: "application/x-archive"
    extension: "ar"

  - variant: UnixCompress
    name: "UNIX compress"
    media_type: "application/x-compress"
    extension: "Z"

  - variant: VirtualBoxVirtualDiskImage
    name: "VirtualBox Virtual Disk Image"
    media_type: "application/x-virtualbox-vdi"
    extension: "vdi"

  - variant: WavPack
    name: "WavPack"
    media_type: "audio/wavpack"
    extension: "wv"

  - variant: WaveformAudio
    name: "Waveform Audio"
    media_type: "audio/vnd.wave"
    extension: "wav"

  - variant: WebAssemblyBinary
    name: "WebAssembly Binary"
    media_type: "application/wasm"
    extension: "wasm"

  - variant: WebOpenFontFormat
    name: "Web Open Font Format"
    media_type: "font/woff"
    extension: "woff"

  - variant: WebOpenFontFormat2
    name: "Web Open Font Format 2"
    media_type: "font/woff2"
    extension: "woff2"

  - variant: WebP
    name: "WebP"
    media_type: "image/webp"
    extension: "webp"

  - variant: WindowsBitmap
    name: "Windows Bitmap"
    media_type: "image/bmp"
    extension: "bmp"

  - variant: WindowsExecutable
    name: "Windows Executable"
    media_type: "application/x-msdownload"
    extension: "exe"

  - variant: WindowsMediaVideo
    name: "Windows Media Video"
    media_type: "video/x-ms-asf"
    extension: "wmv"

  - variant: WindowsMetafile
    name: "Windows Metafile"
    media_type: "image/wmf"
    extension: "wmf"

  - variant: WindowsShortcut
    name: "Windows Shortcut"
    media_type: "application/x-ms-shortcut"
    extension: "lnk"

  - variant: Xap
    name: "XAP"
    media_type: "application/x-silverlight-app"
    extension: "xap"

  - variant: XpInstall
    name: "XPInstall"
    media_type: "application/x-xpinstall"
    extension: "xpi"

  - variant: Xz
    name: "XZ"
    media_type: "application/x-xz"
    extension: "xz"

  - variant: Zip
    name: "ZIP"
    media_type: "application/zip"
    extension: "zip"

  - variant: Zoo
    name: "zoo"
    media_type: "application/x-zoo"
    extension: "zoo"

  - variant: Zstandard
    name: "Zstandard"
    media_type: "application/zstd"
    extension: "zst"
}

signatures! {
  - file_format: VirtualBoxVirtualDiskImage
    signatures:
      - parts:
        - value: b"<<< Oracle VM VirtualBox Disk Image >>>"

  // 32-byte signatures
  - file_format: SketchUp
    signatures:
      - parts:
        - value: b"\xFF\xFE\xFF\x0E\x53\x00\x6B\x00\x65\x00\x74\x00\x63\x00\x68\x00"
        - value: b"\x55\x00\x70\x00\x20\x00\x4D\x00\x6F\x00\x64\x00\x65\x00\x6C\x00"
          offset: 16

  // 29-byte signatures
  - file_format: FlexibleImageTransportSystem
    signatures:
      - parts:
        - value: b"\x49\x4D\x50\x4C\x45\x20\x20\x3D\x20\x20\x20\x20\x20\x20\x20"
        - value: b"\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x54"
          offset: 15

  // 21-byte signatures
  - file_format: DebianBinaryPackage
    signatures:
      - parts:
        - value: b"!<arch>\ndebian-binary"

  // 20-byte signatures
  - file_format: WindowsShortcut
    signatures:
      - parts:
        - value: b"\x4C\x00\x00\x00\x01\x14\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46"

  // 16-byte signatures
  - file_format: AdobeInDesignDocument
    signatures:
      - parts:
        - value: b"\x06\x06\xED\xF5\xD8\x1D\x46\xE5\xBD\x31\xEF\xE7\xFE\x74\xB7\x1D"

  - file_format: FastTracker2ExtendedModule
    signatures:
      - parts:
        - value: b"Extended Module:"

  - file_format: MacOsAlias
    signatures:
      - parts:
        - value: b"\x62\x6F\x6F\x6B\x00\x00\x00\x00\x6D\x61\x72\x6B\x00\x00\x00\x00"

  - file_format: Sqlite3
    signatures:
      - parts:
        - value: b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00"

  - file_format: WindowsMediaVideo
    signatures:
      - parts:
        - value: b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C"

  // 15-byte signatures
  - file_format: FujifilmRaw
    signatures:
      - parts:
        - value: b"FUJIFILMCCD-RAW"

  // 14-byte signatures
  - file_format: MaterialExchangeFormat
    signatures:
      - parts:
        - value: b"\x06\x0E\x2B\x34\x02\x05\x01\x01\x0D\x01\x02\x01\x01\x02"

  // 12-byte signatures
  - file_format: AnimatedPortableNetworkGraphics
    signatures:
      - parts:
        - value: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"
        - value: b"acTL"
          offset: 0x25

  - file_format: JointPhotographicExpertsGroup
    signatures:
      - parts:
        - value: b"\xFF\xD8\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00\x01"
      - parts:
        - value: b"\xFF\xD8\xFF\xE1"
        - value: b"\x45\x78\x69\x66\x00\x00"
          offset: 6
      - parts:
        - value: b"\xFF\xD8\xFF\xDB"
      - parts:
        - value: b"\xFF\xD8\xFF\xEE"

  - file_format: JpegXl
    signatures:
      - parts:
        - value: b"\x00\x00\x00\x0C\x4A\x58\x4C\x20\x0D\x0A\x87\x0A"
      - parts:
        - value: b"\xFF\x0A"

  - file_format: KhronosTexture
    signatures:
      - parts:
        - value: b"\xAB\x4B\x54\x58\x20\x31\x31\xBB\x0D\x0A\x1A\x0A"

  - file_format: KhronosTexture2
    signatures:
      - parts:
        - value: b"\xAB\x4B\x54\x58\x20\x32\x30\xBB\x0D\x0A\x1A\x0A"

  - file_format: MatroskaVideo
    signatures:
      - parts:
        - value: b"\x1A\x45\xDF\xA3"
        - value: b"matroska"
          offset: 24

  - file_format: OggOpus
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"OpusHead"
          offset: 28

  - file_format: PanasonicRaw
    signatures:
      - parts:
        - value: b"\x49\x49\x55\x00\x18\x00\x00\x00\x88\xE7\x74\xD8"

  // 11-byte signatures
  - file_format: OggSpeex
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"Speex  "
          offset: 28

  - file_format: OggTheora
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x80\x74\x68\x65\x6F\x72\x61"
          offset: 28

  - file_format: OggVorbis
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x01\x76\x6F\x72\x62\x69\x73"
          offset: 28

  - file_format: RadianceHdr
    signatures:
      - parts:
        - value: b"\x23\x3F\x52\x41\x44\x49\x41\x4E\x43\x45\x0A"

  // 10-byte signatures
  - file_format: AppleQuickTime
    signatures:
      - parts:
        - value: b"\x00\x00\x00\x14"
        - value: b"ftypqt"
          offset: 4
      - parts:
        - value: b"\x66\x72\x65\x65"
          offset: 4
      - parts:
        - value: b"\x6D\x64\x61\x74"
          offset: 4
      - parts:
        - value: b"\x6D\x6F\x6F\x76"
          offset: 4
      - parts:
        - value: b"\x77\x69\x64\x65"
          offset: 4

  - file_format: OggMedia
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x01\x76\x69\x64\x65\x6F"
          offset: 28

  - file_format: Snappy
    signatures:
      - parts:
        - value: b"\xFF\x06\x00\x00\x73\x4E\x61\x50\x70\x59"

  // 9-byte signatures
  - file_format: GameBoyColorRom
    signatures:
      - parts:
        - value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
          offset: 0x104
        - value: b"\x80"
          offset: 0x143
      - parts:
        - value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
          offset: 0x104
        - value: b"\xC0"
          offset: 0x143

  - file_format: Lzop
    signatures:
      - parts:
        - value: b"\x89\x4C\x5A\x4F\x00\x0D\x0A\x1A\x0A"

  - file_format: MicrosoftVirtualHardDisk
    signatures:
      - parts:
        - value: b"connectix"

  - file_format: OggFlac
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x7F\x46\x4C\x41\x43"
          offset: 28

  - file_format: OlympusRawFormat
    signatures:
      - parts:
        - value: b"\x49\x49\x52\x4F\x08\x00\x00\x00\x18"

  // 8-byte signatures
  - file_format: Ani
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"ACON"
          offset: 8

  - file_format: AudioInterchangeFileFormat
    signatures:
      - parts:
        - value: b"FORM"
        - value: b"AIFF"
          offset: 8
      - parts:
        - value: b"FORM"
        - value: b"AIFC"
          offset: 8

  - file_format: AudioVideoInterleave
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"\x41\x56\x49\x20"
          offset: 8

  - file_format: Av1ImageFileFormat
    signatures:
      - parts:
        - value: b"ftypavif"
          offset: 4

  - file_format: Av1ImageFileFormatSequence
    signatures:
      - parts:
        - value: b"ftypavis"
          offset: 4

  - file_format: CompoundFileBinary
    signatures:
      - parts:
        - value: b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1"

  - file_format: DalvikExecutable
    signatures:
      - parts:
        - value: b"\x64\x65\x78\x0A\x30\x33\x35\x00"

  - file_format: ExperimentalComputingFacility
    signatures:
      - parts:
        - value: b"gimp xcf"

  - file_format: GameBoyAdvanceRom
    signatures:
      - parts:
        - value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"
          offset: 4

  - file_format: GameBoyRom
    signatures:
      - parts:
        - value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
          offset: 0x104

  - file_format: HighEfficiencyImageCoding
    signatures:
      - parts:
        - value: b"ftypheic"
          offset: 4
      - parts:
        - value: b"ftypheix"
          offset: 4

  - file_format: HighEfficiencyImageCodingSequence
    signatures:
      - parts:
        - value: b"ftyphevc"
          offset: 4
      - parts:
        - value: b"ftyphevx"
          offset: 4

  - file_format: HighEfficiencyImageFileFormat
    signatures:
      - parts:
        - value: b"ftypmif1"
          offset: 4

  - file_format: HighEfficiencyImageFileFormatSequence
    signatures:
      - parts:
        - value: b"ftypmsf1"
          offset: 4

  - file_format: Jpeg2000Part1
    signatures:
      - parts:
        - value: b"ftypjp2 "
          offset: 16

  - file_format: Jpeg2000Part2
    signatures:
      - parts:
        - value: b"ftypjpx "
          offset: 16

  - file_format: Jpeg2000Part3
    signatures:
      - parts:
        - value: b"ftypmjp2"
          offset: 16

  - file_format: Jpeg2000Part6
    signatures:
      - parts:
        - value: b"ftypjpm "
          offset: 16

  - file_format: MicrosoftVirtualHardDisk2
    signatures:
      - parts:
        - value: b"vhdxfile"

  - file_format: Mobipocket
    signatures:
      - parts:
        - value: b"BOOKMOBI"
          offset: 60

  - file_format: Mpeg4Part14Video
    signatures:
      - parts:
        - value: b"ftypavc1"
          offset: 4
      - parts:
        - value: b"ftypdash"
          offset: 4
      - parts:
        - value: b"ftypiso2"
          offset: 4
      - parts:
        - value: b"ftypiso3"
          offset: 4
      - parts:
        - value: b"ftypiso4"
          offset: 4
      - parts:
        - value: b"ftypiso5"
          offset: 4
      - parts:
        - value: b"ftypiso6"
          offset: 4
      - parts:
        - value: b"ftypisom"
          offset: 4
      - parts:
        - value: b"ftypmmp4"
          offset: 4
      - parts:
        - value: b"ftypmp41"
          offset: 4
      - parts:
        - value: b"ftypmp42"
          offset: 4
      - parts:
        - value: b"ftypmp4v"
          offset: 4
      - parts:
        - value: b"ftypmp71"
          offset: 4
      - parts:
        - value: b"ftypMSNV"
          offset: 4
      - parts:
        - value: b"ftypNDAS"
          offset: 4
      - parts:
        - value: b"ftypNDSC"
          offset: 4
      - parts:
        - value: b"ftypNDSH"
          offset: 4
      - parts:
        - value: b"ftypNDSM"
          offset: 4
      - parts:
        - value: b"ftypNDSP"
          offset: 4
      - parts:
        - value: b"ftypNDSS"
          offset: 4
      - parts:
        - value: b"ftypNDXC"
          offset: 4
      - parts:
        - value: b"ftypNDXH"
          offset: 4
      - parts:
        - value: b"ftypNDXM"
          offset: 4
      - parts:
        - value: b"ftypNDXP"
          offset: 4

  - file_format: NikonElectronicFile
    signatures:
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
        - value: b"\x1C\x00\xFE\x00"
          offset: 8
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
        - value: b"\x1F\x00\x0B\x00"
          offset: 8
      - parts:
        - value: b"\x49\x49\x2A\x00"
        - value: b"\x1C\x00\xFE\x00"
          offset: 8
      - parts:
        - value: b"\x49\x49\x2A\x00"
        - value: b"\x1F\x00\x0B\x00"
          offset: 8

  - file_format: Nintendo64Rom
    signatures:
      - parts:
        - value: b"\x80\x37\x12\x40\x00\x00\x00\x0F"
      - parts:
        - value: b"\x37\x80\x40\x12\x00\x00\x0F\x00"
      - parts:
        - value: b"\x12\x40\x80\x37\x00\x0F\x00\x00"
      - parts:
        - value: b"\x40\x12\x37\x80\x0F\x00\x00\x00"

  - file_format: NintendoDsRom
    signatures:
      - parts:
        - value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"
          offset: 0xC0
      - parts:
        - value: b"\xC8\x60\x4F\xE2\x01\x70\x8F\xE2"
          offset: 0xC0

  - file_format: PortableNetworkGraphics
    signatures:
      - parts:
        - value: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"

  - file_format: QualcommPureVoice
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"QLCM"
          offset: 8

  - file_format: RoshalArchive
    signatures:
      - parts:
        - value: b"\x52\x61\x72\x21\x1A\x07\x01\x00"
      - parts:
        - value: b"\x52\x61\x72\x21\x1A\x07\x00"

  - file_format: TapeArchive
    signatures:
      - parts:
        - value: b"\x75\x73\x74\x61\x72\x00\x30\x30"
          offset: 257
      - parts:
        - value: b"\x75\x73\x74\x61\x72\x20\x20\x00"
          offset: 257

  - file_format: WaveformAudio
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"WAVE"
          offset: 8

  - file_format: WebP
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"WEBP"
          offset: 8

  // 7-byte signatures
  - file_format: AdobeFlashPlayerAudio
    signatures:
      - parts:
        - value: b"ftypF4A"
          offset: 4

  - file_format: AdobeFlashPlayerAudiobook
    signatures:
      - parts:
        - value: b"ftypF4B"
          offset: 4

  - file_format: AdobeFlashPlayerProtectedVideo
    signatures:
      - parts:
        - value: b"ftypF4P"
          offset: 4

  - file_format: AdobeFlashPlayerVideo
    signatures:
      - parts:
        - value: b"ftypF4V"
          offset: 4

  - file_format: AppleItunesAudio
    signatures:
      - parts:
        - value: b"ftypM4A"
          offset: 4

  - file_format: AppleItunesAudiobook
    signatures:
      - parts:
        - value: b"ftypM4B"
          offset: 4

  - file_format: AppleItunesProtectedAudio
    signatures:
      - parts:
        - value: b"ftypM4P"
          offset: 4

  - file_format: AppleItunesVideo
    signatures:
      - parts:
        - value: b"ftypM4V"
          offset: 4

  - file_format: Blender
    signatures:
      - parts:
        - value: b"BLENDER"

  - file_format: CanonRaw3
    signatures:
      - parts:
        - value: b"ftypcrx"
          offset: 4

  - file_format: ThirdGenerationPartnershipProject
    signatures:
      - parts:
        - value: b"ftyp3gp"
          offset: 4

  - file_format: ThirdGenerationPartnershipProject2
    signatures:
      - parts:
        - value: b"ftyp3g2"
          offset: 4

  - file_format: UnixArchiver
    signatures:
      - parts:
        - value: b"!<arch>"

  // 6-byte signatures
  - file_format: ApacheArrowColumnar
    signatures:
      - parts:
        - value: b"ARROW1"

  - file_format: CanonRaw2
    signatures:
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
        - value: b"CR"
          offset: 8
      - parts:
        - value: b"\x49\x49\x2A\x00"
        - value: b"CR"
          offset: 8

  - file_format: GraphicsInterchangeFormat
    signatures:
      - parts:
        - value: b"GIF87a"
      - parts:
        - value: b"GIF89a"

  - file_format: SevenZip
    signatures:
      - parts:
        - value: b"\x37\x7A\xBC\xAF\x27\x1C"

  - file_format: Xz
    signatures:
      - parts:
        - value: b"\xFD\x37\x7A\x58\x5A\x00"

  // 5-byte signatures
  - file_format: AdaptiveMultiRate
    signatures:
      - parts:
        - value: b"#!AMR"

  - file_format: EmbeddedOpenType
    signatures:
      - parts:
        - value: b"\x00\x00\x01"
          offset: 8
        - value: b"\x4C\x50"
          offset: 34
      - parts:
        - value: b"\x01\x00\x02"
          offset: 8
        - value: b"\x4C\x50"
          offset: 34
      - parts:
        - value: b"\x02\x00\x02"
          offset: 8
        - value: b"\x4C\x50"
          offset: 34

  - file_format: Iso9660
    signatures:
      - parts:
        - value: b"CD001"
          offset: 0x8001
      - parts:
        - value: b"CD001"
          offset: 0x8801
      - parts:
        - value: b"CD001"
          offset: 0x9001

  - file_format: Lha
    signatures:
      - parts:
        - value: b"-lh0-"
          offset: 2
      - parts:
        - value: b"-lh1-"
          offset: 2
      - parts:
        - value: b"-lh2-"
          offset: 2
      - parts:
        - value: b"-lh3-"
          offset: 2
      - parts:
        - value: b"-lh4-"
          offset: 2
      - parts:
        - value: b"-lh5-"
          offset: 2
      - parts:
        - value: b"-lh6-"
          offset: 2
      - parts:
        - value: b"-lh7-"
          offset: 2
      - parts:
        - value: b"-lzs-"
          offset: 2
      - parts:
        - value: b"-lz4-"
          offset: 2
      - parts:
        - value: b"-lz5-"
          offset: 2
      - parts:
        - value: b"-lhd-"
          offset: 2

  - file_format: OpenType
    signatures:
      - parts:
        - value: b"\x4F\x54\x54\x4F\x00"

  - file_format: PortableDocumentFormat
    signatures:
      - parts:
        - value: b"%PDF-"

  - file_format: TrueType
    signatures:
      - parts:
        - value: b"\x00\x01\x00\x00\x00"

  // 4-byte signatures
  - file_format: AdobePhotoshopDocument
    signatures:
      - parts:
        - value: b"8BPS"

  - file_format: Alz
    signatures:
      - parts:
        - value: b"\x41\x4C\x5A\x01"

  - file_format: AndroidBinaryXml
    signatures:
      - parts:
        - value: b"\x03\x00\x08\x00"

  - file_format: AndroidCompiledResources
    signatures:
      - parts:
        - value: b"\x02\x00\x0C\x00"

  - file_format: AppleIconImage
    signatures:
      - parts:
        - value: b"icns"

  - file_format: Au
    signatures:
      - parts:
        - value: b".snd"

  - file_format: BetterPortableGraphics
    signatures:
      - parts:
        - value: b"\x42\x50\x47\xFB"

  - file_format: Cabinet
    signatures:
      - parts:
        - value: b"MSCF"
      - parts:
        - value: b"ISc("

  - file_format: Cineon
    signatures:
      - parts:
        - value: b"\x80\x2A\x5F\xD7"

  - file_format: Cur
    signatures:
      - parts:
        - value: b"\x00\x00\x02\x00"

  - file_format: DigitalImagingAndCommunicationsInMedicine
    signatures:
      - parts:
        - value: b"\x44\x49\x43\x4D"
          offset: 128

  - file_format: DigitalPictureExchange
    signatures:
      - parts:
        - value: b"SDPX"
      - parts:
        - value: b"XPDS"

  - file_format: ExecutableAndLinkableFormat
    signatures:
      - parts:
        - value: b"\x7F\x45\x4C\x46"

  - file_format: ExtensibleArchive
    signatures:
      - parts:
        - value: b"xar!"

  - file_format: FlashVideo
    signatures:
      - parts:
        - value: b"\x46\x4C\x56\x01"

  - file_format: FreeLosslessAudioCodec
    signatures:
      - parts:
        - value: b"fLaC"

  - file_format: FreeLosslessImageFormat
    signatures:
      - parts:
        - value: b"FLIF"

  - file_format: GlTransmissionFormatBinary
    signatures:
      - parts:
        - value: b"glTF"

  - file_format: GoogleChromeExtension
    signatures:
      - parts:
        - value: b"Cr24"

  - file_format: Ico
    signatures:
      - parts:
        - value: b"\x00\x00\x01\x00"

  - file_format: ImpulseTrackerModule
    signatures:
      - parts:
        - value: b"IMPM"

  - file_format: JavaClass
    signatures:
      - parts:
        - value: b"\xCA\xFE\xBA\xBE"

  - file_format: JavaKeyStore
    signatures:
      - parts:
        - value: b"\xFE\xED\xFE\xED"

  - file_format: LempelZivFiniteStateEntropy
    signatures:
      - parts:
        - value: b"bvx-"
      - parts:
        - value: b"bvx1"
      - parts:
        - value: b"bvx2"
      - parts:
        - value: b"bvxn"

  - file_format: LongRangeZip
    signatures:
      - parts:
        - value: b"LRZI"

  - file_format: Lz4
    signatures:
      - parts:
        - value: b"\x04\x22\x4D\x18"

  - file_format: Lzip
    signatures:
      - parts:
        - value: b"LZIP"

  - file_format: MicrosoftCompiledHtmlHelp
    signatures:
      - parts:
        - value: b"ITSF"

  - file_format: MicrosoftDirectDrawSurface
    signatures:
      - parts:
        - value: b"DDS "

  - file_format: MonkeysAudio
    signatures:
      - parts:
        - value: b"MAC "

  - file_format: Mpeg1Video
    signatures:
      - parts:
        - value: b"\x00\x00\x01\xBA"
      - parts:
        - value: b"\x00\x00\x01\xB3"

  - file_format: Musepack
    signatures:
      - parts:
        - value: b"MPCK"
      - parts:
        - value: b"MP+"

  - file_format: MusicalInstrumentDigitalInterface
    signatures:
      - parts:
        - value: b"MThd"

  - file_format: NintendoEntertainmentSystemRom
    signatures:
      - parts:
        - value: b"\x4E\x45\x53\x1A"

  - file_format: OggMultiplexedMedia
    signatures:
      - parts:
        - value: b"OggS"

  - file_format: OpenExr
    signatures:
      - parts:
        - value: b"\x76\x2F\x31\x01"

  - file_format: OptimizedDalvikExecutable
    signatures:
      - parts:
        - value: b"dey\n"

  - file_format: PcapDump
    signatures:
      - parts:
        - value: b"\xA1\xB2\xC3\xD4"
      - parts:
        - value: b"\xD4\xC3\xB2\xA1"

  - file_format: PcapNextGenerationDump
    signatures:
      - parts:
        - value: b"\x0A\x0D\x0D\x0A"

  - file_format: RedHatPackageManager
    signatures:
      - parts:
        - value: b"\xED\xAB\xEE\xDB"

  - file_format: ScreamTracker3Module
    signatures:
      - parts:
        - value: b"SCRM"
          offset: 44

  - file_format: Shapefile
    signatures:
      - parts:
        - value: b"\x00\x00\x27\x0A"

  - file_format: SonyDsdStreamFile
    signatures:
      - parts:
        - value: b"DSD "

  - file_format: TagImageFileFormat
    signatures:
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
      - parts:
        - value: b"\x49\x49\x2A\x00"
      - parts:
        - value: b"\x4D\x4D\x00\x2B"
      - parts:
        - value: b"\x49\x49\x2B\x00"

  - file_format: WavPack
    signatures:
      - parts:
        - value: b"wvpk"

  - file_format: WebAssemblyBinary
    signatures:
      - parts:
        - value: b"\x00\x61\x73\x6D"

  - file_format: WebOpenFontFormat
    signatures:
      - parts:
        - value: b"wOFF"

  - file_format: WebOpenFontFormat2
    signatures:
      - parts:
        - value: b"wOF2"

  - file_format: WindowsMetafile
    signatures:
      - parts:
        - value: b"\xD7\xCD\xC6\x9A"
      - parts:
        - value: b"\x02\x00\x09\x00"
      - parts:
        - value: b"\x01\x00\x09\x00"

  - file_format: Zip
    signatures:
      - parts:
        - value: b"\x50\x4B\x03\x04"
      - parts:
        - value: b"\x50\x4B\x05\x06"
      - parts:
        - value: b"\x50\x4B\x07\x08"

  - file_format: Zstandard
    signatures:
      - parts:
        - value: b"\x28\xB5\x2F\xFD"

  // 3-byte signatures
  - file_format: Bzip2
    signatures:
      - parts:
        - value: b"BZh"

  - file_format: JpegExtendedRange
    signatures:
      - parts:
        - value: b"\x49\x49\xBC"

  - file_format: MpegAudioLayer3
    signatures:
      - parts:
        - value: b"ID3"

  - file_format: SeqBox
    signatures:
      - parts:
        - value: b"SBx"

  - file_format: SmallWebFormat
    signatures:
      - parts:
        - value: b"\x43\x57\x53"
      - parts:
        - value: b"\x46\x57\x53"

  - file_format: Zoo
    signatures:
      - parts:
        - value: b"ZOO"

  // 2-byte signatures
  - file_format: AdvancedAudioCoding
    signatures:
      - parts:
        - value: b"\xFF\xF1"
      - parts:
        - value: b"\xFF\xF9"

  - file_format: AppleDiskImage
    signatures:
      - parts:
        - value: b"\x78\x01"

  - file_format: ArchivedByRobertJung
    signatures:
      - parts:
        - value: b"\x60\xEA"

  - file_format: AudioCodec3
    signatures:
      - parts:
        - value: b"\x0B\x77"

  - file_format: Cpio
    signatures:
      - parts:
        - value: b"\xC7\x71"
      - parts:
        - value: b"\x71\xC7"

  - file_format: Gzip
    signatures:
      - parts:
        - value: b"\x1F\x8B"

  - file_format: UnixCompress
    signatures:
      - parts:
        - value: b"\x1F\xA0"
      - parts:
        - value: b"\x1F\x9D"

  - file_format: WindowsBitmap
    signatures:
      - parts:
        - value: b"BM"

  - file_format: WindowsExecutable
    signatures:
      - parts:
        - value: b"MZ"
}

impl FileFormat {
    /// Maximum number of bytes to read to detect the `FileFormat`.
    const MAX_BYTES: usize = 36870;

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
    pub fn from_bytes(bytes: &[u8]) -> FileFormat {
        FileFormat::from_reader(Cursor::new(bytes)).unwrap_or_default()
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
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
        FileFormat::from_reader(File::open(path)?)
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
    #[cfg(not(feature = "zip"))]
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<FileFormat> {
        let mut reader = BufReader::with_capacity(FileFormat::MAX_BYTES, reader);
        Ok(FileFormat::from_signature(reader.fill_buf()?).unwrap_or_default())
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
    #[cfg(feature = "zip")]
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<FileFormat> {
        let mut reader = BufReader::with_capacity(FileFormat::MAX_BYTES, reader);
        Ok(match FileFormat::from_signature(reader.fill_buf()?) {
            Some(FileFormat::Zip) => FileFormat::from_zip(reader).unwrap_or_default(),
            Some(format) => format,
            _ => FileFormat::default(),
        })
    }

    #[cfg(feature = "zip")]
    fn from_zip<R: Read + Seek>(reader: R) -> Result<FileFormat> {
        let mut archive = ZipArchive::new(reader)?;
        for index in 0..archive.len() {
            let mut file = archive.by_index(index)?;
            let filename = file.name();
            if filename == "AndroidManifest.xml" {
                return Ok(FileFormat::AndroidPackage);
            } else if filename == "AppManifest.xaml" {
                return Ok(FileFormat::Xap);
            } else if filename == "extension.vsixmanifest" {
                return Ok(FileFormat::MicrosoftVisualStudioExtension);
            } else if filename == "META-INF/mozilla.rsa" {
                return Ok(FileFormat::XpInstall);
            } else if filename == "META-INF/MANIFEST.MF" {
                return Ok(FileFormat::JavaArchive);
            } else if filename == "mimetype" {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                match content.as_str() {
                    "application/epub+zip" => {
                        return Ok(FileFormat::ElectronicPublication);
                    }
                    "application/vnd.oasis.opendocument.text" => {
                        return Ok(FileFormat::OpenDocumentText);
                    }
                    "application/vnd.oasis.opendocument.spreadsheet" => {
                        return Ok(FileFormat::OpenDocumentSpreadsheet);
                    }
                    "application/vnd.oasis.opendocument.presentation" => {
                        return Ok(FileFormat::OpenDocumentPresentation);
                    }
                    "application/vnd.oasis.opendocument.graphics" => {
                        return Ok(FileFormat::OpenDocumentGraphics);
                    }
                    _ => {}
                }
            } else if filename.starts_with("dwf/") {
                return Ok(FileFormat::DesignWebFormatXps);
            } else if filename.starts_with("word/") {
                return Ok(FileFormat::OfficeOpenXmlDocument);
            } else if filename.starts_with("ppt/") {
                return Ok(FileFormat::OfficeOpenXmlPresentation);
            } else if filename.starts_with("xl/") {
                return Ok(FileFormat::OfficeOpenXmlWorkbook);
            } else if filename.starts_with("visio/") {
                return Ok(FileFormat::MicrosoftVisioDrawing);
            } else if filename.starts_with("3D/") && filename.ends_with(".model") {
                return Ok(FileFormat::ThreeDimensionalManufacturingFormat);
            }
        }
        Ok(FileFormat::Zip)
    }
}

impl Default for FileFormat {
    /// Returns the default `FileFormat` which corresponds to [`FileFormat::ArbitraryBinaryData`].
    #[inline]
    fn default() -> FileFormat {
        FileFormat::ArbitraryBinaryData
    }
}

impl Display for FileFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
