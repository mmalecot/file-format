use file_format::FileFormat;

#[test]
fn test_3g2() {
    let format = FileFormat::from_file("fixtures/video/sample.3g2").unwrap();
    assert_eq!(format, FileFormat::ThirdGpp2);
    assert_eq!(format.media_type(), "video/3gpp2");
    assert_eq!(format.preferred_extension(), "3g2");
}

#[test]
fn test_3gp() {
    let format = FileFormat::from_file("fixtures/video/sample.3gp").unwrap();
    assert_eq!(format, FileFormat::ThirdGpp);
    assert_eq!(format.media_type(), "video/3gpp");
    assert_eq!(format.preferred_extension(), "3gp");
}

#[test]
fn test_avi() {
    let format = FileFormat::from_file("fixtures/video/sample.avi").unwrap();
    assert_eq!(format, FileFormat::Avi);
    assert_eq!(format.media_type(), "video/avi");
    assert_eq!(format.preferred_extension(), "avi");
}

#[test]
fn test_flv() {
    let format = FileFormat::from_file("fixtures/video/sample.flv").unwrap();
    assert_eq!(format, FileFormat::Flv);
    assert_eq!(format.media_type(), "video/x-flv");
    assert_eq!(format.preferred_extension(), "flv");
}

#[test]
fn test_m2ts() {
    let format = FileFormat::from_file("fixtures/video/sample.m2ts").unwrap();
    assert_eq!(format, FileFormat::M2ts);
    assert_eq!(format.media_type(), "video/mp2t");
    assert_eq!(format.preferred_extension(), "m2ts");
}

#[test]
fn test_m4v() {
    let format = FileFormat::from_file("fixtures/video/sample.m4v").unwrap();
    assert_eq!(format, FileFormat::M4v);
    assert_eq!(format.media_type(), "video/x-m4v");
    assert_eq!(format.preferred_extension(), "m4v");
}

#[test]
fn test_mkv() {
    let format = FileFormat::from_file("fixtures/video/sample.mkv").unwrap();
    assert_eq!(format, FileFormat::Matroska);
    assert_eq!(format.media_type(), "video/x-matroska");
    assert_eq!(format.preferred_extension(), "mkv");
}

#[test]
fn test_mov() {
    let format = FileFormat::from_file("fixtures/video/sample.mov").unwrap();
    assert_eq!(format, FileFormat::QuickTime);
    assert_eq!(format.media_type(), "video/quicktime");
    assert_eq!(format.preferred_extension(), "mov");
}

#[test]
fn test_mp4() {
    let format = FileFormat::from_file("fixtures/video/sample.mp4").unwrap();
    assert_eq!(format, FileFormat::Mp4);
    assert_eq!(format.media_type(), "video/mp4");
    assert_eq!(format.preferred_extension(), "mp4");
}

#[test]
fn test_mpeg() {
    let format = FileFormat::from_file("fixtures/video/sample.mpg").unwrap();
    assert_eq!(format, FileFormat::Mpeg);
    assert_eq!(format.media_type(), "video/mpeg");
    assert_eq!(format.preferred_extension(), "mpg");
}

#[test]
fn test_ogv() {
    let format = FileFormat::from_file("fixtures/video/sample.ogv").unwrap();
    assert_eq!(format, FileFormat::OggVideo);
    assert_eq!(format.media_type(), "video/ogg");
    assert_eq!(format.preferred_extension(), "ogv");
}

#[test]
fn test_webm() {
    let format = FileFormat::from_file("fixtures/video/sample.webm").unwrap();
    assert_eq!(format, FileFormat::WebM);
    assert_eq!(format.media_type(), "video/webm");
    assert_eq!(format.preferred_extension(), "webm");
}

#[test]
fn test_wmv() {
    let format = FileFormat::from_file("fixtures/video/sample.wmv").unwrap();
    assert_eq!(format, FileFormat::Wmv);
    assert_eq!(format.media_type(), "video/x-ms-asf");
    assert_eq!(format.preferred_extension(), "wmv");
}
