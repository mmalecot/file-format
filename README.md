# file-format

![Build](https://img.shields.io/github/workflow/status/mmalecot/file-format/CI)
[![Crates.io](https://img.shields.io/crates/v/file-format.svg)](https://crates.io/crates/file-format)
[![Docs](https://docs.rs/file-format/badge.svg)](https://docs.rs/file-format)
![Rust](https://img.shields.io/badge/rust-1.56+-blueviolet.svg?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

File format library for Rust.

This crate is for determining binary-based file formats.

## Example

Determines from a file:

```rust
use file_format::FileFormat;

let format = FileFormat::from_file("fixtures/application/sample.zip").unwrap();
assert_eq!(format, FileFormat::Zip);
assert_eq!(format.name(), "ZIP");
assert_eq!(format.media_type(), "application/zip");
assert_eq!(format.extension(), "zip");
```

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
