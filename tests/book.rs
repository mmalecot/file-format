use file_format::FileFormat;

#[cfg(feature = "reader-zip")]
#[test]
fn test_electronic_publication() {
    let format = FileFormat::from_file("fixtures/book/sample.epub").unwrap();
    assert_eq!(format, FileFormat::ElectronicPublication);
}

#[test]
fn test_mobipocket() {
    let format = FileFormat::from_file("fixtures/book/sample.mobi").unwrap();
    assert_eq!(format, FileFormat::Mobipocket);
}
