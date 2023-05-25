# Version 0.17.0 (unreleased)

## Fixes

- Fix Additive Manufacturing Format (AMF) detection without XML declaration
- Fix Advanced Stream Redirector (ASX) detection with XML declaration
- Fix AutoCAD Drawing (DWG) media type
- Fix Digital Asset Exchange (DAE) detection without XML declaration
- Fix Drawing Exchange Format ASCII (DXF) media type
- Fix Drawing Exchange Format Binary (DXF) media type
- Fix Extensible 3D Graphics (X3D) detection without XML declaration
- Fix Extensible Stylesheet Language Transformations (XSLT) detection without XML declaration
- Fix GPS Exchange Format (GPX) detection without XML declaration
- Fix Geography Markup Language (GML) detection without XML declaration
- Fix JPEG Extended Range (JXR) name
- Fix Keyhole Markup Language (KML) detection without XML declaration
- Fix Multiple-image Network Graphics (MNG) media type
- Fix MusicXML detection without XML declaration
- Fix PEM Private Key (PEM) signatures
- Fix PEM-based formats short name
- Fix Really Simple Syndication (RSS) detection without XML declaration
- Fix Scalable Vector Graphics (SVG) detection without XML declaration
- Fix Simple Object Access Protocol (SOAP) detection without XML declaration
- Fix XML Localization Interchange File Format (XLIFF) detection without XML declaration
- Fix XML Shareable Playlist Format (XSPF) detection without XML declaration
- Fix draw.io (DRAWIO) detection without XML declaration
- Remove macOS Alias short name

## New formats support

- AbiWord (ABW)
- AbiWord Template (AWT)
- Advanced Compression Engine (ACE)
- Flexible and Interoperable Data Transfer (FIT)
- JPEG 2000 Codestream (J2C)
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
- Training Center XML (TCX)

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
- Adaptable Scalable Texture Compression (ASTC)
- Additive Manufacturing Format (AMF)
- Advanced Media Video (AMV)
- Advanced Stream Redirector (ASX)
- Autodesk Animator (FLI)
- Autodesk Animator Pro (FLC)
- Bitmap Font ASCII (FNT)
- Bitmap Font Binary (FNT)
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

- Apache Avro Object Container (Avro)
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

- Extensible 3D Graphics (X3D)
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
- MPEG-1 Audio Layer 1 (MP1)
- MPEG-1 Audio Layer 2 (MP2)
- MPEG-2 Transport Stream (MTS)
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
- Android Compiled Resources (ARSC)
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
- MPEG-2 Transport Stream (MTS)
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
- Adobe Flash Player Audio (F4A)
- Adobe Flash Player Audiobook (F4B)
- Apache Arrow Columnar (Arrow)
- Apple iTunes Audiobook (M4B)
- Canon Raw 2 (CR2)
- Canon Raw 3 (CR3)
- FastTracker 2 Extended Module (XM)
- Fujifilm Raw (RAF)
- Impulse Tracker Module (IT)
- LHA
- Lempel–Ziv Finite State Entropy (LZFSE)
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
- MPEG-2 Transport Stream (MTS)
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
