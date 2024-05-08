use file_format::FileFormat;

#[test]
fn test_bmfont_ascii() {
    let fmt = FileFormat::from_file("fixtures/font/sample1.fnt").unwrap();
    assert_eq!(fmt, FileFormat::BmfontAscii);
}

#[test]
fn test_bmfont_binary() {
    let fmt = FileFormat::from_file("fixtures/font/sample2.fnt").unwrap();
    assert_eq!(fmt, FileFormat::BmfontBinary);
}

#[test]
fn test_embedded_opentype() {
    let fmt = FileFormat::from_file("fixtures/font/sample.eot").unwrap();
    assert_eq!(fmt, FileFormat::EmbeddedOpentype);
}

#[test]
fn test_glyphs() {
    let fmt = FileFormat::from_file("fixtures/font/sample.glyphs").unwrap();
    assert_eq!(fmt, FileFormat::Glyphs);
}

#[test]
fn test_opentype() {
    let fmt = FileFormat::from_file("fixtures/font/sample.otf").unwrap();
    assert_eq!(fmt, FileFormat::Opentype);
}

#[test]
fn test_truetype() {
    let fmt = FileFormat::from_file("fixtures/font/sample.ttf").unwrap();
    assert_eq!(fmt, FileFormat::Truetype);
}

#[test]
fn test_web_open_font_format() {
    let fmt = FileFormat::from_file("fixtures/font/sample.woff").unwrap();
    assert_eq!(fmt, FileFormat::WebOpenFontFormat);
}

#[test]
fn test_web_open_font_format2() {
    let fmt = FileFormat::from_file("fixtures/font/sample.woff2").unwrap();
    assert_eq!(fmt, FileFormat::WebOpenFontFormat2);
}
