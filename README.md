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
use file_format::FileFormat;

let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
assert_eq!(format.media_type(), "image/jpeg");
assert_eq!(format.preferred_extension(), "jpg");
```

### Determine from bytes

```rust
use file_format::FileFormat;

let format = FileFormat::from_bytes("Hello 😊!".as_bytes());
assert_eq!(format.media_type(), "text/plain");
assert_eq!(format.preferred_extension(), "txt");
```

### Determine from an extension

```rust
use file_format::FileFormat;

let format = FileFormat::from_extension("wdp").unwrap();
assert_eq!(format.media_type(), "image/jxr");
assert_eq!(format.preferred_extension(), "jxr");
```

### Determine from a media type

```rust
use file_format::FileFormat;

let format = FileFormat::from_media_type("image/tiff").unwrap();
assert_eq!(format.media_type(), "image/tiff");
assert_eq!(format.preferred_extension(), "tiff");
```

## References

* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [IANA media types](https://www.iana.org/assignments/media-types/media-types.xhtml)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
