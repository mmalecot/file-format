use file_format::{FileFormat, Kind};

#[test]
fn test_bmp() {
    let format = FileFormat::from_file("fixtures/image/sample.bmp").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/bmp");
    assert_eq!(format.preferred_extension(), "bmp");
}

#[test]
fn test_bpg() {
    let format = FileFormat::from_file("fixtures/image/sample.bpg").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/bpg");
    assert_eq!(format.preferred_extension(), "bpg");
}

#[test]
fn test_flif() {
    let format = FileFormat::from_file("fixtures/image/sample.flif").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/flif");
    assert_eq!(format.preferred_extension(), "flif");
}

#[test]
fn test_gif() {
    let format = FileFormat::from_file("fixtures/image/sample.gif").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/gif");
    assert_eq!(format.preferred_extension(), "gif");
}

#[test]
fn test_heic() {
    let format = FileFormat::from_file("fixtures/image/sample.heic").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/heic");
    assert_eq!(format.preferred_extension(), "heic");
}

#[test]
fn test_ico() {
    let format = FileFormat::from_file("fixtures/image/sample.ico").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/x-icon");
    assert_eq!(format.preferred_extension(), "ico");
}

#[test]
fn test_jp2() {
    let format = FileFormat::from_file("fixtures/image/sample.jp2").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/jp2");
    assert_eq!(format.preferred_extension(), "jp2");
}

#[test]
fn test_jpg() {
    let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/jpeg");
    assert_eq!(format.preferred_extension(), "jpg");
}

#[test]
fn test_jxr() {
    let format = FileFormat::from_file("fixtures/image/sample.jxr").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/jxr");
    assert_eq!(format.preferred_extension(), "jxr");
}

#[test]
fn test_png() {
    let format = FileFormat::from_file("fixtures/image/sample.png").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/png");
    assert_eq!(format.preferred_extension(), "png");
}

#[test]
fn test_psd() {
    let format = FileFormat::from_file("fixtures/image/sample.psd").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/vnd.adobe.photoshop");
    assert_eq!(format.preferred_extension(), "psd");
}

#[test]
fn test_tiff() {
    let format = FileFormat::from_file("fixtures/image/sample.tiff").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/tiff");
    assert_eq!(format.preferred_extension(), "tiff");
}

#[test]
fn test_webp() {
    let format = FileFormat::from_file("fixtures/image/sample.webp").unwrap();
    assert_eq!(format.kind(), Kind::Image);
    assert_eq!(format.media_type(), "image/webp");
    assert_eq!(format.preferred_extension(), "webp");
}
