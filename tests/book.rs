use file_format::FileFormat;

#[cfg(feature = "reader-zip")]
#[test]
fn test_electronic_publication() {
    let fmt = FileFormat::from_file("fixtures/book/sample.epub").unwrap();
    assert_eq!(fmt, FileFormat::ElectronicPublication);
}

#[test]
fn test_mobipocket() {
    let fmt = FileFormat::from_file("fixtures/book/sample.mobi").unwrap();
    assert_eq!(fmt, FileFormat::Mobipocket);
}
