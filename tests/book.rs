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
fn test_fictionbook_1() {
    let fmt = FileFormat::from_file("fixtures/book/sample1.fb2").unwrap();
    assert_eq!(fmt, FileFormat::Fictionbook);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_fictionbook_2() {
    let fmt = FileFormat::from_file("fixtures/book/sample2.fb2").unwrap();
    assert_eq!(fmt, FileFormat::Fictionbook);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_fictionbook_zipped() {
    let fmt = FileFormat::from_file("fixtures/book/sample.fbz").unwrap();
    assert_eq!(fmt, FileFormat::FictionbookZipped);
}

#[test]
fn test_microsoft_reader() {
    let fmt = FileFormat::from_file("fixtures/book/sample.lit").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftReader);
}

#[test]
fn test_mobipocket() {
    let fmt = FileFormat::from_file("fixtures/book/sample.mobi").unwrap();
    assert_eq!(fmt, FileFormat::Mobipocket);
}
