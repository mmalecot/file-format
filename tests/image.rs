use file_format::FileFormat;

#[test]
fn test_apng() {
    let format = FileFormat::from_file("fixtures/image/sample.apng").unwrap();
    assert_eq!(format.media_type(), "image/apng");
    assert_eq!(format.extension(), "apng");
}

#[test]
fn test_avif() {
    let format = FileFormat::from_file("fixtures/image/sample.avif").unwrap();
    assert_eq!(format.media_type(), "image/avif");
    assert_eq!(format.extension(), "avif");
}

#[test]
fn test_bmp() {
    let format = FileFormat::from_file("fixtures/image/sample.bmp").unwrap();
    assert_eq!(format.media_type(), "image/bmp");
    assert_eq!(format.extension(), "bmp");
}

#[test]
fn test_bpg() {
    let format = FileFormat::from_file("fixtures/image/sample.bpg").unwrap();
    assert_eq!(format.media_type(), "image/bpg");
    assert_eq!(format.extension(), "bpg");
}

#[test]
fn test_cin() {
    let format = FileFormat::from_file("fixtures/image/sample.cin").unwrap();
    assert_eq!(format.media_type(), "image/cineon");
    assert_eq!(format.extension(), "cin");
}

#[test]
fn test_cr3() {
    let format = FileFormat::from_file("fixtures/image/sample.cr3").unwrap();
    assert_eq!(format.media_type(), "image/x-canon-cr3");
    assert_eq!(format.extension(), "cr3");
}

#[test]
fn test_dpx() {
    let format = FileFormat::from_file("fixtures/image/sample.dpx").unwrap();
    assert_eq!(format.media_type(), "image/x-dpx");
    assert_eq!(format.extension(), "dpx");
}

#[test]
fn test_exr() {
    let format = FileFormat::from_file("fixtures/image/sample.exr").unwrap();
    assert_eq!(format.media_type(), "image/x-exr");
    assert_eq!(format.extension(), "exr");
}

#[test]
fn test_fits() {
    let format = FileFormat::from_file("fixtures/image/sample.fits").unwrap();
    assert_eq!(format.media_type(), "image/fits");
    assert_eq!(format.extension(), "fits");
}

#[test]
fn test_flif() {
    let format = FileFormat::from_file("fixtures/image/sample.flif").unwrap();
    assert_eq!(format.media_type(), "image/flif");
    assert_eq!(format.extension(), "flif");
}

#[test]
fn test_gif() {
    let format = FileFormat::from_file("fixtures/image/sample.gif").unwrap();
    assert_eq!(format.media_type(), "image/gif");
    assert_eq!(format.extension(), "gif");
}

#[test]
fn test_heic() {
    let format = FileFormat::from_file("fixtures/image/sample.heic").unwrap();
    assert_eq!(format.media_type(), "image/heic");
    assert_eq!(format.extension(), "heic");
}

#[test]
fn test_icns() {
    let format = FileFormat::from_file("fixtures/image/sample.icns").unwrap();
    assert_eq!(format.media_type(), "image/icns");
    assert_eq!(format.extension(), "icns");
}

#[test]
fn test_ico() {
    let format = FileFormat::from_file("fixtures/image/sample.ico").unwrap();
    assert_eq!(format.media_type(), "image/x-icon");
    assert_eq!(format.extension(), "ico");
}

#[test]
fn test_jp2() {
    let format = FileFormat::from_file("fixtures/image/sample.jp2").unwrap();
    assert_eq!(format.media_type(), "image/jp2");
    assert_eq!(format.extension(), "jp2");
}

#[test]
fn test_jpg() {
    let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
    assert_eq!(format.media_type(), "image/jpeg");
    assert_eq!(format.extension(), "jpg");
}

#[test]
fn test_jpm() {
    let format = FileFormat::from_file("fixtures/image/sample.jpm").unwrap();
    assert_eq!(format.media_type(), "image/jpm");
    assert_eq!(format.extension(), "jpm");
}

#[test]
fn test_jpx() {
    let format = FileFormat::from_file("fixtures/image/sample.jpx").unwrap();
    assert_eq!(format.media_type(), "image/jpx");
    assert_eq!(format.extension(), "jpx");
}

#[test]
fn test_jxl() {
    let format = FileFormat::from_file("fixtures/image/sample.jxl").unwrap();
    assert_eq!(format.media_type(), "image/jxl");
    assert_eq!(format.extension(), "jxl");
}

#[test]
fn test_jxr() {
    let format = FileFormat::from_file("fixtures/image/sample.jxr").unwrap();
    assert_eq!(format.media_type(), "image/jxr");
    assert_eq!(format.extension(), "jxr");
}

#[test]
fn test_ktx() {
    let format = FileFormat::from_file("fixtures/image/sample.ktx").unwrap();
    assert_eq!(format.media_type(), "image/ktx");
    assert_eq!(format.extension(), "ktx");
}

#[test]
fn test_ktx2() {
    let format = FileFormat::from_file("fixtures/image/sample.ktx2").unwrap();
    assert_eq!(format.media_type(), "image/ktx2");
    assert_eq!(format.extension(), "ktx2");
}

#[test]
fn test_mj2() {
    let format = FileFormat::from_file("fixtures/image/sample.mj2").unwrap();
    assert_eq!(format.media_type(), "image/mj2");
    assert_eq!(format.extension(), "mj2");
}

#[test]
fn test_orf() {
    let format = FileFormat::from_file("fixtures/image/sample.orf").unwrap();
    assert_eq!(format.media_type(), "image/x-olympus-orf");
    assert_eq!(format.extension(), "orf");
}

#[test]
fn test_png() {
    let format = FileFormat::from_file("fixtures/image/sample.png").unwrap();
    assert_eq!(format.media_type(), "image/png");
    assert_eq!(format.extension(), "png");
}

#[test]
fn test_psd() {
    let format = FileFormat::from_file("fixtures/image/sample.psd").unwrap();
    assert_eq!(format.media_type(), "image/vnd.adobe.photoshop");
    assert_eq!(format.extension(), "psd");
}

#[test]
fn test_tiff() {
    let format = FileFormat::from_file("fixtures/image/sample.tiff").unwrap();
    assert_eq!(format.media_type(), "image/tiff");
    assert_eq!(format.extension(), "tiff");
}

#[test]
fn test_webp() {
    let format = FileFormat::from_file("fixtures/image/sample.webp").unwrap();
    assert_eq!(format.media_type(), "image/webp");
    assert_eq!(format.extension(), "webp");
}

#[test]
fn test_wmf() {
    let format = FileFormat::from_file("fixtures/image/sample.wmf").unwrap();
    assert_eq!(format.media_type(), "image/wmf");
    assert_eq!(format.extension(), "wmf");
}

#[test]
fn test_xcf() {
    let format = FileFormat::from_file("fixtures/image/sample.xcf").unwrap();
    assert_eq!(format.media_type(), "image/x-xcf");
    assert_eq!(format.extension(), "xcf");
}
