use file_format::FileFormat;

#[cfg(feature = "reader-xml")]
#[test]
fn test_additive_manufacturing_format() {
    let format = FileFormat::from_file("fixtures/model/sample.amf").unwrap();
    assert_eq!(format, FileFormat::AdditiveManufacturingFormat);
}

#[test]
fn test_additive_manufacturing_format_no_xml() {
    let format = FileFormat::from_file("fixtures/model/sample-no-xml.amf").unwrap();
    assert_eq!(format, FileFormat::AdditiveManufacturingFormat);
}

#[test]
fn test_autocad_drawing() {
    let format = FileFormat::from_file("fixtures/model/sample.dwg").unwrap();
    assert_eq!(format, FileFormat::AutocadDrawing);
}

#[test]
fn test_blender() {
    let format = FileFormat::from_file("fixtures/model/sample.blend").unwrap();
    assert_eq!(format, FileFormat::Blender);
}

#[test]
fn test_design_web_format() {
    let format = FileFormat::from_file("fixtures/model/sample.dwf").unwrap();
    assert_eq!(format, FileFormat::DesignWebFormat);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_design_web_format_xps() {
    let format = FileFormat::from_file("fixtures/model/sample.dwfx").unwrap();
    assert_eq!(format, FileFormat::DesignWebFormatXps);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_digital_asset_exchange() {
    let format = FileFormat::from_file("fixtures/model/sample.dae").unwrap();
    assert_eq!(format, FileFormat::DigitalAssetExchange);
}

#[test]
fn test_digital_asset_exchange_no_xml() {
    let format = FileFormat::from_file("fixtures/model/sample-no-xml.dae").unwrap();
    assert_eq!(format, FileFormat::DigitalAssetExchange);
}

#[test]
fn test_drawing_exchange_format_ascii() {
    let format = FileFormat::from_file("fixtures/model/sample-ascii.dxf").unwrap();
    assert_eq!(format, FileFormat::DrawingExchangeFormatAscii);
}

#[test]
fn test_drawing_exchange_format_binary() {
    let format = FileFormat::from_file("fixtures/model/sample-binary.dxf").unwrap();
    assert_eq!(format, FileFormat::DrawingExchangeFormatBinary);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_extensible3d_graphics() {
    let format = FileFormat::from_file("fixtures/model/sample.x3d").unwrap();
    assert_eq!(format, FileFormat::Extensible3d);
}

#[test]
fn test_extensible3d_graphics_no_xml() {
    let format = FileFormat::from_file("fixtures/model/sample-no-xml.x3d").unwrap();
    assert_eq!(format, FileFormat::Extensible3d);
}

#[test]
fn test_filmbox() {
    let format = FileFormat::from_file("fixtures/model/sample.fbx").unwrap();
    assert_eq!(format, FileFormat::Filmbox);
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
fn test_inter_quake_export() {
    let format = FileFormat::from_file("fixtures/model/sample.iqe").unwrap();
    assert_eq!(format, FileFormat::InterQuakeExport);
}

#[test]
fn test_inter_quake_model() {
    let format = FileFormat::from_file("fixtures/model/sample.iqm").unwrap();
    assert_eq!(format, FileFormat::InterQuakeModel);
}

#[test]
fn test_magicavoxel() {
    let format = FileFormat::from_file("fixtures/model/sample.vox").unwrap();
    assert_eq!(format, FileFormat::Magicavoxel);
}

#[test]
fn test_maya_ascii() {
    let format = FileFormat::from_file("fixtures/model/sample.ma").unwrap();
    assert_eq!(format, FileFormat::MayaAscii);
}

#[test]
fn test_maya_binary() {
    let format = FileFormat::from_file("fixtures/model/sample.mb").unwrap();
    assert_eq!(format, FileFormat::MayaBinary);
}

#[test]
fn test_model3d_ascii() {
    let format = FileFormat::from_file("fixtures/model/sample.a3d").unwrap();
    assert_eq!(format, FileFormat::Model3dAscii);
}

#[test]
fn test_model3d_binary() {
    let format = FileFormat::from_file("fixtures/model/sample.m3d").unwrap();
    assert_eq!(format, FileFormat::Model3dBinary);
}

#[test]
fn test_polygon_ascii() {
    let format = FileFormat::from_file("fixtures/model/sample-ascii.ply").unwrap();
    assert_eq!(format, FileFormat::PolygonAscii);
}

#[test]
fn test_polygon_binary() {
    let format = FileFormat::from_file("fixtures/model/sample-binary.ply").unwrap();
    assert_eq!(format, FileFormat::PolygonBinary);
}

#[test]
fn test_sketchup() {
    let format = FileFormat::from_file("fixtures/model/sample.skp").unwrap();
    assert_eq!(format, FileFormat::Sketchup);
}

#[test]
fn test_stereolithography_ascii() {
    let format = FileFormat::from_file("fixtures/model/sample-ascii.stl").unwrap();
    assert_eq!(format, FileFormat::StereolithographyAscii);
}

#[test]
fn test_stereolithography_binary() {
    let format = FileFormat::from_file("fixtures/model/sample-binary.stl").unwrap();
    assert_eq!(format, FileFormat::StereolithographyBinary);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_three_dimensional_manufacturing_format() {
    let format = FileFormat::from_file("fixtures/model/sample.3mf").unwrap();
    assert_eq!(format, FileFormat::ThreeDimensionalManufacturingFormat);
}

#[test]
fn test_three_dimensional_studio() {
    let format = FileFormat::from_file("fixtures/model/sample.3ds").unwrap();
    assert_eq!(format, FileFormat::ThreeDimensionalStudio);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_three_dimensional_studio_max() {
    let format = FileFormat::from_file("fixtures/model/sample.max").unwrap();
    assert_eq!(format, FileFormat::ThreeDimensionalStudioMax);
}

#[test]
fn test_universal3d() {
    let format = FileFormat::from_file("fixtures/model/sample.u3d").unwrap();
    assert_eq!(format, FileFormat::Universal3d);
}
