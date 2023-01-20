use file_format::FileFormat;

#[test]
fn test_adobe_flash_player_protected_video() {
    let format = FileFormat::from_file("fixtures/video/sample.f4p").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerProtectedVideo);
}

#[test]
fn test_adobe_flash_player_video() {
    let format = FileFormat::from_file("fixtures/video/sample.f4v").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerVideo);
}

#[test]
fn test_apple_itunes_video() {
    let format = FileFormat::from_file("fixtures/video/sample.m4v").unwrap();
    assert_eq!(format, FileFormat::AppleItunesVideo);
}

#[test]
fn test_apple_quicktime() {
    let format = FileFormat::from_file("fixtures/video/sample.mov").unwrap();
    assert_eq!(format, FileFormat::AppleQuicktime);
}

#[test]
fn test_audio_video_interleave() {
    let format = FileFormat::from_file("fixtures/video/sample.avi").unwrap();
    assert_eq!(format, FileFormat::AudioVideoInterleave);
}

#[test]
fn test_flash_video() {
    let format = FileFormat::from_file("fixtures/video/sample.flv").unwrap();
    assert_eq!(format, FileFormat::FlashVideo);
}

#[test]
fn test_matroska_video() {
    let format = FileFormat::from_file("fixtures/video/sample.mkv").unwrap();
    assert_eq!(format, FileFormat::MatroskaVideo);
}

#[test]
fn test_mpeg1_video() {
    let format = FileFormat::from_file("fixtures/video/sample.mpg").unwrap();
    assert_eq!(format, FileFormat::Mpeg1Video);
}

#[test]
fn test_mpeg2_transport_stream() {
    let format = FileFormat::from_file("fixtures/video/sample.mts").unwrap();
    assert_eq!(format, FileFormat::Mpeg2TransportStream);
}

#[test]
fn test_mpeg4_part14_video() {
    let format = FileFormat::from_file("fixtures/video/sample.mp4").unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Video);
}

#[test]
fn test_ogg_media() {
    let format = FileFormat::from_file("fixtures/video/sample.ogm").unwrap();
    assert_eq!(format, FileFormat::OggMedia);
}

#[test]
fn test_ogg_theora() {
    let format = FileFormat::from_file("fixtures/video/sample.ogv").unwrap();
    assert_eq!(format, FileFormat::OggTheora);
}

#[test]
fn test_sony_movie() {
    let format = FileFormat::from_file("fixtures/video/sample.mqv").unwrap();
    assert_eq!(format, FileFormat::SonyMovie);
}

#[test]
fn test_third_generation_partnership_project() {
    let format = FileFormat::from_file("fixtures/video/sample.3gp").unwrap();
    assert_eq!(format, FileFormat::ThirdGenerationPartnershipProject);
}

#[test]
fn test_third_generation_partnership_project2() {
    let format = FileFormat::from_file("fixtures/video/sample.3g2").unwrap();
    assert_eq!(format, FileFormat::ThirdGenerationPartnershipProject2);
}

#[cfg(feature = "reader-mkv")]
#[test]
fn test_webm() {
    let format = FileFormat::from_file("fixtures/video/sample.webm").unwrap();
    assert_eq!(format, FileFormat::Webm);
}

#[test]
fn test_windows_media_video() {
    let format = FileFormat::from_file("fixtures/video/sample.wmv").unwrap();
    assert_eq!(format, FileFormat::WindowsMediaVideo);
}
