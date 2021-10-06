# file-format

![Build](https://img.shields.io/github/workflow/status/mmalecot/file-format/CI)
[![Crates.io](https://img.shields.io/crates/v/file-format.svg)](https://crates.io/crates/file-format)
[![Docs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![Rust](https://img.shields.io/badge/rust-1.54+-blueviolet.svg?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

File format library for Rust.

This crate is for recognizing the format of a file by checking its signature.

## Examples

Determines from a file:

```rust
use file_format::FileFormat;

let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
assert_eq!(format, FileFormat::Jpeg);
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.preferred_extension(), "jpg");
```

Determines from bytes:

```rust
use file_format::FileFormat;

let format = FileFormat::from_bytes("Hello 😊!".as_bytes());
assert_eq!(format, FileFormat::Text);
assert_eq!(format.media_type(), "text/plain");
assert_eq!(format.preferred_extension(), "txt");
```

Determines from an extension:

```rust
use file_format::FileFormat;

let format = FileFormat::from_extension("vcf");
assert_eq!(format, Some(FileFormat::VCard));
```

Determines from a media type:

```rust
use file_format::FileFormat;

let format = FileFormat::from_media_type("image/vnd.adobe.photoshop");
assert_eq!(format, Some(FileFormat::PhotoshopDocument));
```

## References

* [IANA media types](https://www.iana.org/assignments/media-types/media-types.xhtml)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
