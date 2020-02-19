use file_format::{FileFormat, Kind};

#[test]
fn test_txt() {
    let format = FileFormat::from_file("fixtures/text/sample.txt").unwrap();
    assert_eq!(format.kind(), Kind::Text);
    assert_eq!(format.media_type(), "text/plain");
    assert_eq!(format.preferred_extension(), "txt");
}
