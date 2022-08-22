use file_format::FileFormat;

#[test]
fn test_open_type() {
    let format = FileFormat::from_file("fixtures/font/sample.otf").unwrap();
    assert_eq!(format, FileFormat::OpenType);
}

#[test]
fn test_true_type() {
    let format = FileFormat::from_file("fixtures/font/sample.ttf").unwrap();
    assert_eq!(format, FileFormat::TrueType);
}

#[test]
fn test_web_open_font_format() {
    let format = FileFormat::from_file("fixtures/font/sample.woff").unwrap();
    assert_eq!(format, FileFormat::WebOpenFontFormat);
}

#[test]
fn test_web_open_font_format2() {
    let format = FileFormat::from_file("fixtures/font/sample.woff2").unwrap();
    assert_eq!(format, FileFormat::WebOpenFontFormat2);
}
