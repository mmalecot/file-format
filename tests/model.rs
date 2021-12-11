use file_format::FileFormat;

#[test]
fn test_glb() {
    let format = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(format, FileFormat::GlTransmissionFormatBinary);
}
