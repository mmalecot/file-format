use file_format::{FileFormat, Kind};

#[test]
fn test_bin() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/octet-stream");
    assert_eq!(format.preferred_extension(), Some("bin"));
}
