use file_format::FileFormat;

#[test]
fn test_broad_band_ebook() {
    let fmt = FileFormat::from_file("fixtures/book/sample.lrf").unwrap();
    assert_eq!(fmt, FileFormat::BroadBandEbook);
}

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
