use file_format::FileFormat;

#[test]
fn test_actions_media_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.amv").unwrap();
    assert_eq!(fmt, FileFormat::ActionsMediaVideo);
}

#[test]
fn test_adobe_flash_player_protected_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.f4p").unwrap();
    assert_eq!(fmt, FileFormat::AdobeFlashPlayerProtectedVideo);
}

#[test]
fn test_adobe_flash_player_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.f4v").unwrap();
    assert_eq!(fmt, FileFormat::AdobeFlashPlayerVideo);
}

#[test]
fn test_apple_itunes_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.m4v").unwrap();
    assert_eq!(fmt, FileFormat::AppleItunesVideo);
}

#[test]
fn test_apple_quicktime() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mov").unwrap();
    assert_eq!(fmt, FileFormat::AppleQuicktime);
}

#[test]
fn test_audio_video_interleave() {
    let fmt = FileFormat::from_file("fixtures/video/sample.avi").unwrap();
    assert_eq!(fmt, FileFormat::AudioVideoInterleave);
}

#[test]
fn test_autodesk_animator() {
    let fmt = FileFormat::from_file("fixtures/video/sample.fli").unwrap();
    assert_eq!(fmt, FileFormat::AutodeskAnimator);
}

#[test]
fn test_autodesk_animator_pro() {
    let fmt = FileFormat::from_file("fixtures/video/sample.flc").unwrap();
    assert_eq!(fmt, FileFormat::AutodeskAnimatorPro);
}

#[test]
fn test_bdav_mpeg2_transport_stream() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mts").unwrap();
    assert_eq!(fmt, FileFormat::BdavMpeg2TransportStream);
}

#[test]
fn test_flash_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.flv").unwrap();
    assert_eq!(fmt, FileFormat::FlashVideo);
}

#[test]
fn test_jpeg2000_part3() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mj2").unwrap();
    assert_eq!(fmt, FileFormat::Jpeg2000Part3);
}

#[test]
fn test_material_exchange_format() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mxf").unwrap();
    assert_eq!(fmt, FileFormat::MaterialExchangeFormat);
}

#[cfg(feature = "reader-ebml")]
#[test]
fn test_matroska3d_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mk3d").unwrap();
    assert_eq!(fmt, FileFormat::Matroska3dVideo);
}

#[cfg(feature = "reader-ebml")]
#[test]
fn test_matroska_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mkv").unwrap();
    assert_eq!(fmt, FileFormat::MatroskaVideo);
}

#[cfg(feature = "reader-asf")]
#[test]
fn test_microsoft_digital_video_recording() {
    let fmt = FileFormat::from_file("fixtures/video/sample.dvr-ms").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftDigitalVideoRecording);
}

#[test]
fn test_mpeg12_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mpg").unwrap();
    assert_eq!(fmt, FileFormat::Mpeg12Video);
}

#[test]
fn test_mpeg2_transport_stream() {
    let fmt = FileFormat::from_file("fixtures/video/sample.ts").unwrap();
    assert_eq!(fmt, FileFormat::Mpeg2TransportStream);
}

#[cfg(feature = "reader-mp4")]
#[test]
fn test_mpeg4_part14_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mp4").unwrap();
    assert_eq!(fmt, FileFormat::Mpeg4Part14Video);
}

#[test]
fn test_mtv() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mtv").unwrap();
    assert_eq!(fmt, FileFormat::Mtv);
}

#[test]
fn test_ogg_media() {
    let fmt = FileFormat::from_file("fixtures/video/sample.ogm").unwrap();
    assert_eq!(fmt, FileFormat::OggMedia);
}

#[test]
fn test_ogg_theora() {
    let fmt = FileFormat::from_file("fixtures/video/sample.ogv").unwrap();
    assert_eq!(fmt, FileFormat::OggTheora);
}

#[cfg(feature = "reader-rm")]
#[test]
fn test_realvideo() {
    let fmt = FileFormat::from_file("fixtures/video/sample.rv").unwrap();
    assert_eq!(fmt, FileFormat::Realvideo);
}

#[test]
fn test_silicon_graphics_movie() {
    let fmt = FileFormat::from_file("fixtures/video/sample.sgi").unwrap();
    assert_eq!(fmt, FileFormat::SiliconGraphicsMovie);
}

#[test]
fn test_sony_movie() {
    let fmt = FileFormat::from_file("fixtures/video/sample.mqv").unwrap();
    assert_eq!(fmt, FileFormat::SonyMovie);
}

#[test]
fn test_third_generation_partnership_project() {
    let fmt = FileFormat::from_file("fixtures/video/sample.3gp").unwrap();
    assert_eq!(fmt, FileFormat::ThirdGenerationPartnershipProject);
}

#[test]
fn test_third_generation_partnership_project2() {
    let fmt = FileFormat::from_file("fixtures/video/sample.3g2").unwrap();
    assert_eq!(fmt, FileFormat::ThirdGenerationPartnershipProject2);
}

#[cfg(feature = "reader-ebml")]
#[test]
fn test_webm() {
    let fmt = FileFormat::from_file("fixtures/video/sample.webm").unwrap();
    assert_eq!(fmt, FileFormat::Webm);
}

#[cfg(feature = "reader-asf")]
#[test]
fn test_windows_media_video() {
    let fmt = FileFormat::from_file("fixtures/video/sample.wmv").unwrap();
    assert_eq!(fmt, FileFormat::WindowsMediaVideo);
}

#[test]
fn test_windows_recorded_tv_show() {
    let fmt = FileFormat::from_file("fixtures/video/sample.wtv").unwrap();
    assert_eq!(fmt, FileFormat::WindowsRecordedTvShow);
}
