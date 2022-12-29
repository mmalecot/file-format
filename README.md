# file-format

![Build](https://img.shields.io/github/actions/workflow/status/mmalecot/file-format/ci.yml?branch=main)
[![Crates.io](https://img.shields.io/crates/v/file-format.svg)](https://crates.io/crates/file-format)
[![Docs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![Rust](https://img.shields.io/badge/rust-1.60+-blueviolet.svg?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

Crate for determining the file format of a given file or stream.

It provides a variety of functions for identifying a wide range of file formats, including ZIP, CFB,
XML and more.

It checks the signature of the file to determine its format. If the file format is not recognized by
its signature, it checks if it is Plain Text. Otherwise, it returns the default file format which is
Arbitrary Binary Data.

## Examples

Determines from a file:

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_file("fixtures/application/sample.zip").unwrap();
assert_eq!(format, FileFormat::Zip);
assert_eq!(format.name(), "ZIP");
assert_eq!(format.media_type(), "application/zip");
assert_eq!(format.extension(), "zip");
assert_eq!(format.kind(), Kind::Application);
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
- `cfb` - Enables CFB-based formats support:
    - Microsoft Excel Spreadsheet
    - Microsoft PowerPoint Presentation
    - Microsoft Project Plan
    - Microsoft Publisher Document
    - Microsoft Software Installer
    - Microsoft Visio Drawing
    - Microsoft Word Document
- `zip` - Enables ZIP-based formats support:
    - 3D Manufacturing Format
    - Android Package
    - Circuit Diagram Document
    - Design Web Format XPS
    - Electronic Publication
    - Enterprise Application Archive
    - Java Archive
    - Keyhole Markup Language Zipped
    - Microsoft Visual Studio Extension
    - MusicXML Zipped
    - Office Open XML Document
    - Office Open XML Drawing
    - Office Open XML Presentation
    - Office Open XML Spreadsheet
    - OpenDocument Graphics
    - OpenDocument Presentation
    - OpenDocument Spreadsheet
    - OpenDocument Text
    - OpenRaster
    - Web Application Archive
    - Windows App Package
    - XAP
    - XPInstall
    - iOS App Store Package

All these features are disabled by default.

## Supported formats

- 3D Manufacturing Format
- 3rd Generation Partnership Project
- 3rd Generation Partnership Project 2
- 7-Zip
- ALZ
- AV1 Image File Format
- AV1 Image File Format Sequence
- Adaptive Multi-Rate
- Adobe Flash Player Audio
- Adobe Flash Player Audiobook
- Adobe Flash Player Protected Video
- Adobe Flash Player Video
- Adobe Illustrator Artwork
- Adobe InDesign Document
- Adobe Photoshop Document
- Advanced Audio Coding
- Android Binary XML
- Android Compiled Resources
- Android Package
- Animated Portable Network Graphics
- Apache Arrow Columnar
- Apple Disk Image
- Apple Icon Image
- Apple QuickTime
- Apple iTunes Audio
- Apple iTunes Audiobook
- Apple iTunes Protected Audio
- Apple iTunes Video
- Arbitrary Binary Data
- Archived by Robert Jung
- Au
- Audio Codec 3
- Audio Interchange File Format
- Audio Video Interleave
- Better Portable Graphics
- Blender
- Cabinet
- Canon Raw 2
- Canon Raw 3
- Cineon
- Circuit Diagram Document
- Clojure Script
- Common Object File Format
- Compound File Binary
- Creative Voice
- DER Certificate
- Dalvik Executable
- Debian Binary Package
- Design Web Format XPS
- Digital Imaging and Communications in Medicine
- Digital Picture Exchange
- DjVu
- Dynamic Link Library
- Electronic Publication
- Embedded OpenType
- Encapsulated PostScript
- Enterprise Application Archive
- Executable and Linkable Format
- Experimental Computing Facility
- Extensible Archive
- Extensible Markup Language
- Extensible Stylesheet Language Transformations
- FastTracker 2 Extended Module
- Flash Video
- Flexible Image Transport System
- Free Lossless Audio Codec
- Free Lossless Image Format
- Fujifilm Raw
- GL Transmission Format Binary
- Game Boy Advance ROM
- Game Boy Color ROM
- Game Boy ROM
- Geography Markup Language
- Google Chrome Extension
- Google Draco
- Graphics Interchange Format
- High Efficiency Image Coding
- High Efficiency Image Coding Sequence
- High Efficiency Image File Format
- High Efficiency Image File Format Sequence
- HyperText Markup Language
- ICC Profile
- ISO 9660
- Impulse Tracker Module
- JPEG 2000 Part 1
- JPEG 2000 Part 2
- JPEG 2000 Part 3
- JPEG 2000 Part 6
- JPEG Extended Range
- JPEG XL
- Java Archive
- Java Class
- Java KeyStore
- Joint Photographic Experts Group
- Keyhole Markup Language
- Keyhole Markup Language Zipped
- Khronos Texture
- Khronos Texture 2
- LHA
- LLVM Bitcode
- LZ4
- LaTeX
- Lempel–Ziv Finite State Entropy
- Long Range ZIP
- Lua Bytecode
- Lua Script
- MPEG-1 Audio Layer 1
- MPEG-1 Audio Layer 2
- MPEG-1 Video
- MPEG-1/2 Audio Layer 3
- MPEG-2 Transport Stream
- MPEG-4 Part 14 Video
- MS-DOS Executable
- Mach-O
- Material Exchange Format
- Matroska Video
- Meta Information Encapsulation
- Microsoft Access 2007 Database
- Microsoft Access Database
- Microsoft Compiled HTML Help
- Microsoft DirectDraw Surface
- Microsoft Excel Spreadsheet
- Microsoft PowerPoint Presentation
- Microsoft Project Plan
- Microsoft Publisher Document
- Microsoft Software Installer
- Microsoft Virtual Hard Disk
- Microsoft Virtual Hard Disk 2
- Microsoft Visio Drawing
- Microsoft Visual Studio Extension
- Microsoft Word Document
- Mobipocket
- Monkey’s Audio
- Musepack
- MusicXML
- MusicXML Zipped
- Musical Instrument Digital Interface
- Nikon Electronic File
- Nintendo 64 ROM
- Nintendo DS ROM
- Nintendo Entertainment System ROM
- Office Open XML Document
- Office Open XML Drawing
- Office Open XML Presentation
- Office Open XML Spreadsheet
- Ogg FLAC
- Ogg Media
- Ogg Multiplexed Media
- Ogg Opus
- Ogg Speex
- Ogg Theora
- Ogg Vorbis
- Olympus Raw Format
- OpenDocument Graphics
- OpenDocument Presentation
- OpenDocument Spreadsheet
- OpenDocument Text
- OpenEXR
- OpenRaster
- OpenType
- Optimized Dalvik Executable
- PCAP Dump
- PCAP Next Generation Dump
- PEM Certificate
- PEM Certificate Signing Request
- PEM Private Key
- PGP Message
- PGP Private Key Block
- PGP Public Key Block
- PGP Signature
- PGP Signed Message
- Panasonic Raw
- Perl Script
- Plain Text
- Portable Document Format
- Portable Executable
- Portable Network Graphics
- PostScript
- Python Script
- Qualcomm PureVoice
- Radiance HDR
- Really Simple Syndication
- Red Hat Package Manager
- Rich Text Format
- Roshal Archive
- Ruby Script
- SQLite 3
- Scalable Vector Graphics
- ScreamTracker 3 Module
- SeqBox
- Shapefile
- Shell Script
- Simple Object Access Protocol
- SketchUp
- Small Web Format
- Snappy
- Sony DSD Stream File
- Sony Movie
- TASTy
- Tag Image File Format
- Tape Archive
- Tool Command Language Script
- TrueType
- UNIX archiver
- UNIX compress
- VirtualBox Virtual Disk Image
- WavPack
- Waveform Audio
- Web Application Archive
- Web Open Font Format
- Web Open Font Format 2
- WebAssembly Binary
- WebM
- WebP
- Windows Animated Cursor
- Windows App Package
- Windows Bitmap
- Windows Cursor
- Windows Icon
- Windows Media Video
- Windows Metafile
- Windows Shortcut
- XAP
- XPInstall
- XZ
- ZIP
- Zstandard
- bzip2
- cpio
- gzip
- iOS App Store Package
- lzip
- lzop
- macOS Alias
- vCalendar
- vCard
- zoo

## License

This project is licensed under either of:

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)
