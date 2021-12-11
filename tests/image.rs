use file_format::FileFormat;

#[test]
fn test_apng() {
    let format = FileFormat::from_file("fixtures/image/sample.apng").unwrap();
    assert_eq!(format, FileFormat::AnimatedPortableNetworkGraphics);
}

#[test]
fn test_avif() {
    let format = FileFormat::from_file("fixtures/image/sample.avif").unwrap();
    assert_eq!(format, FileFormat::Av1ImageFileFormat);
}

#[test]
fn test_avifs() {
    let format = FileFormat::from_file("fixtures/image/sample.avifs").unwrap();
    assert_eq!(format, FileFormat::Av1ImageFileFormatSequence);
}

#[test]
fn test_bmp() {
    let format = FileFormat::from_file("fixtures/image/sample.bmp").unwrap();
    assert_eq!(format, FileFormat::WindowsBitmap);
}

#[test]
fn test_bpg() {
    let format = FileFormat::from_file("fixtures/image/sample.bpg").unwrap();
    assert_eq!(format, FileFormat::BetterPortableGraphics);
}

#[test]
fn test_cin() {
    let format = FileFormat::from_file("fixtures/image/sample.cin").unwrap();
    assert_eq!(format, FileFormat::Cineon);
}

#[test]
fn test_cr2() {
    let format = FileFormat::from_file("fixtures/image/sample.cr2").unwrap();
    assert_eq!(format, FileFormat::CanonRaw2);
}

#[test]
fn test_cr3() {
    let format = FileFormat::from_file("fixtures/image/sample.cr3").unwrap();
    assert_eq!(format, FileFormat::CanonRaw3);
}

#[test]
fn test_cur() {
    let format = FileFormat::from_file("fixtures/image/sample.cur").unwrap();
    assert_eq!(format, FileFormat::Cur);
}

#[test]
fn test_dds() {
    let format = FileFormat::from_file("fixtures/image/sample.dds").unwrap();
    assert_eq!(format, FileFormat::MicrosoftDirectDrawSurface);
}

#[test]
fn test_dpx() {
    let format = FileFormat::from_file("fixtures/image/sample.dpx").unwrap();
    assert_eq!(format, FileFormat::DigitalPictureExchange);
}

#[test]
fn test_exr() {
    let format = FileFormat::from_file("fixtures/image/sample.exr").unwrap();
    assert_eq!(format, FileFormat::OpenExr);
}

#[test]
fn test_fits() {
    let format = FileFormat::from_file("fixtures/image/sample.fits").unwrap();
    assert_eq!(format, FileFormat::FlexibleImageTransportSystem);
}

#[test]
fn test_flif() {
    let format = FileFormat::from_file("fixtures/image/sample.flif").unwrap();
    assert_eq!(format, FileFormat::FreeLosslessImageFormat);
}

#[test]
fn test_gif() {
    let format = FileFormat::from_file("fixtures/image/sample.gif").unwrap();
    assert_eq!(format, FileFormat::GraphicsInterchangeFormat);
}

#[test]
fn test_hdr() {
    let format = FileFormat::from_file("fixtures/image/sample.hdr").unwrap();
    assert_eq!(format, FileFormat::RadianceHdr);
}

#[test]
fn test_heic() {
    let format = FileFormat::from_file("fixtures/image/sample.heic").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageCoding);
}

#[test]
fn test_heics() {
    let format = FileFormat::from_file("fixtures/image/sample.heics").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageCodingSequence);
}
#[test]
fn test_heif() {
    let format = FileFormat::from_file("fixtures/image/sample.heif").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageFileFormat);
}

#[test]
fn test_heifs() {
    let format = FileFormat::from_file("fixtures/image/sample.heifs").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageFileFormatSequence);
}

#[test]
fn test_icns() {
    let format = FileFormat::from_file("fixtures/image/sample.icns").unwrap();
    assert_eq!(format, FileFormat::AppleIconImage);
}

#[test]
fn test_ico() {
    let format = FileFormat::from_file("fixtures/image/sample.ico").unwrap();
    assert_eq!(format, FileFormat::Ico);
}

#[test]
fn test_jp2() {
    let format = FileFormat::from_file("fixtures/image/sample.jp2").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part1);
}

#[test]
fn test_jpg() {
    let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
    assert_eq!(format, FileFormat::JointPhotographicExpertsGroup);
}

#[test]
fn test_jpm() {
    let format = FileFormat::from_file("fixtures/image/sample.jpm").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part6);
}

#[test]
fn test_jpx() {
    let format = FileFormat::from_file("fixtures/image/sample.jpx").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part2);
}

#[test]
fn test_jxl() {
    let format = FileFormat::from_file("fixtures/image/sample.jxl").unwrap();
    assert_eq!(format, FileFormat::JpegXl);
}

#[test]
fn test_jxr() {
    let format = FileFormat::from_file("fixtures/image/sample.jxr").unwrap();
    assert_eq!(format, FileFormat::JpegExtendedRange);
}

#[test]
fn test_ktx() {
    let format = FileFormat::from_file("fixtures/image/sample.ktx").unwrap();
    assert_eq!(format, FileFormat::KhronosTexture);
}

#[test]
fn test_ktx2() {
    let format = FileFormat::from_file("fixtures/image/sample.ktx2").unwrap();
    assert_eq!(format, FileFormat::KhronosTexture2);
}

#[test]
fn test_mj2() {
    let format = FileFormat::from_file("fixtures/image/sample.mj2").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part3);
}

#[test]
fn test_nef() {
    let format = FileFormat::from_file("fixtures/image/sample.nef").unwrap();
    assert_eq!(format, FileFormat::NikonElectronicFile);
}

#[test]
fn test_orf() {
    let format = FileFormat::from_file("fixtures/image/sample.orf").unwrap();
    assert_eq!(format, FileFormat::OlympusRawFormat);
}

#[test]
fn test_png() {
    let format = FileFormat::from_file("fixtures/image/sample.png").unwrap();
    assert_eq!(format, FileFormat::PortableNetworkGraphics);
}

#[test]
fn test_psd() {
    let format = FileFormat::from_file("fixtures/image/sample.psd").unwrap();
    assert_eq!(format, FileFormat::AdobePhotoshopDocument);
}

#[test]
fn test_raf() {
    let format = FileFormat::from_file("fixtures/image/sample.raf").unwrap();
    assert_eq!(format, FileFormat::FujifilmRaw);
}

#[test]
fn test_rw2() {
    let format = FileFormat::from_file("fixtures/image/sample.rw2").unwrap();
    assert_eq!(format, FileFormat::PanasonicRaw);
}

#[test]
fn test_tiff() {
    let format = FileFormat::from_file("fixtures/image/sample.tiff").unwrap();
    assert_eq!(format, FileFormat::TagImageFileFormat);
}

#[test]
fn test_webp() {
    let format = FileFormat::from_file("fixtures/image/sample.webp").unwrap();
    assert_eq!(format, FileFormat::WebP);
}

#[test]
fn test_wmf() {
    let format = FileFormat::from_file("fixtures/image/sample.wmf").unwrap();
    assert_eq!(format, FileFormat::WindowsMetafile);
}

#[test]
fn test_xcf() {
    let format = FileFormat::from_file("fixtures/image/sample.xcf").unwrap();
    assert_eq!(format, FileFormat::ExperimentalComputingFacility);
}
