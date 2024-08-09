use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_actions_media_video() {
    let fmt = FileFormat::from_media_type("video/x-amv");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ActionsMediaVideo)), "{:?} does not contain {}", fmt, FileFormat::ActionsMediaVideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apple_itunes_video() {
    let fmt = FileFormat::from_media_type("video/x-m4v");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleItunesVideo)), "{:?} does not contain {}", fmt, FileFormat::AppleItunesVideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apple_quicktime() {
    let fmt = FileFormat::from_media_type("video/quicktime");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleQuicktime)), "{:?} does not contain {}", fmt, FileFormat::AppleQuicktime);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_audio_video_interleave() {
    let fmt = FileFormat::from_media_type("video/avi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AudioVideoInterleave)), "{:?} does not contain {}", fmt, FileFormat::AudioVideoInterleave);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk_animator() {
    let fmt = FileFormat::from_media_type("video/x-fli");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskAnimator)), "{:?} does not contain {}", fmt, FileFormat::AutodeskAnimator);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk_animator_pro() {
    let fmt = FileFormat::from_media_type("video/x-flc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskAnimatorPro)), "{:?} does not contain {}", fmt, FileFormat::AutodeskAnimatorPro);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_bdav_mpeg2_transport_stream() {
    let fmt = FileFormat::from_media_type("video/mp2t");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BdavMpeg2TransportStream)), "{:?} does not contain {}", fmt, FileFormat::BdavMpeg2TransportStream);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flash_mp4_protected_video() {
    let fmt = FileFormat::from_media_type("video/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashMp4ProtectedVideo)), "{:?} does not contain {}", fmt, FileFormat::FlashMp4ProtectedVideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flash_mp4_video() {
    let fmt = FileFormat::from_media_type("video/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashMp4Video)), "{:?} does not contain {}", fmt, FileFormat::FlashMp4Video);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flash_video() {
    let fmt = FileFormat::from_media_type("video/x-flv");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashVideo)), "{:?} does not contain {}", fmt, FileFormat::FlashVideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg2000_part3() {
    let fmt = FileFormat::from_media_type("video/mj2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Jpeg2000Part3)), "{:?} does not contain {}", fmt, FileFormat::Jpeg2000Part3);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_material_exchange_format() {
    let fmt = FileFormat::from_media_type("application/mxf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MaterialExchangeFormat)), "{:?} does not contain {}", fmt, FileFormat::MaterialExchangeFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_matroska3d_video() {
    let fmt = FileFormat::from_media_type("video/x-matroska");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Matroska3dVideo)), "{:?} does not contain {}", fmt, FileFormat::Matroska3dVideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_matroska_video() {
    let fmt = FileFormat::from_media_type("video/x-matroska");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MatroskaVideo)), "{:?} does not contain {}", fmt, FileFormat::MatroskaVideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_digital_video_recording() {
    let fmt = FileFormat::from_media_type("video/x-ms-asf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftDigitalVideoRecording)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftDigitalVideoRecording);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg12_video() {
    let fmt = FileFormat::from_media_type("video/mpeg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg12Video)), "{:?} does not contain {}", fmt, FileFormat::Mpeg12Video);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg2_transport_stream() {
    let fmt = FileFormat::from_media_type("video/mp2t");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg2TransportStream)), "{:?} does not contain {}", fmt, FileFormat::Mpeg2TransportStream);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg4_part14_video() {
    let fmt = FileFormat::from_media_type("video/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg4Part14Video)), "{:?} does not contain {}", fmt, FileFormat::Mpeg4Part14Video);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mtv(){
    let fmt = FileFormat::from_media_type("video/x-mtv");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mtv)), "{:?} does not contain {}", fmt, FileFormat::Mtv);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ogg_media() {
    let fmt = FileFormat::from_media_type("video/ogg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggMedia)), "{:?} does not contain {}", fmt, FileFormat::OggMedia);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ogg_theora() {
    let fmt = FileFormat::from_media_type("video/ogg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggTheora)), "{:?} does not contain {}", fmt, FileFormat::OggTheora);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_realvideo(){
    let fmt = FileFormat::from_media_type("video/x-pn-realvideo");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Realvideo)), "{:?} does not contain {}", fmt, FileFormat::Realvideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_silicon_graphics_movie() {
    let fmt = FileFormat::from_media_type("video/x-sgi-movie");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SiliconGraphicsMovie)), "{:?} does not contain {}", fmt, FileFormat::SiliconGraphicsMovie);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sony_movie() {
    let fmt = FileFormat::from_media_type("video/quicktime");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SonyMovie)), "{:?} does not contain {}", fmt, FileFormat::SonyMovie);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_third_generation_partnership_project() {
    let fmt = FileFormat::from_media_type("video/3gpp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThirdGenerationPartnershipProject)), "{:?} does not contain {}", fmt, FileFormat::ThirdGenerationPartnershipProject);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_third_generation_partnership_project2() {
    let fmt = FileFormat::from_media_type("video/3gpp2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThirdGenerationPartnershipProject2)), "{:?} does not contain {}", fmt, FileFormat::ThirdGenerationPartnershipProject2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_webm(){
    let fmt = FileFormat::from_media_type("video/webm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Webm)), "{:?} does not contain {}", fmt, FileFormat::Webm);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_media_video() {
    let fmt = FileFormat::from_media_type("video/x-ms-wmv");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsMediaVideo)), "{:?} does not contain {}", fmt, FileFormat::WindowsMediaVideo);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_recorded_tv_show() {
    let fmt = FileFormat::from_media_type("video/x-wtv");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsRecordedTvShow)), "{:?} does not contain {}", fmt, FileFormat::WindowsRecordedTvShow);
}

