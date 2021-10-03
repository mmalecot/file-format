# file-format

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mmalecot/file-format/CI)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.54+-blueviolet.svg?logo=rust)

File format library for Rust.

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

let format = FileFormat::from_extension("sqlite3");
assert_eq!(format, Some(FileFormat::Sqlite3));
```

Determines from a media type:

```rust
use file_format::FileFormat;

let format = FileFormat::from_media_type("image/tiff");
assert_eq!(format, Some(FileFormat::Tiff));
```

## References

* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [IANA media types](https://www.iana.org/assignments/media-types/media-types.xhtml)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
