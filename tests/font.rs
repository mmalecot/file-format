use file_format::FileFormat;

#[test]
fn test_otf() {
    let format = FileFormat::from_file("fixtures/font/sample.otf").unwrap();
    assert_eq!(format, FileFormat::OpenType);
}

#[test]
fn test_ttf() {
    let format = FileFormat::from_file("fixtures/font/sample.ttf").unwrap();
    assert_eq!(format, FileFormat::TrueType);
}

#[test]
fn test_woff() {
    let format = FileFormat::from_file("fixtures/font/sample.woff").unwrap();
    assert_eq!(format, FileFormat::WebOpenFontFormat);
}

#[test]
fn test_woff2() {
    let format = FileFormat::from_file("fixtures/font/sample.woff2").unwrap();
    assert_eq!(format, FileFormat::WebOpenFontFormat2);
}
