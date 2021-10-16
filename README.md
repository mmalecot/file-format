# file-format

![Build](https://img.shields.io/github/workflow/status/mmalecot/file-format/CI)
[![Crates.io](https://img.shields.io/crates/v/file-format.svg)](https://crates.io/crates/file-format)
[![Docs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![Rust](https://img.shields.io/badge/rust-1.54+-blueviolet.svg?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

File format library for Rust.

This crate is for recognizing binary-based file formats.

## Examples

Determines from a file:

```rust
use file_format::FileFormat;

let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.extension(), "jpg");
```

Determines from bytes:

```rust
use file_format::FileFormat;

let format = FileFormat::from_bytes(&[0x66, 0x4C, 0x61, 0x43]);
assert_eq!(format.media_type(), "audio/x-flac");
assert_eq!(format.extension(), "flac");
```

## Supported file formats

| Media type                                      | Extension | Description                                    |
|-------------------------------------------------|-----------|------------------------------------------------|
| application/dicom                               | dcm       | Digital Imaging and Communications in Medicine |
| application/epub+zip                            | epub      | Electronic Publication                         |
| application/gzip                                | gz        | Gzip                                           |
| application/java-vm                             | class     | Java class                                     |
| application/mxf                                 | mxf       | Material Exchange Format                       |
| application/octet-stream                        | bin       | Arbitrary binary data                          |
| application/ogg                                 | ogx       | Ogg Multiplexed Media                          |
| application/pdf                                 | pdf       | Portable Document Format                       |
| application/vnd.android.dex                     | dex       | Dalvik Executable                              |
| application/vnd.debian.binary-package           | deb       | Debian package                                 |
| application/vnd.ms-cab-compressed               | cab       | Cabinet                                        |
| application/vnd.ms-fontobject                   | eot       | Embedded OpenType                              |
| application/vnd.ms-htmlhelp                     | chm       | Microsoft Compiled HTML Help                   |
| application/vnd.oasis.opendocument.graphics     | odg       | OpenDocument Graphics                          |
| application/vnd.oasis.opendocument.presentation | odp       | OpenDocument Presentation                      |
| application/vnd.oasis.opendocument.spreadsheet  | ods       | OpenDocument SpreadSheet                       |
| application/vnd.oasis.opendocument.text         | odt       | OpenDocument Text                              |
| application/vnd.rar                             | rar       | Roshal ARchive                                 |
| application/vnd.sketchup.skp                    | skp       | SketchUp document                              |
| application/vnd.sqlite3                         | sqlite    | SQLite 3 database                              |
| application/vnd.tcpdump.pcap                    | pcap      | Libpcap                                        |
| application/wasm                                | wasm      | WebAssembly binary                             |
| application/x-7z-compressed                     | 7z        | 7z                                             |
| application/x-apache-arrow                      | arrow     | Apache Arrow Columnar                          |
| application/x-apple-diskimage                   | dmg       | Apple Disk Image                               |
| application/x-archive                           | ar        | Unix Archiver                                  |
| application/x-blender                           | blend     | Blender 3D data                                |
| application/x-bzip2                             | bz2       | Bzip2                                          |
| application/x-compress                          | Z         | Unix compress                                  |
| application/x-cpio                              | cpio      | Cpio                                           |
| application/x-esri-shape                        | shp       | Shapefile                                      |
| application/x-executable                        | elf       | Executable and Linkable Format                 |
| application/x-gameboy-color-rom                 | gbc       | Game Boy Color ROM                             |
| application/x-gameboy-rom                       | gb        | Game Boy ROM                                   |
| application/x-gba-rom                           | gba       | Game Boy Advance ROM                           |
| application/x-google-chrome-extension           | crx       | Google Chrome Extension                        |
| application/x-indesign                          | indd      | Adobe InDesign document                        |
| application/x-iso9660-image                     | iso       | ISO 9660 image                                 |
| application/x-lrzip                             | lrz       | Long Range ZIP                                 |
| application/x-lz4                               | lz4       | LZ4                                            |
| application/x-lzh-compressed                    | lzh       | LHA                                            |
| application/x-lzip                              | lz        | Lzip                                           |
| application/x-lzop                              | lzo       | Lzop                                           |
| application/x-mobipocket-ebook                  | mobi      | Mobipocket                                     |
| application/x-ms-shortcut                       | lnk       | Windows shortcut                               |
| application/x-msdownload                        | exe       | Windows executable                             |
| application/x-n64-rom                           | z64       | Nintendo 64 ROM                                |
| application/x-nintendo-ds-rom                   | nds       | Nintendo DS ROM                                |
| application/x-nintendo-nes-rom                  | nes       | Nintendo Entertainment System ROM              |
| application/x-ole-storage                       | msi       | Windows Installer                              |
| application/x-pcapng                            | pcapng    | Pcap-NG Packet Capture                         |
| application/x-rpm                               | rpm       | Red Hat Package Manager package                |
| application/x-sbx                               | sbx       | SeqBox                                         |
| application/x-shockwave-flash                   | swf       | Small Web Format                               |
| application/x-snappy-framed                     | sz        | Snappy                                         |
| application/x-tar                               | tar       | Tape archive                                   |
| application/x-virtualbox-vdi                    | vdi       | VirtualBox Virtual Disk Image                  |
| application/x-xar                               | xar       | eXtensible ARchive format                      |
| application/x-xz                                | xz        | XZ                                             |
| application/zip                                 | zip       | ZIP                                            |
| application/zstd                                | zst       | Zstandard                                      |
| audio/aac                                       | aac       | Advanced Audio Coding                          |
| audio/aiff                                      | aif       | Audio Interchange File Format                  |
| audio/amr                                       | amr       | Adaptive Multi-Rate                            |
| audio/basic                                     | au        | Au                                             |
| audio/midi                                      | mid       | Musical Instrument Digital Interface           |
| audio/mp4                                       | f4a       | Adobe Flash Player Audio                       |
| audio/mp4                                       | f4b       | Adobe Flash Player Audiobook                   |
| audio/mp4                                       | m4b       | Apple iTunes ALAC/AAC-LC Audiobook             |
| audio/mpeg                                      | mp3       | MPEG-1/2 Audio Layer III                       |
| audio/ogg                                       | oga       | Ogg FLAC                                       |
| audio/ogg                                       | ogg       | Ogg Vorbis                                     |
| audio/ogg                                       | spx       | Ogg Speex                                      |
| audio/opus                                      | opus      | Ogg Opus                                       |
| audio/vnd.dolby.dd-raw                          | ac3       | Audio Codec 3 (AC-3)                           |
| audio/vnd.wave                                  | wav       | Waveform Audio                                 |
| audio/wavpack                                   | wv        | WavPack                                        |
| audio/x-ape                                     | ape       | Monkey's Audio                                 |
| audio/x-flac                                    | flac      | Free Lossless Audio Codec                      |
| audio/x-it                                      | it        | Impulse Tracker Module                         |
| audio/x-m4a                                     | m4a       | Apple iTunes ALAC/AAC-LC Audio                 |
| audio/x-musepack                                | mpc       | Musepack                                       |
| font/otf                                        | otf       | OpenType                                       |
| font/ttf                                        | ttf       | TrueType                                       |
| font/woff                                       | woff      | Web Open Font Format                           |
| font/woff2                                      | woff2     | Web Open Font Format 2                         |
| image/apng                                      | apng      | Animated Portable Network Graphics             |
| image/avif                                      | avif      | AV1 Image File Format                          |
| image/bmp                                       | bmp       | Windows Bitmap                                 |
| image/bpg                                       | bpg       | Better Portable Graphics                       |
| image/cineon                                    | cin       | Cineon Image                                   |
| image/fits                                      | fits      | Flexible Image Transport System                |
| image/flif                                      | flif      | Free Lossless Image Format                     |
| image/gif                                       | gif       | Graphics Interchange Format                    |
| image/heic                                      | heic      | High Efficiency Image Coding                   |
| image/heic-sequence                             | heic      | High Efficiency Image Coding Sequence          |
| image/heif                                      | heic      | High Efficiency Image File Format              |
| image/heif-sequence                             | heic      | High Efficiency Image File Format Sequence     |
| image/icns                                      | icns      | Apple Icon Image                               |
| image/jp2                                       | jp2       | JPEG 2000 Part 1 (JP2)                         |
| image/jpeg                                      | jpg       | Joint Photographic Experts Group               |
| image/jpm                                       | jpm       | JPEG 2000 Part 6 (JPM)                         |
| image/jpx                                       | jpx       | JPEG 2000 Part 2 (JPX)                         |
| image/jxl                                       | jxl       | JPEG XL                                        |
| image/jxr                                       | jxr       | JPEG extended range                            |
| image/ktx                                       | ktx       | Khronos TeXture                                |
| image/ktx2                                      | ktx2      | Khronos TeXture 2                              |
| image/mj2                                       | mj2       | JPEG 2000 Part 3 (MJ2)                         |
| image/png                                       | png       | Portable Network Graphics                      |
| image/tiff                                      | tiff      | Tag Image File Format                          |
| image/vnd.adobe.photoshop                       | psd       | Adobe Photoshop document                       |
| image/webp                                      | webp      | WebP                                           |
| image/wmf                                       | wmf       | Windows Metafile                               |
| image/x-canon-cr3                               | cr3       | Canon Raw 3                                    |
| image/x-dpx                                     | dpx       | Digital Picture Exchange                       |
| image/x-exr                                     | exr       | OpenEXR                                        |
| image/x-icon                                    | cur       | CUR                                            |
| image/x-icon                                    | ico       | ICO                                            |
| image/x-olympus-orf                             | orf       | Olympus Raw Format                             |
| image/x-xcf                                     | xcf       | eXperimental Computing Facility                |
| model/gltf-binary                               | glb       | GL Transmission Format binary                  |
| video/3gpp                                      | 3gp       | 3rd Generation Partnership Project             |
| video/3gpp2                                     | 3g2       | 3rd Generation Partnership Project 2           |
| video/avi                                       | avi       | Audio Video Interleave                         |
| video/mp2t                                      | m2ts      | MPEG-2 Transport Stream                        |
| video/mp4                                       | mp4       | MPEG-4 Part 14                                 |
| video/mpeg                                      | mpg       | MPEG-1 video                                   |
| video/ogg                                       | ogm       | Ogg Media                                      |
| video/ogg                                       | ogv       | Ogg Theora                                     |
| video/quicktime                                 | mov       | QuickTime Movie                                |
| video/webm                                      | webm      | WebM                                           |
| video/x-flv                                     | flv       | Flash Video                                    |
| video/x-m4v                                     | m4v       | M4V                                            |
| video/x-matroska                                | mkv       | Matroska Multimedia Container                  |
| video/x-ms-asf                                  | wmv       | Windows Media Video                            |

## References

* [IANA media types](https://www.iana.org/assignments/media-types/media-types.xhtml)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
