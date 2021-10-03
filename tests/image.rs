use file_format::FileFormat;

#[test]
fn test_avif() {
    let format = FileFormat::from_file("fixtures/image/sample.avif").unwrap();
    assert_eq!(format, FileFormat::Avif);
    assert_eq!(format.media_type(), "image/avif");
    assert_eq!(format.preferred_extension(), "avif");
}

#[test]
fn test_bmp() {
    let format = FileFormat::from_file("fixtures/image/sample.bmp").unwrap();
    assert_eq!(format, FileFormat::Bmp);
    assert_eq!(format.media_type(), "image/bmp");
    assert_eq!(format.preferred_extension(), "bmp");
}

#[test]
fn test_bpg() {
    let format = FileFormat::from_file("fixtures/image/sample.bpg").unwrap();
    assert_eq!(format, FileFormat::Bpg);
    assert_eq!(format.media_type(), "image/bpg");
    assert_eq!(format.preferred_extension(), "bpg");
}

#[test]
fn test_cin() {
    let format = FileFormat::from_file("fixtures/image/sample.cin").unwrap();
    assert_eq!(format, FileFormat::Cineon);
    assert_eq!(format.media_type(), "image/cineon");
    assert_eq!(format.preferred_extension(), "cin");
}

#[test]
fn test_dpx() {
    let format = FileFormat::from_file("fixtures/image/sample.dpx").unwrap();
    assert_eq!(format, FileFormat::Dpx);
    assert_eq!(format.media_type(), "image/x-dpx");
    assert_eq!(format.preferred_extension(), "dpx");
}

#[test]
fn test_exr() {
    let format = FileFormat::from_file("fixtures/image/sample.exr").unwrap();
    assert_eq!(format, FileFormat::OpenExr);
    assert_eq!(format.media_type(), "image/x-exr");
    assert_eq!(format.preferred_extension(), "exr");
}

#[test]
fn test_fits() {
    let format = FileFormat::from_file("fixtures/image/sample.fits").unwrap();
    assert_eq!(format, FileFormat::Fits);
    assert_eq!(format.media_type(), "image/fits");
    assert_eq!(format.preferred_extension(), "fits");
}

#[test]
fn test_flif() {
    let format = FileFormat::from_file("fixtures/image/sample.flif").unwrap();
    assert_eq!(format, FileFormat::Flif);
    assert_eq!(format.media_type(), "image/flif");
    assert_eq!(format.preferred_extension(), "flif");
}

#[test]
fn test_gif() {
    let format = FileFormat::from_file("fixtures/image/sample.gif").unwrap();
    assert_eq!(format, FileFormat::Gif);
    assert_eq!(format.media_type(), "image/gif");
    assert_eq!(format.preferred_extension(), "gif");
}

#[test]
fn test_heic() {
    let format = FileFormat::from_file("fixtures/image/sample.heic").unwrap();
    assert_eq!(format, FileFormat::Heif);
    assert_eq!(format.media_type(), "image/heic");
    assert_eq!(format.preferred_extension(), "heic");
}

#[test]
fn test_icns() {
    let format = FileFormat::from_file("fixtures/image/sample.icns").unwrap();
    assert_eq!(format, FileFormat::AppleIconImage);
    assert_eq!(format.media_type(), "image/icns");
    assert_eq!(format.preferred_extension(), "icns");
}

#[test]
fn test_ico() {
    let format = FileFormat::from_file("fixtures/image/sample.ico").unwrap();
    assert_eq!(format, FileFormat::Ico);
    assert_eq!(format.media_type(), "image/x-icon");
    assert_eq!(format.preferred_extension(), "ico");
}

#[test]
fn test_jp2() {
    let format = FileFormat::from_file("fixtures/image/sample.jp2").unwrap();
    assert_eq!(format, FileFormat::Jp2);
    assert_eq!(format.media_type(), "image/jp2");
    assert_eq!(format.preferred_extension(), "jp2");
}

#[test]
fn test_jpg() {
    let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
    assert_eq!(format, FileFormat::Jpeg);
    assert_eq!(format.media_type(), "image/jpeg");
    assert_eq!(format.preferred_extension(), "jpg");
}

#[test]
fn test_jxl() {
    let format = FileFormat::from_file("fixtures/image/sample.jxl").unwrap();
    assert_eq!(format, FileFormat::JpegXl);
    assert_eq!(format.media_type(), "image/jxl");
    assert_eq!(format.preferred_extension(), "jxl");
}

#[test]
fn test_jxr() {
    let format = FileFormat::from_file("fixtures/image/sample.jxr").unwrap();
    assert_eq!(format, FileFormat::JpegXr);
    assert_eq!(format.media_type(), "image/jxr");
    assert_eq!(format.preferred_extension(), "jxr");
}

#[test]
fn test_png() {
    let format = FileFormat::from_file("fixtures/image/sample.png").unwrap();
    assert_eq!(format, FileFormat::Png);
    assert_eq!(format.media_type(), "image/png");
    assert_eq!(format.preferred_extension(), "png");
}

#[test]
fn test_psd() {
    let format = FileFormat::from_file("fixtures/image/sample.psd").unwrap();
    assert_eq!(format, FileFormat::PhotoshopDocument);
    assert_eq!(format.media_type(), "image/vnd.adobe.photoshop");
    assert_eq!(format.preferred_extension(), "psd");
}

#[test]
fn test_tiff() {
    let format = FileFormat::from_file("fixtures/image/sample.tiff").unwrap();
    assert_eq!(format, FileFormat::Tiff);
    assert_eq!(format.media_type(), "image/tiff");
    assert_eq!(format.preferred_extension(), "tiff");
}

#[test]
fn test_webp() {
    let format = FileFormat::from_file("fixtures/image/sample.webp").unwrap();
    assert_eq!(format, FileFormat::WebP);
    assert_eq!(format.media_type(), "image/webp");
    assert_eq!(format.preferred_extension(), "webp");
}

#[test]
fn test_xcf() {
    let format = FileFormat::from_file("fixtures/image/sample.xcf").unwrap();
    assert_eq!(format, FileFormat::Xcf);
    assert_eq!(format.media_type(), "image/x-xcf");
    assert_eq!(format.preferred_extension(), "xcf");
}
