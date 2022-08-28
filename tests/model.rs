use file_format::FileFormat;

#[cfg(feature = "zip")]
#[test]
fn test_design_web_format() {
    let format = FileFormat::from_file("fixtures/model/sample.dwf").unwrap();
    assert_eq!(format, FileFormat::DesignWebFormat);
}

#[cfg(feature = "zip")]
#[test]
fn test_design_web_format_xps() {
    let format = FileFormat::from_file("fixtures/model/sample.dwfx").unwrap();
    assert_eq!(format, FileFormat::DesignWebFormatXps);
}

#[test]
fn test_gl_transmission_format_binary() {
    let format = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(format, FileFormat::GlTransmissionFormatBinary);
}
