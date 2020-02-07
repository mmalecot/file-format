use file_format::{FileFormat, Kind};

#[test]
fn test_otf() {
    let format = FileFormat::from_file("fixtures/font/sample.otf").unwrap();
    assert_eq!(format.kind(), Kind::Font);
    assert_eq!(format.media_type(), "font/otf");
    assert_eq!(format.preferred_extension(), Some("otf"));
}

#[test]
fn test_ttf() {
    let format = FileFormat::from_file("fixtures/font/sample.ttf").unwrap();
    assert_eq!(format.kind(), Kind::Font);
    assert_eq!(format.media_type(), "font/ttf");
    assert_eq!(format.preferred_extension(), Some("ttf"));
}

#[test]
fn test_woff() {
    let format = FileFormat::from_file("fixtures/font/sample.woff").unwrap();
    assert_eq!(format.kind(), Kind::Font);
    assert_eq!(format.media_type(), "font/woff");
    assert_eq!(format.preferred_extension(), Some("woff"));
}

#[test]
fn test_woff2() {
    let format = FileFormat::from_file("fixtures/font/sample.woff2").unwrap();
    assert_eq!(format.kind(), Kind::Font);
    assert_eq!(format.media_type(), "font/woff2");
    assert_eq!(format.preferred_extension(), Some("woff2"));
}
