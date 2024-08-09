use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_matroska_subtitles() {
    let fmt = FileFormat::from_media_type("application/x-matroska");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MatroskaSubtitles)), "{:?} does not contain {}", fmt, FileFormat::MatroskaSubtitles);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg4_part14_subtitles() {
    let fmt = FileFormat::from_media_type("application/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg4Part14Subtitles)), "{:?} does not contain {}", fmt, FileFormat::Mpeg4Part14Subtitles);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_subrip_text() {
    let fmt = FileFormat::from_media_type("application/x-subrip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SubripText)), "{:?} does not contain {}", fmt, FileFormat::SubripText);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_timed_text_markup_language(){
    let fmt = FileFormat::from_media_type("application/ttml+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::TimedTextMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::TimedTextMarkupLanguage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_universal_subtitle_format(){
    let fmt = FileFormat::from_media_type("application/x-usf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniversalSubtitleFormat)), "{:?} does not contain {}", fmt, FileFormat::UniversalSubtitleFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_web_video_text_tracks() {
    let fmt = FileFormat::from_media_type("text/vtt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebVideoTextTracks)), "{:?} does not contain {}", fmt, FileFormat::WebVideoTextTracks);
}

