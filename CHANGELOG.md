# Version 0.24.0 (unreleased)

## API

- Add `Compressed` kind
- Add `Diagram` kind
- Add `Ebook` kind
- Add `Formula` kind
- Add `Metadata` kind
- Add `Other` kind
- Add `Presentation` kind
- Add `Spreadsheet` kind
- Change `FileFormat::from_bytes` argument type
- Drop `Application` kind
- Drop `Book` kind
- Drop `Certificate` kind
- Drop `Compression` kind
- Drop `Syndication` kind
- Drop `Text` kind
- Drop `serde` feature
- Rename `AdobeFlashPlayerAudio` to `FlashMp4Audio`
- Rename `AdobeFlashPlayerAudiobook` to `FlashMp4Audiobook`
- Rename `AdobeFlashPlayerProtectedVideo` to `FlashMp4ProtectedVideo`
- Rename `AdobeFlashPlayerVideo` to `FlashMp4Video`
- Rename `AdvancedCompressionEngine` to `Ace`
- Rename `AndroidCompiledResources` to `AndroidResourceStorageContainer`
- Rename `ApacheAvroObjectContainer` to `ApacheAvro`
- Rename `BitmapFontAscii` to `BmfontAscii`
- Rename `BitmapFontBinary` to `BmfontBinary`

## Fixes

- Fix Apache Arrow Columnar (Arrow) media type
- Fix Apache Parquet (Parquet) media type
- Fix Audio Codec 3 (AC-3) media type
- Fix Audio Codec 3 (AC-3) short name
- Fix BMFont ASCII (FNT) media type
- Fix BMFont Binary (FNT) media type
- Fix Dalvik Executable (DEX) signature
- Fix DjVu kind
- Fix EOCD research when using `reader-zip` feature
- Fix Encapsulated PostScript (EPS) kind
- Fix LaTeX (TeX) kind
- Fix Microsoft Project Plan (MPP) kind
- Fix OpenDocument Graphics (ODG) kind
- Fix OpenDocument Graphics Template (OTG) kind
- Fix PostScript (PS) kind
- Fix Small Web Format (SWF) signature
- Fix StarDraw (SDA) kind
- Fix Sun XML Draw (SXD) kind
- Fix Sun XML Draw Template (STD) kind
- Fix WordPerfect Graphics (WPG) kind
- Fix WordPerfect Macro (WPM) kind

## Internal changes

- Refactor and improve readers

## New formats support

- Android App Bundle (AAB)
- AppImage
- Commodore 64 Cartridge (CRT)
- Commodore 64 Program (PRG)
- Corel Presentations (SHW)
- Corel Presentations 7 (SHW)
- Flash CS5 Project (FLA)
- Flash Project (FLA)
- Mozilla Archive (MAR)
- Multi Layer Archive (MLA)
- OpenXPS (OXPS)
- Uniform Office Format Presentation (UOP)
- Uniform Office Format Spreadsheet (UOS)
- Uniform Office Format Text (UOT)
- Windows App Bundle (APPXBUNDLE)
- WordPerfect Presentations (SHW)

# Version 0.23.0 (2023-12-11)

## Fixes

- Fix Neo Geo Pocket ROM (NGP) extension

## Improvements

- Add precision to the JSON Feed signature

## Internal changes

- Improve performance and precision of all readers

## New formats support

- Empty
- Microsoft Write (WRI)
- Neo Geo Pocket Color ROM (NGC)
- Picture Exchange (PCX)

# Version 0.22.0 (2023-11-04)

## Internal changes

- Remove `cfb` dependency when using `reader-cfb` feature
- Remove `zip` dependency when using `reader-zip` feature

# Version 0.21.0 (2023-09-29)

## API

- Add `reader-mp4` feature
- Drop MPEG-1/2 Audio Layer 1 (MP1) support

## Fixes

- Fix MPEG-DASH Manifest (MPD) name

## Improvements

- Add precision to the 3rd Generation Partnership Project (3GPP) signature
- Add precision to the 3rd Generation Partnership Project 2 (3GPP2) signature
- Add precision to the High Efficiency Image File Format (HEIF) signature
- Add precision to the High Efficiency Image File Format Sequence (HEIFS) signature
- Add precision to the JPEG 2000 Part 1 (JP2) signature
- Add precision to the MPEG-4 Part 14 (MP4) signature

## New formats support

- ActiveMime (MSO)
- Amiga Disk File (ADF)
- MPEG-4 Part 14 (MP4)
- MPEG-4 Part 14 Audio (MP4)
- MPEG-4 Part 14 Subtitles (MP4)
- Matroska Subtitles (MKS)
- Silicon Graphics Movie (SGI)

# Version 0.20.0 (2023-09-07)

## New formats support

- MPEG-DASH Manifest (MPD)
- Microsoft Works 6 Spreadsheet (XLR)
- Microsoft Works Database (WDB)
- Microsoft Works Spreadsheet (WKS)
- Microsoft Works Word Processor (WPS)

## Improvements

- Improve Timed Text Markup Language (TTML) detection

# Version 0.19.0 (2023-08-19)

## API

- Add `Database` kind
- Drop Stereolithography Binary (STL) support

## Improvements

- Detect FictionBook Zipped (FBZ) only with files in the root of the archive

## New formats support

- Autodesk 123D (123DX)
- Autodesk Alias (WIRE)
- Autodesk Inventor Assembly (IAM)
- Autodesk Inventor Drawing (IDW)
- Autodesk Inventor Part (IPT)
- Autodesk Inventor Presentation (IPN)
- CD Audio (CDA)
- Cinema 4D (C4D)
- Fusion 360 (F3D)
- Initial Graphics Exchange Specification (IGES)
- Microsoft Reader (LIT)
- SolidWorks Assembly (SLDASM)
- SolidWorks Drawing (SLDDRW)
- SolidWorks Part (SLDPRT)
- Standard for the Exchange of Product model data (STEP)
- Universal Scene Description ASCII (USDA)
- Universal Scene Description Binary (USDC)
- Universal Scene Description Zipped (USDZ)
- Virtual Reality Modeling Language (VRML)
- Windows Media Playlist (WPL)

# Version 0.18.0 (2023-07-14)

## API

- Add `Syndication` kind
- Rename `Mpeg1AudioLayer1` to `Mpeg12AudioLayer1`
- Rename `Mpeg1AudioLayer2` to `Mpeg12AudioLayer2`

## Docs

- Improve internal documentation

## Fixes

- Fix MPEG-1/2 Audio Layer 1 (MP1) name
- Fix MPEG-1/2 Audio Layer 2 (MP2) name

## Improvements

- Add MPEG-1/2 Audio Layer 1 (MP1) signature
- Add MPEG-1/2 Audio Layer 3 (MP3) signature
- Detect Scalable Vector Graphics (SVG) with an uppercase tag

## New formats support

- Atom
- JSON Feed
- Lempel-Ziv-Markov chain algorithm (LZMA)
- Mathematical Markup Language (MathML)
- Squashfs
- StarCalc (SDC)
- StarChart (SDS)
- StarDraw (SDA)
- StarImpress (SDD)
- StarMath (SMF)
- StarWriter (SDW)
- Sun XML Calc (SXC)
- Sun XML Calc Template (STC)
- Sun XML Draw (SXD)
- Sun XML Draw Template (STD)
- Sun XML Impress (SXI)
- Sun XML Impress Template (STI)
- Sun XML Math (SXM)
- Sun XML Writer (SXW)
- Sun XML Writer Global (SGW)
- Sun XML Writer Template (STW)
- Tiled Map XML (TMX)
- Tiled Tileset XML (TSX)
- Windows Imaging Format (WIM)
- WordPerfect Document (WPD)
- WordPerfect Graphics (WPG)
- WordPerfect Macro (WPM)
- openNURBS (3DM)
- rzip (RZ)

# Version 0.17.3 (2023-06-09)

## Fixes

- Fix performance issue on malformed EBML files when using `reader-ebml` feature

# Version 0.17.2 (2023-06-08)

## Fixes

- Fix performance issue on large Matroska files when using `reader-ebml` feature

# Version 0.17.1 (2023-06-07)

## Fixes

- Fix Lempel-Ziv Finite State Entropy (LZFSE) name
- Fix panic when reading an infinite stream via a reader (e.g. /dev/urandom on Linux)

# Version 0.17.0 (2023-06-05)

## API

- Add `Archive` kind
- Add `Book` kind
- Add `Certificate` kind
- Add `Compression` kind
- Add `Disk` kind
- Add `Document` kind
- Add `Executable` kind
- Add `Geospatial` kind
- Add `Package` kind
- Add `Playlist` kind
- Add `Rom` kind
- Add `Subtitle` kind
- Add `reader-asf` feature
- Add `reader-ebml` feature
- Add `reader-rm` feature
- Add `serde` feature
- Change the return type of `FileFormat::short_name` from `&str` to `Option<&str>`
- Drop `accuracy` features
- Drop `reader-mkv` feature
- Rename `AdvancedMediaVideo` to `ActionsMediaVideo`
- Rename `Extensible3dGraphics` to `Extensible3d`
- Rename `Mpeg1Video` to `Mpeg12Video`

## Fixes

- Fix Audio Interchange File Format (AIFF) media type
- Fix AutoCAD Drawing (DWG) media type
- Fix Drawing Exchange Format ASCII (DXF) media type
- Fix Drawing Exchange Format Binary (DXF) media type
- Fix Flexible Image Transport System (FITS) kind and media type
- Fix JPEG 2000 Part 3 (MJ2) kind and media type
- Fix JPEG Extended Range (JXR) name
- Fix MPEG-2 Transport Stream (TS) short name and extension
- Fix Multiple-image Network Graphics (MNG) media type
- Fix PEM Private Key (PEM) signatures
- Fix PEM-based formats short name
- Remove Java Class short name
- Remove macOS Alias short name

## Improvements

- Add precision to the AbiWord (ABW) signature
- Add precision to the AbiWord Template (AWT) signature
- Add precision to the Archived by Robert Jung (ARJ) signature
- Add precision to the Flash MP4 Audio (F4A) signature
- Add precision to the Flash MP4 Audiobook (F4B) signature
- Add precision to the Flash MP4 Protected Video (F4P) signature
- Add precision to the Flash MP4 Video (F4V) signature
- Add precision to the LLVM Bitcode (BC) signature
- Add precision to the MS-DOS Executable (EXE) signature
- Add precision to the Stereolithography Binary (STL) signature
- Add precision to the Windows Bitmap (BMP) signature
- Add precision to the cpio signature
- Detect Additive Manufacturing Format (AMF) without XML declaration
- Detect Advanced Stream Redirector (ASX) with XML declaration
- Detect Digital Asset Exchange (DAE) without XML declaration
- Detect Extensible 3D (X3D) without XML declaration
- Detect Extensible Stylesheet Language Transformations (XSLT) without XML declaration
- Detect GPS Exchange Format (GPX) without XML declaration
- Detect Geography Markup Language (GML) without XML declaration
- Detect Keyhole Markup Language (KML) without XML declaration
- Detect MusicXML without XML declaration
- Detect Really Simple Syndication (RSS) without XML declaration
- Detect Scalable Vector Graphics (SVG) without XML declaration
- Detect Simple Object Access Protocol (SOAP) without XML declaration
- Detect XML Localization Interchange File Format (XLIFF) without XML declaration
- Detect XML Shareable Playlist Format (XSPF) without XML declaration
- Detect draw.io (DRAWIO) without XML declaration
- Distinguish between BDAV MPEG-2 Transport Stream (MT2S) and MPEG-2 Transport Stream (TS)
- Optimize detection of EXE-based file formats when using `reader-exe` feature
- Optimize detection of XML-based file formats when using `reader-xml` feature

## Internal changes

- Update `cfb` dependency from 0.7 to 0.8

## New formats support

- ACE
- AbiWord (ABW)
- AbiWord Template (AWT)
- Adobe Integrated Runtime (AIR)
- Advanced Systems Format (ASF)
- Atari 7800 ROM (A78)
- Audio Visual Research (AVR)
- BZip3 (BZ3)
- Broad Band eBook (BBeB)
- Canon Raw (CRW)
- Design Web Format (DWF)
- FictionBook (FB2)
- FictionBook Zipped (FBZ)
- Flexible and Interoperable Data Transfer (FIT)
- Game Gear ROM (GG)
- IFF 8-Bit Sampled Voice (8SVX)
- JPEG 2000 Codestream (J2C)
- Linear Executable (LE)
- MTV
- Mega Drive ROM (MD)
- Microsoft Digital Video Recording (DVR-MS)
- Neo Geo Pocket ROM (NGP)
- New Executable (NE)
- Nintendo Switch Executable (NSO)
- Nintendo Switch Package (NSP)
- Nintendo Switch ROM (XCI)
- OpenDocument Database (ODB)
- OpenDocument Formula (ODF)
- OpenDocument Formula Template (OTF)
- OpenDocument Graphics Template (OTG)
- OpenDocument Presentation Template (OTP)
- OpenDocument Spreadsheet Template (OTS)
- OpenDocument Text Master (ODM)
- OpenDocument Text Master Template (OTM)
- OpenDocument Text Template (OTT)
- PEM Public Key (PEM)
- QEMU Copy On Write (QCOW)
- RealAudio (RA)
- RealMedia (RM)
- RealVideo (RV)
- Sega Master System ROM (SMS)
- SpaceClaim Document (SCDOC)
- StuffIt (SIT)
- StuffIt X (SITX)
- Timed Text Markup Language (TTML)
- Training Center XML (TCX)
- Universal Subtitle Format (USF)
- Virtual Machine Disk (VMDK)
- Web Video Text Tracks (WebVTT)
- Windows Media Audio (WMA)
- Windows Recorded TV Show (WTV)
- Xbox 360 Executable (XEX)
- Xbox Executable (XBE)
- ZPAQ
- bzip (BZ)

# Version 0.16.0 (2023-03-28)

## API

- Differentiate Polygon ASCII (PLY) and Polygon Binary (PLY)
- Differentiate Stereolithography ASCII (STL) and Stereolithography Binary (STL)
- Rename `Extensible3DGraphics` to `Extensible3dGraphics`
- Rename `FastTracker2ExtendedModule` to `Fasttracker2ExtendedModule`
- Rename `MicrosoftDirectDrawSurface` to `MicrosoftDirectdrawSurface`
- Rename `OpenDocumentGraphics` to `OpendocumentGraphics`
- Rename `OpenDocumentPresentation` to `OpendocumentPresentation`
- Rename `OpenDocumentSpreadsheet` to `OpendocumentSpreadsheet`
- Rename `OpenDocumentText` to `OpendocumentText`
- Rename `QualcommPureVoice` to `QualcommPurevoice`
- Rename `Screamtracker3Module` to `ScreamTracker3Module`

## Fixes

- Fix Flexible Image Transport System (FITS) signature
- Fix Microsoft DirectDraw Surface (DDS) media type
- Fix PGP-based formats short name
- Fix Portable GrayMap (PGM) variant name
- Fix Scream Tracker 3 Module (S3M) name

## Improvements

- Add additional CLSIDs for some Compound File Binary (CFB) based file formats
- Add detection of HyperText Markup Language (HTML) files with a BOM
- Add detection of SubRip Text (SRT) files with a BOM
- Add precision to the Polygon ASCII (PLY) and Polygon Binary (PLY) signatures
- Add unit test for LLVM Bitcode (BC)

## New formats support

- 3D Studio (3DS)
- 3D Studio Max (MAX)
- Actions Media Video (AMV)
- Adaptable Scalable Texture Compression (ASTC)
- Additive Manufacturing Format (AMF)
- Advanced Stream Redirector (ASX)
- Autodesk Animator (FLI)
- Autodesk Animator Pro (FLC)
- BMFont ASCII (FNT)
- BMFont Binary (FNT)
- Drawing Exchange Format ASCII (DXF)
- Drawing Exchange Format Binary (DXF)
- Inter-Quake Export (IQE)
- Inter-Quake Model (IQM)
- JPEG Network Graphics (JNG)
- MagicaVoxel (VOX)
- Magick Image File Format (MIFF)
- Maya ASCII (MA)
- Maya Binary (MB)
- Model 3D ASCII (A3D)
- Model 3D Binary (M3D)
- Multiple-image Network Graphics (MNG)
- Portable FloatMap (PFM)
- Quite OK Audio (QOA)
- Quite OK Image (QOI)
- SHOUTcast Playlist (PLS)
- Silicon Graphics Image (SGI)
- SoundFont 2 (SF2)
- Ultimate Soundtracker Module (MOD)
- Universal 3D (U3D)
- WebAssembly Text (WAT)
- X PixMap (XPM)
- XML Shareable Playlist Format (XSPF)
- gettext Machine Object (MO)

# Version 0.15.0 (2023-03-06)

## Improvements

- Add detection of Extensible Markup Language (XML) files with a BOM

## Internal changes

- Make the file format kind optional in the declaration

## New formats support

- Digital Asset Exchange (DAE)
- Filmbox (FBX)
- InDesign Markup Language (IDML)
- MS-DOS Batch (Batch)
- Microsoft Visual Studio Solution (SLN)
- Polygon File Format (PLY)
- Portable Arbitrary Map (PAM)
- Portable BitMap (PBM)
- Portable GrayMap (PGM)
- Portable PixMap (PPM)
- Stereolithography (STL)
- SubRip Text (SRT)
- draw.io (DRAWIO)
- farbfeld (FF)

# Version 0.14.0 (2023-01-30)

## API

- Add a short name for Tool Command Language Script (Tcl Script)
- Add a short name for UNIX archiver (archiver)
- Add a short name for UNIX compress (compress)

## New formats support

- Apache Avro (Avro)
- Apache Parquet (Parquet)
- AutoCAD Drawing (DWG)
- BitTorrent File (Torrent)
- JPEG-LS (JLS)
- MP3 URL (M3U)
- Personal Storage Table (PST)

# Version 0.13.0 (2023-01-22)

## API

- Add `FileFormat::short_name`
- Improve crate features
- Change the media type of Adobe Illustrator Artwork (AI) to `application/vnd.adobe.illustrator`

## Docs

- Improve global documentation

## Fixes

- Fix LHA signatures
- Fix vCalendar (VCS) extension

## Internal changes

- Improve the detection of Extensible Markup Language (XML) based file formats
- Move the declaration of the file formats to a dedicated module

## New formats support

- Extensible 3D (X3D)
- GPS Exchange Format (GPX)
- LArc (LZS)
- PMarc (PMA)
- XML Localization Interchange File Format (XLIFF)
- iCalendar (ICS)

# Version 0.12.0 (2022-12-30)

## API

- Add `accuracy` feature

## Docs

- Improve global documentation
- Simplify `lefi` example

## New formats support

- LaTeX (TeX)

# Version 0.11.0 (2022-12-21)

## Docs

- Remove dependencies of `lefi` example

## New formats support

- Clojure Script
- Extensible Markup Language (XML)
- Extensible Stylesheet Language Transformations (XSLT)
- Geography Markup Language (GML)
- HyperText Markup Language (HTML)
- Keyhole Markup Language (KML)
- Lua Script
- MusicXML
- MusicXML Zipped (MXL)
- Perl Script
- Plain Text (TXT)
- Python Script
- Really Simple Syndication (RSS)
- Ruby Script
- Scalable Vector Graphics (SVG)
- Shell Script
- Simple Object Access Protocol (SOAP)
- Tool Command Language Script (Tcl Script)
- vCalendar (VCS)
- vCard (VCF)

# Version 0.10.0 (2022-12-04)

## Fixes

- Fix inverted names between `MatroskaVideo` and `MetaInformationEncapsulation`

## New formats support

- Common Object File Format (COFF)
- Google Draco (Draco)
- ICC Profile (ICC)
- Mach-O
- OpenRaster (ORA)

# Version 0.9.1 (2022-11-30)

## Docs

- Fix `README.md`

# Version 0.9.0 (2022-11-30)

## API

- Add `FileFormat::kind`
- Rename `AdobeInDesignDocument` to `AdobeIndesignDocument`
- Rename `Ani` to `WindowsAnimatedCursor`
- Rename `AppleQuickTime` to `AppleQuicktime`
- Rename `Cur` to `WindowsCursor`
- Rename `EmbeddedOpenType` to `EmbeddedOpentype`
- Rename `Ico` to `WindowsIcon`
- Rename `JavaKeyStore` to `JavaKeystore`
- Rename `MacOsAlias` to `MacosAlias`
- Rename `MicrosoftVisioDrawing` to `OfficeOpenXmlDrawing`
- Rename `MpegAudioLayer3` to `Mpeg12AudioLayer3`
- Rename `OfficeOpenXmlWorkbook` to `OfficeOpenXmlSpreadsheet`
- Rename `OpenExr` to `Openexr`
- Rename `OpenType` to `Opentype`
- Rename `ScreamTracker3Module` to `Screamtracker3Module`
- Rename `SeqBox` to `Seqbox`
- Rename `SketchUp` to `Sketchup`
- Rename `TrueType` to `Truetype`
- Rename `VirtualBoxVirtualDiskImage` to `VirtualboxVirtualDiskImage`
- Rename `WavPack` to `Wavpack`
- Rename `WebAssemblyBinary` to `WebassemblyBinary`
- Rename `WebP` to `Webp`
- Rename `WindowsExecutable` to `MsDosExecutable`
- Rename `XpInstall` to `Xpinstall`

## Fixes

- Add MPEG-1/2 Audio Layer 3 (MP3) signature
- Add Matroska Video (MKV) signature
- Change UNIX archiver (archiver) extension from `ar` to `a` (preferred)
- Fix Apple QuickTime (MOV) signature
- Fix Joint Photographic Experts Group (JPEG) signature

## Internal changes

- Add `formats` macro
- Split items into modules

## New formats support

- Adobe Illustrator Artwork (AI)
- Circuit Diagram Document (CDDX)
- Creative Voice (VOC)
- DER Certificate (DER)
- DjVu
- Dynamic Link Library (DLL)
- Encapsulated PostScript (EPS)
- Enterprise Application Archive (EAR)
- Keyhole Markup Language Zipped (KMZ)
- LLVM Bitcode (BC)
- Lua Bytecode
- MPEG-1/2 Audio Layer 1 (MP1)
- MPEG-1/2 Audio Layer 2 (MP2)
- MPEG-2 Transport Stream (TS)
- Meta Information Encapsulation (MIE)
- Microsoft Access 2007 Database (ACCDB)
- Microsoft Access Database (MDB)
- Microsoft Excel Spreadsheet (XLS)
- Microsoft PowerPoint Presentation (PPTX)
- Microsoft Project Plan (MPP)
- Microsoft Publisher Document (PUB)
- Microsoft Software Installer (MSI)
- Microsoft Visio Drawing (VSD)
- Microsoft Word Document (DOC)
- PEM Certificate (PEM)
- PEM Certificate Signing Request (PEM)
- PEM Private Key (PEM)
- PGP Message (PGP)
- PGP Private Key Block (PGP)
- PGP Public Key Block (PGP)
- PGP Signature (PGP)
- PGP Signed Message (PGP)
- Portable Executable (PE)
- PostScript (PS)
- Rich Text Format (RTF)
- Sony Movie (MQV)
- TASTy
- Web Application Archive (WAR)
- WebM
- Windows App Package (APPX)
- iOS App Store Package (IPA)

# Version 0.8.0 (2022-11-06)

## API

- Add `FileFormat::from_reader`
- Add `impl From<&[u8]> for FileFormat`

## Docs

- Add `lefi` example
- Add `CHANGELOG.md`

## Internal changes

- Make signature offset optional
- Remove `FileFormat` enum generation with macro
- Simplify `signatures` macro

## New formats support

- 3D Manufacturing Format (3MF)
- Android Package (APK)
- Design Web Format XPS (DWFX)
- Electronic Publication (EPUB)
- Java Archive (JAR)
- Microsoft Visio Drawing (VSD)
- Microsoft Visual Studio Extension (VSIX)
- Office Open XML Document (DOCX)
- Office Open XML Presentation (PPTX)
- Office Open XML Spreadsheet (XLSX)
- OpenDocument Graphics (ODG)
- OpenDocument Presentation (ODP)
- OpenDocument Spreadsheet (ODS)
- OpenDocument Text (ODT)
- XAP
- XPInstall (XPI)

# Version 0.7.0 (2022-08-22)

## New formats support

- Android Binary XML (AXML)
- Android Resource Storage Container (ARSC)
- Optimized Dalvik Executable (DEY)

# Version 0.6.0 (2021-12-18)

## API

- Add `FileFormat::from_bytes`

# Version 0.5.0 (2021-12-12)

## API

- Switch back `FileFormat` type from a structure to an enum

## Discontinued formats

- 3D Manufacturing Format (3MF)
- Java Archive (JAR)
- MPEG-2 Transport Stream (TS)
- Microsoft Visio Drawing (VSD)
- Office Open XML Document (DOCX)
- Office Open XML Presentation (PPTX)
- Office Open XML Spreadsheet (XLSX)
- OpenDocument Graphics (ODG)
- OpenDocument Presentation (ODP)
- OpenDocument Spreadsheet (ODS)
- OpenDocument Text (ODT)
- Web Application Resource (WAR)
- XAP
- XPInstall (XPI)

## Improvements

- Add precision to the Dalvik Executable (DEX) signature
- Switch to Rust 2021

## New formats support

- Java KeyStore (JKS)

# Version 0.4.0 (2021-10-22)

## Docs

- Reorganize supported file formats table

## Improvements

- Add tests for all High Efficiency Image Coding Sequence (HEICS) format
- Add tests for all High Efficiency Image Coding (HEIC) format

## New formats support

- 3D Manufacturing Format (3MF)
- Java Archive (JAR)
- Microsoft DirectDraw Surface (DDS)
- Microsoft Visio Drawing (VSD)
- Office Open XML Document (DOCX)
- Office Open XML Presentation (PPTX)
- Office Open XML Spreadsheet (XLSX)
- Radiance HDR (HDR)
- Web Application Resource (WAR)
- XAP
- XPInstall (XPI)

# Version 0.3.0 (2021-10-18)

## API

- Switch `FileFormat` type from an enum to a structure

## Discontinued formats

- HyperText Markup Language (HTML)

## Fixes

- Use of the correct Tag Image File Format (TIFF) extension

## Improvements

- Add new Apple QuickTime (MOV) signatures
- Add new Audio Interchange File Format (AIFF) signature
- Add precision to the Debian Binary Package (DEB) signature
- Add precision to the Flexible Image Transport System (FITS) signature
- Add precision to the Windows Media Video (WMV) signature
- Add precision to the Windows Shortcut (LNK) signature
- Improve support of some file formats
- Replace Microsoft Software Installer (MSI) by Compound File Binary (CFB)

## New formats support

- ALZ
- Apache Arrow Columnar (Arrow)
- Apple iTunes Audiobook (M4B)
- Canon Raw 2 (CR2)
- Canon Raw 3 (CR3)
- FastTracker 2 Extended Module (XM)
- Flash MP4 Audio (F4A)
- Flash MP4 Audiobook (F4B)
- Fujifilm Raw (RAF)
- Impulse Tracker Module (IT)
- LHA
- Lempel-Ziv Finite State Entropy (LZFSE)
- Microsoft Compiled HTML Help (CHM)
- Microsoft Virtual Hard Disk (VHD)
- Microsoft Virtual Hard Disk 2 (VHDX)
- Nikon Electronic File (NEF)
- Panasonic Raw (RW2)
- Qualcomm PureVoice (QCP)
- ScreamTracker 3 Module (S3M)
- SeqBox (SBX)
- Snappy
- Sony DSD Stream File (DSF)
- Windows Animated Cursor (ANI)
- Windows Cursor (CUR)
- cpio
- macOS Alias
- zoo

# Version 0.2.1 (2021-10-14)

## Fixes

- Fix Tag Image File Format (TIFF) signature

# Version 0.2.0 (2021-10-07)

## New formats support

- Animated Portable Network Graphics (APNG)
- Electronic Publication (EPUB)
- Game Boy Color ROM (GBC)
- HyperText Markup Language (HTML)
- Khronos Texture (KTX)
- Khronos Texture 2 (KTX2)
- MPEG-2 Transport Stream (TS)
- Material Exchange Format (MXF)
- Mobipocket (MOBI)
- Olympus Raw Format (ORF)
- OpenDocument Graphics (ODG)
- OpenDocument Presentation (ODP)
- OpenDocument Spreadsheet (ODS)
- OpenDocument Text (ODT)
- Rich Text Format (RTF)
- Shapefile (SHP)
- SketchUp (SKP)
- UNIX archiver (archiver)

# Version 0.1.0 (2021-10-03)

First version.
