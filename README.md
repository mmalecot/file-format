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
assert_eq!(format.preferred_extension(), Some("jpg"));
```

### Determine from bytes

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_bytes("Hello 😊!".as_bytes());
assert_eq!(format.kind(), Kind::Text);
assert_eq!(format.media_type(), "text/plain");
assert_eq!(format.preferred_extension(), Some("txt"));
```

### Determine from an extension

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_extension("wdp").unwrap();
assert_eq!(format.kind(), Kind::Image);
assert_eq!(format.media_type(), "image/jxr");
assert_eq!(format.preferred_extension(), Some("jxr"));
```

### Determine from a media type

```rust
use file_format::{FileFormat, Kind};

let format = FileFormat::from_media_type("image/tiff").unwrap();
assert_eq!(format.kind(), Kind::Image);
assert_eq!(format.media_type(), "image/tiff");
assert_eq!(format.preferred_extension(), Some("tiff"));
```

## Supported file formats

| Name                              | Media type                | Extensions    |
|-----------------------------------|---------------------------|---------------|
| Binary                            | application/octet-stream  | bin           |
| OpenType                          | font/otf                  | otf           |
| TrueType                          | font/ttf                  | ttf           |
| Web Open Font Format              | font/woff                 | woff          |
| Web Open Font Format 2            | font/woff2                | woff2         |
| Microsoft Windows bitmap          | image/bmp                 | bmp, dlib     |
| Better Portable Graphics          | image/bpg                 | bpg           |
| Free Lossless Image Format        | image/flif                | flif          |
| Graphics Interchange Format       | image/gif                 | gif           |
| High Efficiency Image File Format | image/heic                | heic          |
| Microsoft icon                    | image/x-icon              | ico           |
| JPEG-2000 JP2                     | image/jp2                 | jp2           |
| Joint Photographic Experts Group  | image/jpeg                | jpg, jpeg     |
| JPEG extended range               | image/jxr                 | jxr, hdp, wdp |
| Portable Network Graphics         | image/png                 | png           |
| Adobe Photoshop bitmap            | image/vnd.adobe.photoshop | psd           |
| Tagged Image File Format          | image/tiff                | tiff, tif     |
| Weppy image format                | image/webp                | webp          |
| 3GPP                              | video/3gpp                | 3gp           |
| Audio Video Interleave            | video/avi                 | avi           |
| Flash Video                       | video/x-flv               | flv           |
| M4V                               | video/x-m4v               | m4v           |
| Matroska Multimedia Container     | video/x-matroska          | mkv           |
| QuickTime File Format             | video/quicktime           | mov, qt       |
| MPEG-4 Part 14                    | video/mp4                 | mp4           |
| MPEG                              | video/mpeg                | mpg           |
| Ogg                               | video/ogv                 | ogv           |
| Weppy video format                | video/webp                | webp          |
| Windows Media Video               | x-ms-asf                  | wmv, wm       |
| UTF-8 text                        | text/plain                | txt           |

## References

* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [IANA media types](https://www.iana.org/assignments/media-types/media-types.xhtml)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
