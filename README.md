# file-format

![Build](https://img.shields.io/github/actions/workflow/status/mmalecot/file-format/ci.yml?branch=main)
[![Crates.io](https://img.shields.io/crates/v/file-format.svg)](https://crates.io/crates/file-format)
[![Docs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![Rust](https://img.shields.io/badge/rust-1.60+-blueviolet.svg?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

Crate for determining the file format of a given file or stream.

It provides a variety of functions for identifying a wide range of file formats, including ZIP,
Compound File Binary (CFB), Extensible Markup Language (XML) and more.

It checks the signature of the file to determine its format. If it is not recognized by its
signature, it returns the default file format which is Arbitrary Binary Data (BIN).

## Examples

Determines from a file:

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_file("fixtures/application/sample.pdf")?;
assert_eq!(format, FileFormat::PortableDocumentFormat);
assert_eq!(format.name(), "Portable Document Format");
assert_eq!(format.short_name(), "PDF");
assert_eq!(format.media_type(), "application/pdf");
assert_eq!(format.extension(), "pdf");
assert_eq!(format.kind(), Kind::Application);
```

Determines from bytes:

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_bytes(&[0xFF, 0xD8, 0xFF]);
assert_eq!(format, FileFormat::JointPhotographicExpertsGroup);
assert_eq!(format.name(), "Joint Photographic Experts Group");
assert_eq!(format.short_name(), "JPEG");
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.extension(), "jpg");
assert_eq!(format.kind(), Kind::Image);
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
file-format = "0.14"
```

## Supported file formats

- 3D Manufacturing Format (3MF)
- 3rd Generation Partnership Project (3GPP)
- 3rd Generation Partnership Project 2 (3GPP2)
- 7-Zip (7Z)
- ALZ
- AV1 Image File Format (AVIF)
- AV1 Image File Format Sequence (AVIFS)
- Adaptive Multi-Rate (AMR)
- Adobe Flash Player Audio (F4A)
- Adobe Flash Player Audiobook (F4B)
- Adobe Flash Player Protected Video (F4P)
- Adobe Flash Player Video (F4V)
- Adobe Illustrator Artwork (AI)
- Adobe InDesign Document (INDD)
- Adobe Photoshop Document (PSD)
- Advanced Audio Coding (AAC)
- Android Binary XML (AXML)
- Android Compiled Resources (ARSC)
- Android Package (APK)
- Animated Portable Network Graphics (APNG)
- Apache Arrow Columnar (Arrow)
- Apache Avro Object Container (Avro)
- Apache Parquet (Parquet)
- Apple Disk Image (DMG)
- Apple Icon Image (ICNS)
- Apple QuickTime (MOV)
- Apple iTunes Audio (M4A)
- Apple iTunes Audiobook (M4B)
- Apple iTunes Protected Audio (M4P)
- Apple iTunes Video (M4V)
- Arbitrary Binary Data (BIN)
- Archived by Robert Jung (ARJ)
- Au
- Audio Codec 3 (AC3)
- Audio Interchange File Format (AIFF)
- Audio Video Interleave (AVI)
- AutoCAD Drawing (DWG)
- Better Portable Graphics (BPG)
- BitTorrent File (Torrent)
- Blender (BLEND)
- Cabinet (CAB)
- Canon Raw 2 (CR2)
- Canon Raw 3 (CR3)
- Cineon (CIN)
- Circuit Diagram Document (CDDX)
- Clojure Script
- Common Object File Format (COFF)
- Compound File Binary (CFB)
- Creative Voice (VOC)
- DER Certificate (DER)
- Dalvik Executable (DEX)
- Debian Binary Package (DEB)
- Design Web Format XPS (DWFX)
- Digital Imaging and Communications in Medicine (DICOM)
- Digital Picture Exchange (DPX)
- DjVu
- Dynamic Link Library (DLL)
- Electronic Publication (EPUB)
- Embedded OpenType (EOT)
- Encapsulated PostScript (EPS)
- Enterprise Application Archive (EAR)
- Executable and Linkable Format (ELF)
- Experimental Computing Facility (XCF)
- Extensible 3D Graphics (X3D)
- Extensible Archive (XAR)
- Extensible Markup Language (XML)
- Extensible Stylesheet Language Transformations (XSLT)
- FastTracker 2 Extended Module (XM)
- Flash Video (FLV)
- Flexible Image Transport System (FITS)
- Free Lossless Audio Codec (FLAC)
- Free Lossless Image Format (FLIF)
- Fujifilm Raw (RAF)
- GL Transmission Format Binary (GLB)
- GPS Exchange Format (GPX)
- Game Boy Advance ROM (GBA)
- Game Boy Color ROM (GBC)
- Game Boy ROM (GB)
- Geography Markup Language (GML)
- Google Chrome Extension (CRX)
- Google Draco (Draco)
- Graphics Interchange Format (GIF)
- High Efficiency Image Coding (HEIC)
- High Efficiency Image Coding Sequence (HEICS)
- High Efficiency Image File Format (HEIF)
- High Efficiency Image File Format Sequence (HEIFS)
- HyperText Markup Language (HTML)
- ICC Profile (ICC)
- ISO 9660 (ISO)
- Impulse Tracker Module (IT)
- InDesign Markup Language (IDML)
- JPEG 2000 Part 1 (JP2)
- JPEG 2000 Part 2 (JPX)
- JPEG 2000 Part 3 (MJ2)
- JPEG 2000 Part 6 (JPM)
- JPEG XL (JXL)
- JPEG-LS (JLS)
- Java Archive (JAR)
- Java Class (Class)
- Java KeyStore (JKS)
- Joint Photographic Experts Group (JPEG)
- Keyhole Markup Language (KML)
- Keyhole Markup Language Zipped (KMZ)
- Khronos Texture (KTX)
- Khronos Texture 2 (KTX2)
- LArc (LZS)
- LHA
- LLVM Bitcode (BC)
- LZ4
- LaTeX (TeX)
- Lempel–Ziv Finite State Entropy (LZFSE)
- Long Range ZIP (LRZIP)
- Lua Bytecode
- Lua Script
- MP3 URL (M3U)
- MPEG-1 Audio Layer 1 (MP1)
- MPEG-1 Audio Layer 2 (MP2)
- MPEG-1 Video (MPG)
- MPEG-1/2 Audio Layer 3 (MP3)
- MPEG-2 Transport Stream (MTS)
- MPEG-4 Part 14 Video (MP4)
- MS-DOS Batch (Batch)
- MS-DOS Executable (EXE)
- Mach-O
- Material Exchange Format (MXF)
- Matroska Video (MKV)
- Meta Information Encapsulation (MIE)
- Microsoft Access 2007 Database (ACCDB)
- Microsoft Access Database (MDB)
- Microsoft Compiled HTML Help (CHM)
- Microsoft DirectDraw Surface (DDS)
- Microsoft Excel Spreadsheet (XLS)
- Microsoft PowerPoint Presentation (PPT)
- Microsoft Project Plan (MPP)
- Microsoft Publisher Document (PUB)
- Microsoft Software Installer (MSI)
- Microsoft Virtual Hard Disk (VHD)
- Microsoft Virtual Hard Disk 2 (VHDX)
- Microsoft Visio Drawing (VSD)
- Microsoft Visual Studio Extension (VSIX)
- Microsoft Visual Studio Solution (SLN)
- Microsoft Word Document (DOC)
- Mobipocket (MOBI)
- Monkey's Audio (APE)
- Musepack (MPC)
- MusicXML
- MusicXML Zipped (MXL)
- Musical Instrument Digital Interface (MIDI)
- Nikon Electronic File (NEF)
- Nintendo 64 ROM (Z64)
- Nintendo DS ROM (NDS)
- Nintendo Entertainment System ROM (NES)
- Office Open XML Document (DOCX)
- Office Open XML Drawing (VSDX)
- Office Open XML Presentation (PPTX)
- Office Open XML Spreadsheet (XLSX)
- Ogg FLAC (OGA)
- Ogg Media (OGM)
- Ogg Multiplexed Media (OGX)
- Ogg Opus (Opus)
- Ogg Speex (Speex)
- Ogg Theora (Theora)
- Ogg Vorbis (Vorbis)
- Olympus Raw Format (ORF)
- OpenDocument Graphics (ODG)
- OpenDocument Presentation (ODP)
- OpenDocument Spreadsheet (ODS)
- OpenDocument Text (ODT)
- OpenEXR (EXR)
- OpenRaster (ORA)
- OpenType (OTF)
- Optimized Dalvik Executable (DEY)
- PCAP Dump (PCAP)
- PCAP Next Generation Dump (PCAPNG)
- PEG Extended Range (JXR)
- PEM Certificate (CRT)
- PEM Certificate Signing Request (CSR)
- PEM Private Key (KEY)
- PGP Message (ASC)
- PGP Private Key Block (ASC)
- PGP Public Key Block (ASC)
- PGP Signature (ASC)
- PGP Signed Message (ASC)
- PMarc (PMA)
- Panasonic Raw (RW2)
- Perl Script
- Personal Storage Table (PST)
- Plain Text (TXT)
- Portable Document Format (PDF)
- Portable Executable (PE)
- Portable Network Graphics (PNG)
- PostScript (PS)
- Python Script
- Qualcomm PureVoice (QCP)
- Radiance HDR (HDR)
- Really Simple Syndication (RSS)
- Red Hat Package Manager (RPM)
- Rich Text Format (RTF)
- Roshal Archive (RAR)
- Ruby Script
- SQLite 3
- Scalable Vector Graphics (SVG)
- ScreamTracker 3 Module (S3M)
- SeqBox (SBX)
- Shapefile (SHP)
- Shell Script
- Simple Object Access Protocol (SOAP)
- SketchUp (SKP)
- Small Web Format (SWF)
- Snappy
- Sony DSD Stream File (DSF)
- Sony Movie (MQV)
- SubRip Text (SRT)
- TASTy
- Tag Image File Format (TIFF)
- Tape Archive (TAR)
- Tool Command Language Script (Tcl Script)
- TrueType (TTF)
- UNIX archiver (archiver)
- UNIX compress (compress)
- VirtualBox Virtual Disk Image (VDI)
- WavPack (WV)
- Waveform Audio (WAV)
- Web Application Archive (WAR)
- Web Open Font Format (WOFF)
- Web Open Font Format 2 (WOFF2)
- WebAssembly Binary (Wasm)
- WebM
- WebP
- Windows Animated Cursor (ANI)
- Windows App Package (APPX)
- Windows Bitmap (BMP)
- Windows Cursor (CUR)
- Windows Icon (ICO)
- Windows Media Video (WMV)
- Windows Metafile (WMF)
- Windows Shortcut (LNK)
- XAP
- XML Localization Interchange File Format (XLIFF)
- XPInstall (XPI)
- XZ
- ZIP
- Zstandard (zstd)
- bzip2 (BZ2)
- cpio
- draw.io (DRAWIO)
- gzip (GZ)
- iCalendar (ICS)
- iOS App Store Package (IPA)
- lzip (LZ)
- lzop (LZO)
- macOS Alias (Alias)
- vCalendar (VCS)
- vCard (VCF)
- zoo

## License

This project is licensed under either of:

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)
