use file_format::FileFormat;

#[cfg(feature = "reader-zip")]
#[test]
fn test_design_web_format_xps() {
    let format = FileFormat::from_file("fixtures/model/sample.dwfx").unwrap();
    assert_eq!(format, FileFormat::DesignWebFormatXps);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_extensible_3d_graphics() {
    let format = FileFormat::from_file("fixtures/model/sample.x3d").unwrap();
    assert_eq!(format, FileFormat::Extensible3DGraphics);
}

#[test]
fn test_gl_transmission_format_binary() {
    let format = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(format, FileFormat::GlTransmissionFormatBinary);
}

#[test]
fn test_google_draco() {
    let format = FileFormat::from_file("fixtures/model/sample.drc").unwrap();
    assert_eq!(format, FileFormat::GoogleDraco);
}

#[test]
fn test_stereolithography() {
    let format = FileFormat::from_file("fixtures/model/sample.stl").unwrap();
    assert_eq!(format, FileFormat::Stereolithography);
}
