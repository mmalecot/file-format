use file_format::FileFormat;

#[test]
fn test_3g2() {
    let format = FileFormat::from_file("fixtures/video/sample.3g2").unwrap();
    assert_eq!(format, FileFormat::ThirdGenerationPartnershipProject2);
}

#[test]
fn test_3gp() {
    let format = FileFormat::from_file("fixtures/video/sample.3gp").unwrap();
    assert_eq!(format, FileFormat::ThirdGenerationPartnershipProject);
}

#[test]
fn test_avi() {
    let format = FileFormat::from_file("fixtures/video/sample.avi").unwrap();
    assert_eq!(format, FileFormat::AudioVideoInterleave);
}

#[test]
fn test_f4p() {
    let format = FileFormat::from_file("fixtures/video/sample.f4p").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerProtectedVideo);
}

#[test]
fn test_f4v() {
    let format = FileFormat::from_file("fixtures/video/sample.f4v").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerVideo);
}

#[test]
fn test_flv() {
    let format = FileFormat::from_file("fixtures/video/sample.flv").unwrap();
    assert_eq!(format, FileFormat::FlashVideo);
}

#[test]
fn test_m4v() {
    let format = FileFormat::from_file("fixtures/video/sample.m4v").unwrap();
    assert_eq!(format, FileFormat::AppleItunesVideo);
}

#[test]
fn test_mkv() {
    let format = FileFormat::from_file("fixtures/video/sample.mkv").unwrap();
    assert_eq!(format, FileFormat::MatroskaVideo);
}

#[test]
fn test_mov() {
    let format = FileFormat::from_file("fixtures/video/sample.mov").unwrap();
    assert_eq!(format, FileFormat::AppleQuickTime);
}

#[test]
fn test_mp4() {
    let format = FileFormat::from_file("fixtures/video/sample.mp4").unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Video);
}

#[test]
fn test_mpg() {
    let format = FileFormat::from_file("fixtures/video/sample.mpg").unwrap();
    assert_eq!(format, FileFormat::Mpeg1Video);
}

#[test]
fn test_ogm() {
    let format = FileFormat::from_file("fixtures/video/sample.ogm").unwrap();
    assert_eq!(format, FileFormat::OggMedia);
}

#[test]
fn test_ogv() {
    let format = FileFormat::from_file("fixtures/video/sample.ogv").unwrap();
    assert_eq!(format, FileFormat::OggTheora);
}

#[test]
fn test_wmv() {
    let format = FileFormat::from_file("fixtures/video/sample.wmv").unwrap();
    assert_eq!(format, FileFormat::WindowsMediaVideo);
}
