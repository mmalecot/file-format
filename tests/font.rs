use file_format::FileFormat;

#[test]
fn test_bitmap_font_ascii() {
    let format = FileFormat::from_file("fixtures/font/sample-ascii.fnt").unwrap();
    assert_eq!(format, FileFormat::BitmapFontAscii);
}

#[test]
fn test_bitmap_font_binary() {
    let format = FileFormat::from_file("fixtures/font/sample-binary.fnt").unwrap();
    assert_eq!(format, FileFormat::BitmapFontBinary);
}

#[test]
fn test_opentype() {
    let format = FileFormat::from_file("fixtures/font/sample.otf").unwrap();
    assert_eq!(format, FileFormat::Opentype);
}

#[test]
fn test_truetype() {
    let format = FileFormat::from_file("fixtures/font/sample.ttf").unwrap();
    assert_eq!(format, FileFormat::Truetype);
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
