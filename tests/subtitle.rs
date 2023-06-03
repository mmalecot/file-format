use file_format::FileFormat;

#[test]
fn test_subrip_text() {
    let fmt = FileFormat::from_file("fixtures/subtitle/sample.srt").unwrap();
    assert_eq!(fmt, FileFormat::SubripText);
}

#[test]
fn test_timed_text_markup_language_1() {
    let fmt = FileFormat::from_file("fixtures/subtitle/sample1.ttml").unwrap();
    assert_eq!(fmt, FileFormat::TimedTextMarkupLanguage);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_timed_text_markup_language_2() {
    let fmt = FileFormat::from_file("fixtures/subtitle/sample2.ttml").unwrap();
    assert_eq!(fmt, FileFormat::TimedTextMarkupLanguage);
}

#[test]
fn test_universal_subtitle_format_1() {
    let fmt = FileFormat::from_file("fixtures/subtitle/sample1.usf").unwrap();
    assert_eq!(fmt, FileFormat::UniversalSubtitleFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_universal_subtitle_format_2() {
    let fmt = FileFormat::from_file("fixtures/subtitle/sample2.usf").unwrap();
    assert_eq!(fmt, FileFormat::UniversalSubtitleFormat);
}

#[test]
fn test_web_video_text_tracks() {
    let fmt = FileFormat::from_file("fixtures/subtitle/sample.vtt").unwrap();
    assert_eq!(fmt, FileFormat::WebVideoTextTracks);
}
