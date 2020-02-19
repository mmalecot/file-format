use file_format::{FileFormat, Kind};

#[test]
fn test_aac() {
    let format = FileFormat::from_file("fixtures/audio/sample.aac").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/aac");
    assert_eq!(format.preferred_extension(), "aac");
}

#[test]
fn test_aif() {
    let format = FileFormat::from_file("fixtures/audio/sample.aif").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/aiff");
    assert_eq!(format.preferred_extension(), "aif");
}

#[test]
fn test_au() {
    let format = FileFormat::from_file("fixtures/audio/sample.au").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/basic");
    assert_eq!(format.preferred_extension(), "au");
}

#[test]
fn test_flac() {
    let format = FileFormat::from_file("fixtures/audio/sample.flac").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/x-flac");
    assert_eq!(format.preferred_extension(), "flac");
}

#[test]
fn test_m4a() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4a").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/x-m4a");
    assert_eq!(format.preferred_extension(), "m4a");
}

#[test]
fn test_mp3() {
    let format = FileFormat::from_file("fixtures/audio/sample.mp3").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/mpeg");
    assert_eq!(format.preferred_extension(), "mp3");
}

#[test]
fn test_oga() {
    let format = FileFormat::from_file("fixtures/audio/sample.oga").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/ogg");
    assert_eq!(format.preferred_extension(), "ogg");
}

#[test]
fn test_ogg() {
    let format = FileFormat::from_file("fixtures/audio/sample.ogg").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/ogg");
    assert_eq!(format.preferred_extension(), "ogg");
}

#[test]
fn test_spx() {
    let format = FileFormat::from_file("fixtures/audio/sample.spx").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/ogg");
    assert_eq!(format.preferred_extension(), "ogg");
}

#[test]
fn test_wav() {
    let format = FileFormat::from_file("fixtures/audio/sample.wav").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/vnd.wave");
    assert_eq!(format.preferred_extension(), "wav");
}

#[test]
fn test_wc() {
    let format = FileFormat::from_file("fixtures/audio/sample.wv").unwrap();
    assert_eq!(format.kind(), Kind::Audio);
    assert_eq!(format.media_type(), "audio/wavpack");
    assert_eq!(format.preferred_extension(), "wv");
}
