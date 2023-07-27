use file_format::FileFormat;

#[test]
fn test_additive_manufacturing_format_1() {
    let fmt = FileFormat::from_file("fixtures/model/sample1.amf").unwrap();
    assert_eq!(fmt, FileFormat::AdditiveManufacturingFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_additive_manufacturing_format_2() {
    let fmt = FileFormat::from_file("fixtures/model/sample2.amf").unwrap();
    assert_eq!(fmt, FileFormat::AdditiveManufacturingFormat);
}

#[test]
fn test_autocad_drawing() {
    let fmt = FileFormat::from_file("fixtures/model/sample.dwg").unwrap();
    assert_eq!(fmt, FileFormat::AutocadDrawing);
}

#[test]
fn test_blender() {
    let fmt = FileFormat::from_file("fixtures/model/sample.blend").unwrap();
    assert_eq!(fmt, FileFormat::Blender);
}

#[test]
fn test_cinema4d() {
    let fmt = FileFormat::from_file("fixtures/model/sample.c4d").unwrap();
    assert_eq!(fmt, FileFormat::Cinema4d);
}

#[test]
fn test_design_web_format() {
    let fmt = FileFormat::from_file("fixtures/model/sample.dwf").unwrap();
    assert_eq!(fmt, FileFormat::DesignWebFormat);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_design_web_format_xps() {
    let fmt = FileFormat::from_file("fixtures/model/sample.dwfx").unwrap();
    assert_eq!(fmt, FileFormat::DesignWebFormatXps);
}

#[test]
fn test_digital_asset_exchange_1() {
    let fmt = FileFormat::from_file("fixtures/model/sample1.dae").unwrap();
    assert_eq!(fmt, FileFormat::DigitalAssetExchange);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_digital_asset_exchange_2() {
    let fmt = FileFormat::from_file("fixtures/model/sample2.dae").unwrap();
    assert_eq!(fmt, FileFormat::DigitalAssetExchange);
}

#[test]
fn test_drawing_exchange_format_ascii() {
    let fmt = FileFormat::from_file("fixtures/model/sample1.dxf").unwrap();
    assert_eq!(fmt, FileFormat::DrawingExchangeFormatAscii);
}

#[test]
fn test_drawing_exchange_format_binary() {
    let fmt = FileFormat::from_file("fixtures/model/sample2.dxf").unwrap();
    assert_eq!(fmt, FileFormat::DrawingExchangeFormatBinary);
}

#[test]
fn test_extensible3d_graphics_1() {
    let fmt = FileFormat::from_file("fixtures/model/sample1.x3d").unwrap();
    assert_eq!(fmt, FileFormat::Extensible3d);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_extensible3d_graphics_2() {
    let fmt = FileFormat::from_file("fixtures/model/sample2.x3d").unwrap();
    assert_eq!(fmt, FileFormat::Extensible3d);
}

#[test]
fn test_filmbox() {
    let fmt = FileFormat::from_file("fixtures/model/sample.fbx").unwrap();
    assert_eq!(fmt, FileFormat::Filmbox);
}

#[test]
fn test_gl_transmission_format_binary() {
    let fmt = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(fmt, FileFormat::GlTransmissionFormatBinary);
}

#[test]
fn test_google_draco() {
    let fmt = FileFormat::from_file("fixtures/model/sample.drc").unwrap();
    assert_eq!(fmt, FileFormat::GoogleDraco);
}

#[test]
fn test_inter_quake_export() {
    let fmt = FileFormat::from_file("fixtures/model/sample.iqe").unwrap();
    assert_eq!(fmt, FileFormat::InterQuakeExport);
}

#[test]
fn test_inter_quake_model() {
    let fmt = FileFormat::from_file("fixtures/model/sample.iqm").unwrap();
    assert_eq!(fmt, FileFormat::InterQuakeModel);
}

#[test]
fn test_magicavoxel() {
    let fmt = FileFormat::from_file("fixtures/model/sample.vox").unwrap();
    assert_eq!(fmt, FileFormat::Magicavoxel);
}

#[test]
fn test_maya_ascii() {
    let fmt = FileFormat::from_file("fixtures/model/sample.ma").unwrap();
    assert_eq!(fmt, FileFormat::MayaAscii);
}

#[test]
fn test_maya_binary() {
    let fmt = FileFormat::from_file("fixtures/model/sample.mb").unwrap();
    assert_eq!(fmt, FileFormat::MayaBinary);
}

#[test]
fn test_model3d_ascii() {
    let fmt = FileFormat::from_file("fixtures/model/sample.a3d").unwrap();
    assert_eq!(fmt, FileFormat::Model3dAscii);
}

#[test]
fn test_model3d_binary() {
    let fmt = FileFormat::from_file("fixtures/model/sample.m3d").unwrap();
    assert_eq!(fmt, FileFormat::Model3dBinary);
}

#[test]
fn test_opennurbs() {
    let fmt = FileFormat::from_file("fixtures/model/sample.3dm").unwrap();
    assert_eq!(fmt, FileFormat::Opennurbs);
}

#[test]
fn test_polygon_ascii() {
    let fmt = FileFormat::from_file("fixtures/model/sample1.ply").unwrap();
    assert_eq!(fmt, FileFormat::PolygonAscii);
}

#[test]
fn test_polygon_binary() {
    let fmt = FileFormat::from_file("fixtures/model/sample2.ply").unwrap();
    assert_eq!(fmt, FileFormat::PolygonBinary);
}

#[test]
fn test_sketchup() {
    let fmt = FileFormat::from_file("fixtures/model/sample.skp").unwrap();
    assert_eq!(fmt, FileFormat::Sketchup);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_spaceclaim_document() {
    let fmt = FileFormat::from_file("fixtures/model/sample.scdoc").unwrap();
    assert_eq!(fmt, FileFormat::SpaceclaimDocument);
}

#[test]
fn test_standard_for_the_exchange_of_product_model_data() {
    let fmt = FileFormat::from_file("fixtures/model/sample.step").unwrap();
    assert_eq!(fmt, FileFormat::StandardForTheExchangeOfProductModelData);
}

#[test]
fn test_stereolithography_ascii() {
    let fmt = FileFormat::from_file("fixtures/model/sample1.stl").unwrap();
    assert_eq!(fmt, FileFormat::StereolithographyAscii);
}

#[test]
fn test_stereolithography_binary() {
    let fmt = FileFormat::from_file("fixtures/model/sample2.stl").unwrap();
    assert_eq!(fmt, FileFormat::StereolithographyBinary);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_three_dimensional_manufacturing_format() {
    let fmt = FileFormat::from_file("fixtures/model/sample.3mf").unwrap();
    assert_eq!(fmt, FileFormat::ThreeDimensionalManufacturingFormat);
}

#[test]
fn test_three_dimensional_studio() {
    let fmt = FileFormat::from_file("fixtures/model/sample.3ds").unwrap();
    assert_eq!(fmt, FileFormat::ThreeDimensionalStudio);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_three_dimensional_studio_max() {
    let fmt = FileFormat::from_file("fixtures/model/sample.max").unwrap();
    assert_eq!(fmt, FileFormat::ThreeDimensionalStudioMax);
}

#[test]
fn test_universal3d() {
    let fmt = FileFormat::from_file("fixtures/model/sample.u3d").unwrap();
    assert_eq!(fmt, FileFormat::Universal3d);
}

#[test]
fn test_universal_scene_description_ascii() {
    let fmt = FileFormat::from_file("fixtures/model/sample.usda").unwrap();
    assert_eq!(fmt, FileFormat::UniversalSceneDescriptionAscii);
}

#[test]
fn test_universal_scene_description_binary() {
    let fmt = FileFormat::from_file("fixtures/model/sample.usdc").unwrap();
    assert_eq!(fmt, FileFormat::UniversalSceneDescriptionBinary);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_universal_scene_description_zipped() {
    let fmt = FileFormat::from_file("fixtures/model/sample.usdz").unwrap();
    assert_eq!(fmt, FileFormat::UniversalSceneDescriptionZipped);
}

#[test]
fn test_virtual_reality_modeling_language() {
    let fmt = FileFormat::from_file("fixtures/model/sample.wrl").unwrap();
    assert_eq!(fmt, FileFormat::VirtualRealityModelingLanguage);
}
