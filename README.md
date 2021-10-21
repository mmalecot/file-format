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

| Name                                           | Media type                                      | Extension |
|------------------------------------------------|-------------------------------------------------|-----------|
| Digital Imaging and Communications in Medicine | application/dicom                               | dcm       |
| Electronic Publication                         | application/epub+zip                            | epub      |
| gzip                                           | application/gzip                                | gz        |
| Java Class                                     | application/java-vm                             | class     |
| Material Exchange Format                       | application/mxf                                 | mxf       |
| Arbitrary Binary Data                          | application/octet-stream                        | bin       |
| Ogg Multiplexed Media                          | application/ogg                                 | ogx       |
| Portable Document Format                       | application/pdf                                 | pdf       |
| Dalvik Executable                              | application/vnd.android.dex                     | dex       |
| Debian Package                                 | application/vnd.debian.binary-package           | deb       |
| Cabinet                                        | application/vnd.ms-cab-compressed               | cab       |
| Embedded OpenType                              | application/vnd.ms-fontobject                   | eot       |
| Microsoft Compiled HTML Help                   | application/vnd.ms-htmlhelp                     | chm       |
| OpenDocument Graphics                          | application/vnd.oasis.opendocument.graphics     | odg       |
| OpenDocument Presentation                      | application/vnd.oasis.opendocument.presentation | odp       |
| OpenDocument SpreadSheet                       | application/vnd.oasis.opendocument.spreadsheet  | ods       |
| OpenDocument Text                              | application/vnd.oasis.opendocument.text         | odt       |
| Roshal Archive                                 | application/vnd.rar                             | rar       |
| SketchUp Document                              | application/vnd.sketchup.skp                    | skp       |
| SQLite 3 Database                              | application/vnd.sqlite3                         | sqlite    |
| libpcap                                        | application/vnd.tcpdump.pcap                    | pcap      |
| WebAssembly Binary                             | application/wasm                                | wasm      |
| 7z                                             | application/x-7z-compressed                     | 7z        |
| ALZip                                          | application/x-alz-compressed                    | alz       |
| Apache Arrow Columnar                          | application/x-apache-arrow                      | arrow     |
| macOS Alias                                    | application/x-apple-alias                       | alias     |
| Apple Disk Image                               | application/x-apple-diskimage                   | dmg       |
| UNIX archiver                                  | application/x-archive                           | ar        |
| Archived by Robert Jung                        | application/x-arj                               | arj       |
| Blender 3D Data                                | application/x-blender                           | blend     |
| bzip2                                          | application/x-bzip2                             | bz2       |
| Compound File Binary                           | application/x-cfb                               | cfb       |
| UNIX compress                                  | application/x-compress                          | Z         |
| cpio                                           | application/x-cpio                              | cpio      |
| Shapefile                                      | application/x-esri-shape                        | shp       |
| Executable and Linkable Format                 | application/x-executable                        | elf       |
| Game Boy Color ROM                             | application/x-gameboy-color-rom                 | gbc       |
| Game Boy ROM                                   | application/x-gameboy-rom                       | gb        |
| Game Boy Advance ROM                           | application/x-gba-rom                           | gba       |
| Google Chrome Extension                        | application/x-google-chrome-extension           | crx       |
| Adobe InDesign Document                        | application/x-indesign                          | indd      |
| ISO 9660 image                                 | application/x-iso9660-image                     | iso       |
| Long Range ZIP                                 | application/x-lrzip                             | lrz       |
| LZ4                                            | application/x-lz4                               | lz4       |
| Lempel–Ziv Finite State Entropy                | application/x-lzfse                             | lzfse     |
| LHA                                            | application/x-lzh-compressed                    | lzh       |
| lzip                                           | application/x-lzip                              | lz        |
| lzop                                           | application/x-lzop                              | lzo       |
| Mobipocket                                     | application/x-mobipocket-ebook                  | mobi      |
| Windows Shortcut                               | application/x-ms-shortcut                       | lnk       |
| Windows Executable                             | application/x-msdownload                        | exe       |
| Nintendo 64 ROM                                | application/x-n64-rom                           | z64       |
| ANI                                            | application/x-navi-animation                    | ani       |
| Nintendo DS ROM                                | application/x-nintendo-ds-rom                   | nds       |
| Nintendo Entertainment System ROM              | application/x-nintendo-nes-rom                  | nes       |
| PCAP Next Generation Dump                      | application/x-pcapng                            | pcapng    |
| Red Hat Package Manager Package                | application/x-rpm                               | rpm       |
| SeqBox                                         | application/x-sbx                               | sbx       |
| Small Web Format                               | application/x-shockwave-flash                   | swf       |
| Snappy                                         | application/x-snappy-framed                     | sz        |
| Tape Archive                                   | application/x-tar                               | tar       |
| Microsoft Virtual Hard Disk                    | application/x-vhd                               | vhd       |
| Microsoft Virtual Hard Disk 2                  | application/x-vhdx                              | vhdx      |
| VirtualBox Virtual Disk Image                  | application/x-virtualbox-vdi                    | vdi       |
| Extensible Archive                             | application/x-xar                               | xar       |
| XZ                                             | application/x-xz                                | xz        |
| zoo                                            | application/x-zoo                               | zoo       |
| ZIP                                            | application/zip                                 | zip       |
| Zstandard                                      | application/zstd                                | zst       |
| Advanced Audio Coding                          | audio/aac                                       | aac       |
| Audio Interchange File Format                  | audio/aiff                                      | aif       |
| Adaptive Multi-Rate                            | audio/amr                                       | amr       |
| Au                                             | audio/basic                                     | au        |
| Musical Instrument Digital Interface           | audio/midi                                      | mid       |
| Adobe Flash Player Audio                       | audio/mp4                                       | f4a       |
| Adobe Flash Player Audiobook                   | audio/mp4                                       | f4b       |
| Apple iTunes ALAC/AAC-LC Audiobook             | audio/mp4                                       | m4b       |
| Apple iTunes ALAC/AAC-LC Protected Audio       | audio/mp4                                       | m4p       |
| MPEG-1/2 Audio Layer III                       | audio/mpeg                                      | mp3       |
| Ogg FLAC                                       | audio/ogg                                       | oga       |
| Ogg Vorbis                                     | audio/ogg                                       | ogg       |
| Ogg Speex                                      | audio/ogg                                       | spx       |
| Ogg Opus                                       | audio/opus                                      | opus      |
| Qualcomm PureVoice                             | audio/qcelp                                     | qcp       |
| Audio Codec 3                                  | audio/vnd.dolby.dd-raw                          | ac3       |
| Waveform Audio                                 | audio/vnd.wave                                  | wav       |
| WavPack                                        | audio/wavpack                                   | wv        |
| Monkey's Audio                                 | audio/x-ape                                     | ape       |
| Sony DSD Stream File                           | audio/x-dsf                                     | dsf       |
| Free Lossless Audio Codec                      | audio/x-flac                                    | flac      |
| Impulse Tracker Module                         | audio/x-it                                      | it        |
| Apple iTunes ALAC/AAC-LC Audio                 | audio/x-m4a                                     | m4a       |
| Musepack                                       | audio/x-musepack                                | mpc       |
| ScreamTracker 3 Module                         | audio/x-s3m                                     | s3m       |
| FastTracker 2 Module                           | audio/x-xm                                      | xm        |
| OpenType                                       | font/otf                                        | otf       |
| TrueType                                       | font/ttf                                        | ttf       |
| Web Open Font Format                           | font/woff                                       | woff      |
| Web Open Font Format 2                         | font/woff2                                      | woff2     |
| Animated Portable Network Graphics             | image/apng                                      | apng      |
| AV1 Image File Format                          | image/avif                                      | avif      |
| Windows Bitmap                                 | image/bmp                                       | bmp       |
| Better Portable Graphics                       | image/bpg                                       | bpg       |
| Cineon Image                                   | image/cineon                                    | cin       |
| Flexible Image Transport System                | image/fits                                      | fits      |
| Free Lossless Image Format                     | image/flif                                      | flif      |
| Graphics Interchange Format                    | image/gif                                       | gif       |
| High Efficiency Image Coding                   | image/heic                                      | heic      |
| High Efficiency Image Coding Sequence          | image/heic-sequence                             | heic      |
| High Efficiency Image File Format              | image/heif                                      | heic      |
| High Efficiency Image File Format Sequence     | image/heif-sequence                             | heic      |
| Apple Icon Image                               | image/icns                                      | icns      |
| JPEG 2000 Part 1 (JP2)                         | image/jp2                                       | jp2       |
| Joint Photographic Experts Group               | image/jpeg                                      | jpg       |
| JPEG 2000 Part 6 (JPM)                         | image/jpm                                       | jpm       |
| JPEG 2000 Part 2 (JPX)                         | image/jpx                                       | jpx       |
| JPEG XL                                        | image/jxl                                       | jxl       |
| JPEG Extended Range                            | image/jxr                                       | jxr       |
| Khronos Texture                                | image/ktx                                       | ktx       |
| Khronos Texture 2                              | image/ktx2                                      | ktx2      |
| JPEG 2000 Part 3 (MJ2)                         | image/mj2                                       | mj2       |
| Portable Network Graphics                      | image/png                                       | png       |
| Tag Image File Format                          | image/tiff                                      | tiff      |
| DirectDraw Surface                             | image/vnd-ms.dds                                | dds       |
| Adobe Photoshop document                       | image/vnd.adobe.photoshop                       | psd       |
| Radiance HDR                                   | image/vnd.radiance                              | hdr       |
| WebP                                           | image/webp                                      | webp      |
| Windows Metafile                               | image/wmf                                       | wmf       |
| Canon Raw 2                                    | image/x-canon-cr2                               | cr2       |
| Canon Raw 3                                    | image/x-canon-cr3                               | cr3       |
| Digital Picture Exchange                       | image/x-dpx                                     | dpx       |
| OpenEXR                                        | image/x-exr                                     | exr       |
| Fujifilm Raw                                   | image/x-fujifilm-raf                            | raf       |
| CUR                                            | image/x-icon                                    | cur       |
| ICO                                            | image/x-icon                                    | ico       |
| Nikon Electronic File                          | image/x-nikon-nef                               | nef       |
| Olympus Raw Format                             | image/x-olympus-orf                             | orf       |
| Panasonic Raw                                  | image/x-panasonic-rw2                           | rw2       |
| Experimental Computing Facility                | image/x-xcf                                     | xcf       |
| GL Transmission Format Binary                  | model/gltf-binary                               | glb       |
| 3rd Generation Partnership Project             | video/3gpp                                      | 3gp       |
| 3rd Generation Partnership Project 2           | video/3gpp2                                     | 3g2       |
| Audio Video Interleave                         | video/avi                                       | avi       |
| MPEG-2 Transport Stream                        | video/mp2t                                      | m2ts      |
| Adobe Flash Player Protected Video             | video/mp4                                       | f4p       |
| Adobe Flash Player Video                       | video/mp4                                       | f4v       |
| MPEG-4 Part 14                                 | video/mp4                                       | mp4       |
| MPEG-1 video                                   | video/mpeg                                      | mpg       |
| Ogg Media                                      | video/ogg                                       | ogm       |
| Ogg Theora                                     | video/ogg                                       | ogv       |
| QuickTime Movie                                | video/quicktime                                 | mov       |
| WebM                                           | video/webm                                      | webm      |
| Flash Video                                    | video/x-flv                                     | flv       |
| M4V                                            | video/x-m4v                                     | m4v       |
| Matroska Multimedia Container                  | video/x-matroska                                | mkv       |
| Windows Media Video                            | video/x-ms-asf                                  | wmv       |

## References

* [IANA media types](https://www.iana.org/assignments/media-types/media-types.xhtml)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
