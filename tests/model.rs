use file_format::FileFormat;

#[test]
fn test_3mf() {
    let format = FileFormat::from_file("fixtures/model/sample.3mf").unwrap();
    assert_eq!(format, FileFormat::ThreeDimensionalManufacturingFormat);
}

#[test]
fn test_glb() {
    let format = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(format, FileFormat::GlTransmissionFormatBinary);
}
