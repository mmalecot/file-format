# file-format

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mmalecot/file-format/CI)
[![Crates.io](https://img.shields.io/crates/v/file-format)](https://crates.io/crates/file-format)
[![Docs.rs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.41+-blueviolet.svg?logo=rust)

File format library for Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
file_format = "0.1"
```

## Examples

### Determine from a file

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
assert_eq!(format.kind(), Kind::Image);
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.preferred_extension(), "jpg");
```

### Determine from bytes

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_bytes("Hello 😊!".as_bytes());
assert_eq!(format.kind(), Kind::Text);
assert_eq!(format.media_type(), "text/plain");
assert_eq!(format.preferred_extension(), "txt");
```

### Determine from an extension

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_extension("wdp").unwrap();
assert_eq!(format.kind(), Kind::Image);
assert_eq!(format.media_type(), "image/jxr");
assert_eq!(format.preferred_extension(), "jxr");
```

### Determine from a media type

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_media_type("image/tiff").unwrap();
assert_eq!(format.kind(), Kind::Image);
assert_eq!(format.media_type(), "image/tiff");
assert_eq!(format.preferred_extension(), "tiff");
```

## Supported file formats

| Name                              | Media type                   | Extensions          |
|-----------------------------------|------------------------------|---------------------|
| 7z                                | application/x-7z-compressed  | bin                 |
| Binary                            | application/octet-stream     | bin                 |
| Bzip2                             | application/x-bzip2          | bz2                 |
| Debian package                    | application/x-deb            | deb                 |
| Executable and Linkable Format    | application/x-executable     | elf, so             |
| EXE                               | application/x-msdownload     | exe, dll            |
| GNU zip                           | application/gzip             | gz                  |
| Long Range ZIP                    | application/x-lrzip          | lrz                 |
| Lzip                              | application/x-lzip           | lz                  |
| LZ4                               | application/x-lz4            | lz4                 |
| Lempel-Ziv-Oberhumer              | application/x-lzop           | lzo                 |
| Packet capture                    | application/vnd.tcpdump.pcap | pcap, cap, dmp      |
| Packet capture next generation    | application/x-pcapng         | pcapng              |
| Portable Document Format          | application/pdf              | pdf                 |
| Roshal Archive                    | application/vnd.rar          | rar                 |
| RedHat package                    | application/x-rpm            | rpm                 |
| SQLite                            | application/vnd.sqlite3      | sqlite, sqlite3, db |
| Tape archiver                     | application/x-tar            | tar                 |
| XZ                                | application/x-xz             | xz                  |
| Compress                          | application/x-compress       | Z                   |
| ZIP                               | application/zip              | zip                 |
| Zstandard                         | application/zstd             | zst                 |
| Advanced Audio Coding             | audio/aac                    | aac                 |
| Audio Interchange File Format     | audio/aiff                   | aif, aiff           |
| Au                                | audio/basic                  | au, snd             |
| Free Lossless Audio Codec         | audio/x-flac                 | flac                |
| M4V                               | audio/x-m4a                  | m4a                 |
| MPEG-1 Audio Layer III            | audio/mpeg                   | mp3                 |
| Ogg                               | audio/ogg                    | ogg, oga, spx       |
| Waveform Audio File Format        | audio/vnd.wave               | wav                 |
| WavPack                           | audio/wavpack                | wv                  |
| OpenType                          | font/otf                     | otf                 |
| TrueType                          | font/ttf                     | ttf                 |
| Web Open Font Format              | font/woff                    | woff                |
| Web Open Font Format 2            | font/woff2                   | woff2               |
| Microsoft Windows bitmap          | image/bmp                    | bmp, dlib           |
| Better Portable Graphics          | image/bpg                    | bpg                 |
| Free Lossless Image Format        | image/flif                   | flif                |
| Graphics Interchange Format       | image/gif                    | gif                 |
| High Efficiency Image File Format | image/heic                   | heic                |
| Microsoft icon                    | image/x-icon                 | ico                 |
| JPEG-2000 JP2                     | image/jp2                    | jp2                 |
| Joint Photographic Experts Group  | image/jpeg                   | jpg, jpeg           |
| JPEG extended range               | image/jxr                    | jxr, hdp, wdp       |
| Portable Network Graphics         | image/png                    | png                 |
| Adobe Photoshop bitmap            | image/vnd.adobe.photoshop    | psd                 |
| Tagged Image File Format          | image/tiff                   | tiff, tif           |
| Weppy image format                | image/webp                   | webp                |
| GL Transmission Format binary     | model/gltf-binary            | glb                 |
| 3GPP                              | video/3gpp                   | 3gp                 |
| Audio Video Interleave            | video/avi                    | avi                 |
| Flash Video                       | video/x-flv                  | flv                 |
| M4V                               | video/x-m4v                  | m4v                 |
| Matroska Multimedia Container     | video/x-matroska             | mkv                 |
| QuickTime File Format             | video/quicktime              | mov, qt             |
| MPEG                              | video/mpeg                   | mpg                 |
| MPEG-4 Part 14                    | video/mp4                    | mp4                 |
| Ogg                               | video/ogv                    | ogv                 |
| Weppy video format                | video/webp                   | webp                |
| Windows Media Video               | video/x-ms-asf               | wmv, wm             |
| UTF-8 text                        | text/plain                   | txt                 |

## References

* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [IANA media types](https://www.iana.org/assignments/media-types/media-types.xhtml)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
