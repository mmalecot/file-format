# file-format

![Build](https://img.shields.io/github/workflow/status/mmalecot/file-format/CI)
[![Crates.io](https://img.shields.io/crates/v/file-format.svg)](https://crates.io/crates/file-format)
[![Docs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![Rust](https://img.shields.io/badge/rust-1.60+-blueviolet.svg?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

Crate for determining **binary-based** file formats.

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
file-format = "0.10"
```

## Features

- **cfb** - Enables **Compound File Binary** formats support. 
- **zip** - Enables **ZIP** formats support.

All these features are disabled by default.

## Supported formats

- 3rd Generation Partnership Project - `3gp`
- 3rd Generation Partnership Project 2 - `3g2`
- 7-Zip - `7z`
- ALZ - `alz`
- AV1 Image File Format - `avif`
- AV1 Image File Format Sequence - `avifs`
- Adaptive Multi-Rate - `amr`
- Adobe Flash Player Audio - `f4a`
- Adobe Flash Player Audiobook - `f4b`
- Adobe Flash Player Protected Video - `f4p`
- Adobe Flash Player Video - `f4v`
- Adobe Illustrator Artwork - `ai`
- Adobe InDesign Document - `indd`
- Adobe Photoshop Document - `psd`
- Advanced Audio Coding - `aac`
- Android Binary XML - `xml`
- Android Compiled Resources - `arsc`
- Animated Portable Network Graphics - `apng`
- Apache Arrow Columnar - `arrow`
- Apple Disk Image - `dmg`
- Apple Icon Image - `icns`
- Apple QuickTime - `mov`
- Apple iTunes Audio - `m4a`
- Apple iTunes Audiobook - `m4b`
- Apple iTunes Protected Audio - `m4p`
- Apple iTunes Video - `m4v`
- Arbitrary Binary Data - `bin`
- Archived by Robert Jung - `arj`
- Au - `au`
- Audio Codec 3 - `ac3`
- Audio Interchange File Format - `aiff`
- Audio Video Interleave - `avi`
- Better Portable Graphics - `bpg`
- Blender - `blend`
- Cabinet - `cab`
- Canon Raw 2 - `cr2`
- Canon Raw 3 - `cr3`
- Cineon - `cin`
- Common Object File Format - `coff`
- Compound File Binary - `cfb`
- Creative Voice - `voc`
- DER Certificate - `der`
- Dalvik Executable - `dex`
- Debian Binary Package - `deb`
- Digital Imaging and Communications in Medicine - `dcm`
- Digital Picture Exchange - `dpx`
- DjVu - `djvu`
- Dynamic Link Library - `dll`
- Embedded OpenType - `eot`
- Encapsulated PostScript - `eps`
- Executable and Linkable Format - `elf`
- Experimental Computing Facility - `xcf`
- Extensible Archive - `xar`
- FastTracker 2 Extended Module - `xm`
- Flash Video - `flv`
- Flexible Image Transport System - `fits`
- Free Lossless Audio Codec - `flac`
- Free Lossless Image Format - `flif`
- Fujifilm Raw - `raf`
- GL Transmission Format Binary - `glb`
- Game Boy Advance ROM - `gba`
- Game Boy Color ROM - `gbc`
- Game Boy ROM - `gb`
- Google Chrome Extension - `crx`
- Google Draco - `drc`
- Graphics Interchange Format - `gif`
- High Efficiency Image Coding - `heic`
- High Efficiency Image Coding Sequence - `heics`
- High Efficiency Image File Format - `heif`
- High Efficiency Image File Format Sequence - `heifs`
- ICC Profile - `icc`
- ISO 9660 - `iso`
- Impulse Tracker Module - `it`
- JPEG 2000 Part 1 - `jp2`
- JPEG 2000 Part 2 - `jpx`
- JPEG 2000 Part 3 - `mj2`
- JPEG 2000 Part 6 - `jpm`
- JPEG Extended Range - `jxr`
- JPEG XL - `jxl`
- Java Class - `class`
- Java KeyStore - `jks`
- Joint Photographic Experts Group - `jpg`
- Khronos Texture - `ktx`
- Khronos Texture 2 - `ktx2`
- LHA - `lzh`
- LLVM Bitcode - `bc`
- LZ4 - `lz4`
- Lempel–Ziv Finite State Entropy - `lzfse`
- Long Range ZIP - `lrz`
- Lua Bytecode - `luac`
- MPEG-1 Audio Layer 1 - `mp1`
- MPEG-1 Audio Layer 2 - `mp2`
- MPEG-1 Video - `mpg`
- MPEG-1/2 Audio Layer 3 - `mp3`
- MPEG-2 Transport Stream - `mts`
- MPEG-4 Part 14 Video - `mp4`
- MS-DOS Executable - `exe`
- Mach-O - `mach`
- Material Exchange Format - `mxf`
- Matroska Video - `mkv`
- Meta Information Encapsulation - `mie`
- Microsoft Access 2007 Database - `accdb`
- Microsoft Access Database - `mdb`
- Microsoft Compiled HTML Help - `chm`
- Microsoft DirectDraw Surface - `dds`
- Microsoft Virtual Hard Disk - `vhd`
- Microsoft Virtual Hard Disk 2 - `vhdx`
- Mobipocket - `mobi`
- Monkey’s Audio - `ape`
- Musepack - `mpc`
- Musical Instrument Digital Interface - `mid`
- Nikon Electronic File - `nef`
- Nintendo 64 ROM - `z64`
- Nintendo DS ROM - `nds`
- Nintendo Entertainment System ROM - `nes`
- Ogg FLAC - `oga`
- Ogg Media - `ogm`
- Ogg Multiplexed Media - `ogx`
- Ogg Opus - `opus`
- Ogg Speex - `spx`
- Ogg Theora - `ogv`
- Ogg Vorbis - `ogg`
- Olympus Raw Format - `orf`
- OpenEXR - `exr`
- OpenType - `otf`
- Optimized Dalvik Executable - `dey`
- PCAP Dump - `pcap`
- PCAP Next Generation Dump - `pcapng`
- PEM Certificate - `crt`
- PEM Certificate Signing Request - `csr`
- PEM Private Key - `key`
- PGP Message - `asc`
- PGP Private Key Block - `asc`
- PGP Public Key Block - `asc`
- PGP Signature - `asc`
- PGP Signed Message - `asc`
- Panasonic Raw - `rw2`
- Portable Document Format - `pdf`
- Portable Executable - `exe`
- Portable Network Graphics - `png`
- PostScript - `ps`
- Qualcomm PureVoice - `qcp`
- Radiance HDR - `hdr`
- Red Hat Package Manager - `rpm`
- Rich Text Format - `rtf`
- Roshal Archive - `rar`
- SQLite 3 - `sqlite`
- ScreamTracker 3 Module - `s3m`
- SeqBox - `sbx`
- Shapefile - `shp`
- SketchUp - `skp`
- Small Web Format - `swf`
- Snappy - `sz`
- Sony DSD Stream File - `dsf`
- Sony Movie - `mqv`
- TASTy - `tasty`
- Tag Image File Format - `tiff`
- Tape Archive - `tar`
- TrueType - `ttf`
- UNIX archiver - `a`
- UNIX compress - `Z`
- VirtualBox Virtual Disk Image - `vdi`
- WavPack - `wv`
- Waveform Audio - `wav`
- Web Open Font Format - `woff`
- Web Open Font Format 2 - `woff2`
- WebAssembly Binary - `wasm`
- WebM - `webm`
- WebP - `webp`
- Windows Animated Cursor - `ani`
- Windows Bitmap - `bmp`
- Windows Cursor - `cur`
- Windows Icon - `ico`
- Windows Media Video - `wmv`
- Windows Metafile - `wmf`
- Windows Shortcut - `lnk`
- XZ - `xz`
- ZIP - `zip`
- Zstandard - `zst`
- bzip2 - `bz2`
- cpio - `cpio`
- gzip - `gz`
- lzip - `lz`
- lzop - `lzo`
- macOS Alias - `alias`
- zoo - `zoo`

Compound File Binary:
- Microsoft Excel Spreadsheet - `xls`
- Microsoft PowerPoint Presentation - `ppt`
- Microsoft Project Plan - `mpp`
- Microsoft Publisher Document - `pub`
- Microsoft Software Installer - `msi`
- Microsoft Visio Drawing - `vsd`
- Microsoft Word Document - `doc`

ZIP:
- 3D Manufacturing Format - `3mf`
- Android Package - `apk`
- Circuit Diagram Document - `cddx`
- Design Web Format XPS - `dwfx`
- Electronic Publication - `epub`
- Enterprise Application Archive - `ear`
- Java Archive - `jar`
- Keyhole Markup Language Zipped - `kmz`
- Microsoft Visual Studio Extension - `vsix`
- Office Open XML Document - `docx`
- Office Open XML Drawing - `vsdx`
- Office Open XML Presentation - `pptx`
- Office Open XML Spreadsheet - `xlsx`
- OpenDocument Graphics - `odg`
- OpenDocument Presentation - `odp`
- OpenDocument Spreadsheet - `ods`
- OpenDocument Text - `odt`
- OpenRaster - `ora`
- Web Application Archive - `war`
- Windows App Package - `appx`
- XAP - `xap`
- XPInstall - `xpi`
- iOS App Store Package - `ipa`

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
