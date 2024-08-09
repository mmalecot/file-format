mod compressed;
mod other;
mod metadata;
mod video;
mod model;
mod presentation;
mod database;
mod document;
mod image;
mod geospatial;
mod package;
mod formula;
mod audio;
mod archive;
mod playlist;
mod disk;
mod rom;
mod font;
mod ebook;
mod diagram;
mod subtitle;
mod spreadsheet;
mod executable;

#[cfg(feature = "extended-enums")]
use file_format::FileFormat;
#[cfg(feature = "extended-enums")]
use strum::IntoEnumIterator;

#[test]
#[cfg(all(feature = "extended-enums", feature = "from-extension"))]
fn all_types_supported_by_from_extension(){
    for format in FileFormat::iter() {
        let fmt = FileFormat::from_extension(format.extension());
        assert!(fmt.is_some_and(|types| types.contains(&format)), "{:?} does not contain {}", fmt, format);
    }
}

#[test]
#[cfg(all(feature = "extended-enums", feature = "from-extension"))]
fn all_types_supported_by_from_extension_with_dot(){
    for format in FileFormat::iter() {
        let fmt = FileFormat::from_extension(format!(".{}", format.extension()));
        assert!(fmt.is_some_and(|types| types.contains(&format)), "{:?} does not contain {}", fmt, format);
    }
}